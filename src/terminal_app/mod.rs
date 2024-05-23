mod raw_mode_guard;
mod renderer;

use std::io::stdout;

use crate::*;
use async_trait::async_trait;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use raw_mode_guard::RawModeGuard;
use renderer::*;

use self::{fullscreen_renderer::FullScreenRenderer, inline_renderer::InlineRenderer};

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
    fn update(&mut self, event: Event<Self::Message>, sender: &mpsc::UnboundedSender<Self::Message>) -> bool;

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
pub trait AsyncTerminalAppExt: AsyncTerminalApp {
    async fn execute(&mut self, use_full_screen: bool) {
        let (message_sender, mut message_receiver) = mpsc::unbounded_channel::<Self::Message>();
        let (terminal_event_sender, mut terminal_event_receiver) = mpsc::unbounded_channel::<CrosstermEvent>();

        let mut renderer = create_renderer(use_full_screen);
        let _guard = RawModeGuard::new(use_full_screen);
        let terminal_event_task = handle_event(terminal_event_sender);

        self.init(&message_sender);

        loop {
            renderer.render(&self.render());

            tokio::select! {
                Some(event) = terminal_event_receiver.recv() => {
                    match event {
                        CrosstermEvent::Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        }) => break,
                        CrosstermEvent::Resize(w, h) => renderer.resize(w, h),
                        CrosstermEvent::Key(key) => {
                            if !self.update(Event::Key(key), &message_sender) {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                Some(msg) = message_receiver.recv() => {
                    if !self.update(Event::Message(msg), &message_sender) {
                        break;
                    }
                }
                else => break,
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

pub trait SyncTerminalApp {
    fn render(&self) -> impl View;
    fn update(&mut self, event: KeyEvent);
    fn handle_exit(&mut self) -> Option<impl View> {
        None as Option<EmptyView>
    }
}

pub trait SyncTerminalAppExt: SyncTerminalApp {
    fn execute(&mut self, use_full_screen: bool) {
        let mut renderer = create_renderer(use_full_screen);
        let _guard = RawModeGuard::new(use_full_screen);

        loop {
            renderer.render(&self.render());
            let event = crossterm::event::read().unwrap();
            match event {
                CrosstermEvent::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break,
                CrosstermEvent::Resize(width, height) => {
                    renderer.resize(width, height);
                }
                CrosstermEvent::Key(event) => {
                    self.update(event);
                }
                _ => {}
            }
        }

        self.handle_exit();
        renderer.render(&self.render());
        renderer.move_cursor_to_bottom_of_current_view();
    }
}

impl<T: SyncTerminalApp> SyncTerminalAppExt for T {}
