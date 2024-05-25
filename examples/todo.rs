use terminus::*;
use tokio::sync::mpsc;
use uuid::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Todo {
    id: Uuid,
    name: String,
    is_complete: bool,
}

impl Todo {
    fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            is_complete: false,
        }
    }
}

struct TodoApp {
    todos: Vec<Todo>,
    input: String,
    mode: AppMode,
    todo_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AppMode {
    Viewing,
    Adding,
}

enum Message {}

impl TodoApp {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        tx: &mpsc::UnboundedSender<Message>,
    ) -> bool {
        match self.mode {
            AppMode::Viewing => self.handle_viewing_mode(key_event, tx),
            AppMode::Adding => self.handle_adding_mode(key_event),
        }
    }

    fn handle_viewing_mode(
        &mut self,
        key_event: KeyEvent,
        _tx: &mpsc::UnboundedSender<Message>,
    ) -> bool {
        match key_event.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('n') => self.mode = AppMode::Adding,
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
                    self.todos.insert(0, Todo::new(&self.input));
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

    fn render_todo(&self, index: usize, todo: &Todo) -> impl View {
        let is_selected = index == self.todo_index && self.mode == AppMode::Viewing;

        let cursor = if_then_view(
            is_selected,
            RenderCounter {}.blue(),
            RenderCounter {}.green(),
        );

        hstack((
            cursor,
            text(todo.name.clone()).strikethrough_when(todo.is_complete),
        ))
        .dim_when(todo.is_complete)
        .bold_when(is_selected)
    }

    fn input_view(&self) -> impl View {
        hstack((
            text("+"), //
            text(self.input.clone()),
        ))
        .green()
        .bold()
        .visible(self.mode == AppMode::Adding)
    }

    fn render_todos(&self) -> impl View {
        vstack((
            self.input_view(),
            vstack(
                self.todos
                    .iter()
                    .enumerate()
                    .map(|(index, todo)| self.render_todo(index, todo).id(todo.id))
                    .collect::<Vec<_>>(),
            ),
        ))
    }
}

impl AsyncTerminalApp for TodoApp {
    type Message = Message;

    fn render(&self) -> impl View {
        let todos = self.render_todos();

        fn shortcut_text(key: &str, description: &str) -> impl View {
            hstack((
                text(key).bold(), //
                text(description).dim(),
            ))
            .blue()
        }

        let commands_view = hstack((
            shortcut_text("n", "new"),
            shortcut_text("space", "toggle"),
            shortcut_text("Up/Down", "navigate"),
            shortcut_text("q", "quit"),
        ))
        .spacing(2);

        vstack((
            terminus::view::RenderCounter {},
            todos.fill(),
            commands_view,
        ))
        .border()
        .title(" TODOS ")
    }

    fn update(
        &mut self,
        event: Event<Self::Message>,
        tx: &mpsc::UnboundedSender<Self::Message>,
    ) -> bool {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event, tx),
            Event::Message(_) => true,
        }
    }

    fn handle_exit(&self) -> Option<impl View> {
        Some(text("You quit the app!"))
    }
}

#[tokio::main]
async fn main() {
    let mut app = TodoApp {
        todos: vec![Todo::new("Buy Milk"), Todo::new("Buy Bread")],
        input: String::new(),
        mode: AppMode::Viewing,
        todo_index: 0,
    };

    app.run(true).await;
}
