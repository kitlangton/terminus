mod raw_mode_guard;
mod renderer;

use std::io::stdout;

use crate::*;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use raw_mode_guard::RawModeGuard;
use renderer::*;

use self::{fullscreen_renderer::FullScreenRenderer, inline_renderer::InlineRenderer};

pub enum TerminalEvent<M> {
    Key(KeyEvent),
    Message(M),
}

pub trait AsyncTerminalApp {
    type Message: Send + 'static;
    fn render(&self) -> impl View;
    fn update(&mut self, event: TerminalEvent<Self::Message>, tx: &mpsc::UnboundedSender<Self::Message>) -> bool;

    fn init(&mut self, tx: &mpsc::UnboundedSender<Self::Message>) {}

    fn handle_exit(&self) -> Option<impl View> {
        None as Option<EmptyView>
    }
}

fn handle_event(tx: mpsc::UnboundedSender<crossterm::event::Event>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut reader = crossterm::event::EventStream::new();
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

pub struct VecWriter {
    buffer: Vec<u8>,
}

#[async_trait]
pub trait AsyncTerminalAppExt: AsyncTerminalApp {
    async fn execute(&mut self, use_full_screen: bool) {
        let (message_sender, mut message_receiver) = mpsc::unbounded_channel::<Self::Message>();
        let (terminal_event_sender, mut terminal_event_receiver) = mpsc::unbounded_channel::<crossterm::event::Event>();
        let mut renderer = if use_full_screen {
            SomeRenderer::FullScreen(FullScreenRenderer::new(stdout()))
        } else {
            SomeRenderer::Inline(InlineRenderer::new(stdout()))
        };
        let event_task = handle_event(terminal_event_sender);
        let _guard = RawModeGuard::new(true);

        self.init(&message_sender);

        loop {
            renderer.render(&self.render());

            tokio::select! {
                Some(event) = terminal_event_receiver.recv() => match event {
                    crossterm::event::Event::Resize(w, h) => renderer.resize(w, h),
                    crossterm::event::Event::Key(key) => {
                        if !self.update(TerminalEvent::Key(key), &message_sender) {
                            break;
                        }
                    }
                    _ => {}
                },
                Some(msg) = message_receiver.recv() => {
                    if !self.update(TerminalEvent::Message(msg), &message_sender) {
                        break;
                    }
                }
                else => break,
            }
        }

        if let Some(view) = self.handle_exit() {
            renderer.render(&view);
        }
        // renderer.move_cursor_to_bottom_of_current_view();

        event_task.abort();
        event_task.await.unwrap_err();
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
        let mut renderer = if use_full_screen {
            SomeRenderer::FullScreen(FullScreenRenderer::new(stdout()))
        } else {
            SomeRenderer::Inline(InlineRenderer::new(stdout()))
        };

        let _guard = RawModeGuard::new(false);
        loop {
            renderer.render(&self.render());
            let event = crossterm::event::read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break,
                Event::Resize(width, height) => {
                    renderer.resize(width, height);
                }
                Event::Key(event) => {
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
