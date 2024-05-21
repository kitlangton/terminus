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

pub enum TerminalEvent<M> {
    Key(KeyEvent),
    Message(M),
}

pub trait AsyncTerminalApp {
    type Message: Send + 'static;
    fn render(&self) -> impl View;
    fn update(&mut self, event: TerminalEvent<Self::Message>, tx: &mpsc::UnboundedSender<Self::Message>) -> bool;

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
    async fn execute(&mut self) {
        let (msg_tx, mut msg_rx) = mpsc::unbounded_channel::<Self::Message>();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<crossterm::event::Event>();
        let mut renderer = Renderer::new(stdout());
        let event_task = handle_event(event_tx);
        let _guard = RawModeGuard::new();

        loop {
            renderer.render(&self.render());

            tokio::select! {
                Some(event) = event_rx.recv() => match event {
                    crossterm::event::Event::Resize(w, h) => renderer.resize(w, h),
                    crossterm::event::Event::Key(key) => {
                        if !self.update(TerminalEvent::Key(key), &msg_tx) {
                            break;
                        }
                    }
                    _ => {}
                },
                Some(msg) = msg_rx.recv() => {
                    if !self.update(TerminalEvent::Message(msg), &msg_tx) {
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
    fn execute(&mut self) {
        let mut renderer = Renderer::new(stdout());
        let _guard = RawModeGuard::new();
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
