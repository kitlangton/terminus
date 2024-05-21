struct SimpleApp {
    count: usize,
}

use crossterm::event::*;
use terminus::*;

impl SyncTerminalApp for SimpleApp {
    fn render(&self) -> impl View {
        let count = self.count.to_string();

        // for the numbers 0 through count, have a Vec of text(<index>)
        let children: Vec<Text> = (0..self.count).map(|i| text(&i.to_string())).collect::<Vec<_>>();
        vstack((hstack((text("Count:"), text(&count))), vstack(children)))
    }

    fn update(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.count = self.count.saturating_sub(1),
            KeyCode::Up => self.count = self.count.saturating_add(1),
            _ => {}
        }
    }
}

fn main() {
    let mut app = SimpleApp { count: 5 };
    app.execute(false);
}
