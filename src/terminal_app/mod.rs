mod raw_mode_guard;
mod renderer;

use std::time::Duration;
use std::{io::stdout, time::Instant};
use tokio::time::timeout;

use crate::*;
use async_trait::async_trait;
use crossterm::event::{
    Event as CrosstermEvent, EventStream, KeyCode, KeyEvent, KeyModifiers, MouseEvent,
    MouseEventKind,
};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use raw_mode_guard::RawModeGuard;
use renderer::*;

use self::{fullscreen_renderer::FullScreenRenderer, inline_renderer::InlineRenderer};

pub use sync_terminal_app::*;
pub mod sync_terminal_app;

pub enum Event<M> {
    Key(KeyEvent),
    Message(M),
}

/// A trait representing an asynchronous terminal application.
pub trait AsyncTerminalApp {
    /// The type of messages that the application will handle.
    type Message: Send + 'static;

    /// Renders the current application.
    ///
    /// # Returns
    /// An implementation of the `View` trait that represents the current state of the application.
    fn render(&self) -> impl View;

    /// Update the application's state based on the given event.
    ///
    /// # Parameters
    /// - `event`: The event that occurred, which could be a key press or a custom message.
    /// - `sender`: A sender that can be used to send messages back to the application.
    ///
    /// # Returns
    /// `true` if the application should continue running, `false` otherwise.
    fn update(
        &mut self,
        event: Event<Self::Message>,
        sender: &mpsc::UnboundedSender<Self::Message>,
    ) -> bool;

    /// Initialize the application.
    ///
    /// This method is called when the terminal application is first initialized.
    /// It can be used to set up any necessary state or start background tasks.
    ///
    /// # Parameters
    /// - `sender`: A sender that can be used to send messages back to the application.
    fn init(&mut self, sender: &mpsc::UnboundedSender<Self::Message>) {
        let _ = sender;
    }

    /// Handle the application's exit.
    ///
    /// This method is called when the application is about to exit.
    /// It can be used to perform any necessary cleanup or to return a final view to be rendered.
    ///
    /// # Returns
    /// An optional implementation of the `View` trait that represents the final state of the application.
    /// If `None` is returned, no final view will be rendered.
    fn handle_exit(&self) -> Option<impl View> {
        None as Option<EmptyView>
    }
}

fn handle_event(tx: mpsc::UnboundedSender<CrosstermEvent>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut reader = EventStream::new();
        loop {
            let crossterm_event = reader.next().fuse();
            tokio::select! {
                Some(Ok(event)) = crossterm_event => {
                  if tx.send(event).is_err() {
                      break;
                  }
                }
                else => {}
            }
        }
    })
}

fn create_renderer(use_full_screen: bool) -> SomeRenderer<std::io::Stdout> {
    if use_full_screen {
        SomeRenderer::FullScreen(FullScreenRenderer::new(stdout()))
    } else {
        SomeRenderer::Inline(InlineRenderer::new(stdout()))
    }
}

#[async_trait]
pub trait AsyncTerminalAppExt: AsyncTerminalApp + Sized {
    async fn run(&mut self, use_full_screen: bool) {
        let (message_sender, mut message_receiver) = mpsc::unbounded_channel::<Self::Message>();
        let (terminal_event_sender, mut terminal_event_receiver) =
            mpsc::unbounded_channel::<CrosstermEvent>();

        let mut renderer = create_renderer(use_full_screen);
        let _guard = RawModeGuard::new(use_full_screen);
        let terminal_event_task = handle_event(terminal_event_sender);

        // Allow the application to initialize itself
        self.init(&message_sender);

        // Initial render
        renderer.render(&self.render());

        let collect_duration = Duration::from_millis(5);

        loop {
            let mut events = Vec::new();
            let mut messages = Vec::new();

            // Collect events and messages for a short duration
            let start = Instant::now();
            while start.elapsed() < collect_duration {
                tokio::select! {
                    Ok(Some(event)) = timeout(collect_duration, terminal_event_receiver.recv()) => {
                        events.push(event);
                    },
                    Ok(Some(msg)) = timeout(collect_duration, message_receiver.recv()) => {
                        messages.push(msg);
                    },
                    else => break,
                }
            }

            let should_render = !events.is_empty() || !messages.is_empty();

            // Process collected events
            let mut should_continue = true;
            for event in events {
                if !handle_terminal_event(self, event, &message_sender, &mut renderer) {
                    should_continue = false;
                    break;
                }
            }

            // Process collected messages
            for msg in messages {
                if !self.update(Event::Message(msg), &message_sender) {
                    should_continue = false;
                    break;
                }
            }

            // Break out of the loop if necessary
            if !should_continue {
                break;
            }

            // Render after processing the batch
            if should_render {
                renderer.render(&self.render());
            }
        }

        if let Some(view) = self.handle_exit() {
            renderer.render(&view);
        }
        renderer.move_cursor_to_bottom_of_current_view();
        terminal_event_task.abort();
        terminal_event_task.await.unwrap_err();
    }
}

impl<T: AsyncTerminalApp> AsyncTerminalAppExt for T {}

#[inline]
fn handle_terminal_event<App: AsyncTerminalApp>(
    app: &mut App,
    event: CrosstermEvent,
    message_sender: &mpsc::UnboundedSender<App::Message>,
    renderer: &mut SomeRenderer<std::io::Stdout>,
) -> bool {
    match event {
        CrosstermEvent::Key(KeyEvent {
            code: KeyCode::Char('c' | 'd'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => false,
        CrosstermEvent::Key(key) => app.update(Event::Key(key), message_sender),
        CrosstermEvent::Resize(w, h) => {
            renderer.resize(w, h);
            true
        }
        CrosstermEvent::Mouse(MouseEvent { kind, .. }) => {
            handle_mouse_event(app, kind, message_sender)
        }
        _ => true,
    }
}

#[inline]
fn handle_mouse_event<App: AsyncTerminalApp>(
    app: &mut App,
    kind: MouseEventKind,
    message_sender: &mpsc::UnboundedSender<App::Message>,
) -> bool {
    match kind {
        MouseEventKind::ScrollDown => app.update(
            Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
            message_sender,
        ),
        MouseEventKind::ScrollUp => app.update(
            Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
            message_sender,
        ),
        _ => true,
    }
}
