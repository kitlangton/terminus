use terminus::*;
use tokio::sync::mpsc;

struct BinaryTutorialApp {
    count: i16,
    shift: i16,
    show_alphabet: bool,
    show_colors: bool,
    show_two_complement: bool,
}

impl Default for BinaryTutorialApp {
    fn default() -> Self {
        Self {
            count: 0,
            shift: 0,
            show_alphabet: false,
            show_colors: false,
            show_two_complement: false,
        }
    }
}

enum Message {}

impl BinaryTutorialApp {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _tx: &mpsc::UnboundedSender<Message>,
    ) -> bool {
        match key_event.code {
            KeyCode::Down => {
                if self.count == 0 {
                    self.count = 15;
                } else {
                    self.count = self.count.saturating_sub(1);
                }
            }
            KeyCode::Up => {
                self.count = self.count.saturating_add(1);
            }
            KeyCode::Left => {
                self.shift = self.shift.saturating_sub(1);
            }
            KeyCode::Right => {
                self.shift = self.shift.saturating_add(1);
            }
            KeyCode::Char('q') => return false,
            KeyCode::Char('a') => self.show_alphabet = !self.show_alphabet,
            KeyCode::Char('c') => self.show_colors = !self.show_colors,
            KeyCode::Char('t') => self.show_two_complement = !self.show_two_complement,
            KeyCode::Char(digit) if digit.is_digit(10) => {
                let digit = digit.to_digit(10).unwrap();
                self.count = digit as i16;
                self.shift = 0;
            }
            _ => {}
        }
        // wrap count around if greater than 16
        self.count = self.count % 16;
        true
    }
}

// 0000 0000
fn render_binary(n: i16) -> impl View {
    let bits = (0..4)
        .map(|i| {
            if n & (1 << (3 - i)) != 0 {
                "1".to_string()
            } else {
                "0".to_string()
            }
        })
        .collect::<Vec<_>>();
    render_helper(bits)
}

fn render_numbers(n: i16, shift: i16) -> impl View {
    let numbers = (0..16)
        .enumerate()
        .map(|(idx, i)| {
            let number = i as i16 + shift;
            text(number.to_string())
                .bold_when(idx as i16 == n)
                .dim_when(idx as i16 != n)
        })
        .collect::<Vec<_>>();
    hstack(numbers).spacing(1)
}

fn render_two_complement_numbers(n: i16) -> impl View {
    let numbers = (0..16)
        .enumerate()
        .map(|(idx, i)| {
            let number = if i >= 8 { i as i16 - 16 } else { i as i16 };
            text(number.to_string())
                .bold_when(idx as i16 == n)
                .dim_when(idx as i16 != n)
        })
        .collect::<Vec<_>>();
    hstack(numbers).spacing(1)
}

fn render_alphabet(n: i16, shift: i16) -> impl View {
    let alphabet = (0..16)
        .enumerate()
        .map(|(idx, i)| {
            let letter = (((i as i16 + shift) % 26 + 26) % 26 + 65) as u8 as char;
            text(letter.to_string())
                .bold_when(idx as i16 == n)
                .dim_when(idx as i16 != n)
        })
        .collect::<Vec<_>>();
    hstack(alphabet).spacing(1)
}

fn render_colors(selected: i16, shift: i16) -> impl View {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
    ];

    let blocks = (0..16)
        .enumerate()
        .map(|(idx, i)| {
            let is_selected = idx as i16 == selected;
            let color = colors[((i + shift + 16) % 16) as usize]; // Ensure wrapping both ways
            let char = if is_selected { '█' } else { '░' };
            text(char.to_string()).color(color)
        })
        .collect::<Vec<_>>();
    hstack(blocks).spacing(1)
}

fn render_helper(parts: Vec<String>) -> impl View {
    let mut leading_zero = true;
    let parts = parts
        .into_iter()
        .map(|s| {
            if leading_zero && s == "0" {
                text(s).dim().as_any()
            } else {
                leading_zero = false;
                text(s).as_any()
            }
        })
        .collect::<Vec<_>>();
    hstack(parts)
}

impl AsyncTerminalApp for BinaryTutorialApp {
    type Message = Message;

    fn render(&self) -> impl View {
        vstack((
            render_numbers(self.count, self.shift)
                .border()
                .title(" Numbers ")
                .border_style(BorderStyle::Rounded),
            render_two_complement_numbers(self.count)
                .border()
                .title(" Two's Complement ")
                .border_style(BorderStyle::Rounded)
                .visible(self.show_two_complement),
            render_alphabet(self.count, self.shift)
                .border()
                .title(" Alphabet ")
                .border_style(BorderStyle::Rounded)
                .visible(self.show_alphabet),
            render_colors(self.count, self.shift)
                .border()
                .title(" Colors ")
                .border_style(BorderStyle::Rounded)
                .visible(self.show_colors),
            render_binary(self.count)
                .border()
                .title(" Binary ")
                .border_style(BorderStyle::Rounded),
        ))
        .fill_horizontally()
        .fill_vertically()
        .padding_h(8)
        .padding_v(4)
        // .alignment(HorizontalAlignment::Center)
        // .center()
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
}

#[tokio::main]
async fn main() {
    let mut app = BinaryTutorialApp::default();
    app.execute(true).await;
}
