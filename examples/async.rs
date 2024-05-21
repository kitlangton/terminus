use std::sync::Arc;

use crossterm::event::*;
use terminus::{
    terminal_app::{AsyncTerminalApp, AsyncTerminalAppExt, TerminalEvent},
    *,
};
use tokio::sync::mpsc;

struct SimpleAsyncApp {
    count: usize,
    super_charge: Option<usize>,
}

enum Message {
    Charge,
}

impl SimpleAsyncApp {
    fn handle_key_event(&mut self, key_event: KeyEvent, tx: &mpsc::UnboundedSender<Message>) -> bool {
        match key_event.code {
            KeyCode::Down => self.count = self.count.saturating_sub(1),
            KeyCode::Up => self.count = self.count.saturating_add(1),
            KeyCode::Left => self.charged_adjust(false),
            KeyCode::Right => self.charged_adjust(true),
            KeyCode::Char('q') => return false,
            KeyCode::Char('c') => self.start_charging(tx),
            _ => {}
        }
        true
    }

    fn charged_adjust(&mut self, should_add: bool) {
        if let Some(charge) = self.super_charge {
            if charge == 10 {
                if should_add {
                    self.count = self.count.saturating_add(10);
                } else {
                    self.count = self.count.saturating_sub(10);
                }
                self.super_charge = None;
            }
        }
    }

    fn start_charging(&mut self, tx: &mpsc::UnboundedSender<Message>) {
        if self.super_charge.is_none() {
            self.super_charge = Some(0);
            let tx = tx.clone();
            tokio::spawn(async move {
                for _ in 0..10 {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    tx.send(Message::Charge).unwrap();
                }
            });
        }
    }
}

impl AsyncTerminalApp for SimpleAsyncApp {
    type Message = Message;

    fn render(&self) -> impl View {
        let count = self.count.to_string();

        let children = (0..self.count).map(|i| text(&i.to_string())).collect::<Vec<_>>();

        let charging = self.super_charge.map(|c| {
            let slots: String = "=".repeat(c) + &" ".repeat(10 - c);
            hstack(("CHARGING: ", hstack(("|", slots, "|")).spacing(0).color(Color::Red)))
        });
        vstack((hstack(("Count:", count)), charging, vstack(children)))
    }

    fn update(&mut self, event: TerminalEvent<Self::Message>, tx: &mpsc::UnboundedSender<Self::Message>) -> bool {
        match event {
            TerminalEvent::Key(key_event) => self.handle_key_event(key_event, tx),
            TerminalEvent::Message(Message::Charge) => {
                self.super_charge = self.super_charge.map(|x| x.saturating_add(1));
                true
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut app = SimpleAsyncApp {
        count: 5,
        super_charge: None,
    };

    app.execute().await;
}
