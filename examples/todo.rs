use std::sync::Arc;

use crossterm::event::*;
use terminus::*;
use tokio::sync::mpsc;

struct Todo {
    name: String,
    is_complete: bool,
}

struct TodoApp {
    todos: Vec<Todo>,
    input: String,
    mode: AppMode,
    todo_index: usize,
}

enum AppMode {
    Viewing,
    Adding,
}

enum Message {
    AddTodo,
}

impl TodoApp {
    fn handle_key_event(&mut self, key_event: KeyEvent, tx: &mpsc::UnboundedSender<Message>) -> bool {
        match self.mode {
            AppMode::Viewing => self.handle_viewing_mode(key_event, tx),
            AppMode::Adding => self.handle_adding_mode(key_event),
        }
    }

    fn handle_viewing_mode(&mut self, key_event: KeyEvent, tx: &mpsc::UnboundedSender<Message>) -> bool {
        match key_event.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('n') => self.start_adding(tx),
            KeyCode::Up => {
                if self.todo_index > 0 {
                    self.todo_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.todo_index < self.todos.len().saturating_sub(1) {
                    self.todo_index += 1;
                }
            }
            KeyCode::Char(' ') => {
                if !self.todos.is_empty() {
                    let todo = &mut self.todos[self.todo_index];
                    todo.is_complete = !todo.is_complete;
                }
            }
            _ => {}
        }
        true
    }

    fn handle_adding_mode(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    self.todos.push(Todo {
                        name: self.input.clone(),
                        is_complete: false,
                    });
                    self.input.clear();
                    self.mode = AppMode::Viewing;
                }
            }
            KeyCode::Esc => {
                self.input.clear();
                self.mode = AppMode::Viewing;
            }
            _ => {}
        }
        true
    }

    fn start_adding(&mut self, tx: &mpsc::UnboundedSender<Message>) {
        self.mode = AppMode::Adding;
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(Message::AddTodo).unwrap();
        });
    }

    fn render_todo(&self, index: usize, todo: &Todo) -> impl View {
        let status = if todo.is_complete { "[x]" } else { "[ ]" };
        let prefix = if index == self.todo_index { ">" } else { " " };
        let text_view = text(&format!("{} {} {}", prefix, status, todo.name)).color(if todo.is_complete {
            Color::Red
        } else {
            Color::Reset
        });

        text_view.bold_when(index == self.todo_index)
    }

    fn render_todos(&self) -> impl View {
        vstack(
            self.todos
                .iter()
                .enumerate()
                .map(|(index, todo)| self.render_todo(index, todo))
                .collect::<Vec<_>>(),
        )
    }
}

impl AsyncTerminalApp for TodoApp {
    type Message = Message;

    fn render(&self) -> impl View {
        let todos = self.render_todos();

        let input_view = if let AppMode::Adding = self.mode {
            hstack(("Input:", self.input.clone())).as_any()
        } else {
            empty().as_any()
        };

        fn shortcut_text(key: &str, description: &str) -> impl View {
            hstack((text(key).color(Color::Blue).bold(), text(description).dim()))
        }

        let commands_view = hstack((
            shortcut_text("n", "new"),
            shortcut_text("space", "toggle"),
            shortcut_text("Up/Down", "navigate"),
            shortcut_text("q", "quit"),
        ))
        .spacing(2);

        vstack((text("Todos:").bold().underline(), todos, input_view, commands_view)).border()
    }

    fn update(&mut self, event: TerminalEvent<Self::Message>, tx: &mpsc::UnboundedSender<Self::Message>) -> bool {
        match event {
            TerminalEvent::Key(key_event) => self.handle_key_event(key_event, tx),
            TerminalEvent::Message(Message::AddTodo) => {
                if !self.input.is_empty() {
                    self.todos.push(Todo {
                        name: self.input.clone(),
                        is_complete: false,
                    });
                    self.input.clear();
                }
                true
            }
        }
    }

    fn handle_exit(&self) -> Option<impl View> {
        Some(text("You quit the app!"))
    }
}

#[tokio::main]
async fn main() {
    let mut app = TodoApp {
        todos: vec![
            Todo {
                name: "Buy Milk".to_string(),
                is_complete: false,
            },
            Todo {
                name: "Buy Bread".to_string(),
                is_complete: false,
            },
        ],
        input: String::new(),
        mode: AppMode::Viewing,
        todo_index: 0,
    };

    app.execute().await;
}
