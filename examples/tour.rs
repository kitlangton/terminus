use std::fmt::Display;

use terminus::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    ZStack,
    Frame,
    Stack,
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tab::Frame => "Frame",
            Tab::Stack => "Stack",
            Tab::ZStack => "ZStack",
        };
        write!(f, "{}", s)
    }
}

const TABS: [Tab; 3] = [Tab::Frame, Tab::Stack, Tab::ZStack];

impl Tab {
    fn next(&self) -> Self {
        let index = TABS.iter().position(|x| x == self).unwrap();
        TABS[(index + 1) % TABS.len()]
    }
}

struct FrameTab {
    alignment: Alignment,
}

struct ZStackTab {
    alignment: Alignment,
}

struct TourApp {
    tab: Tab,
    frame_tab: FrameTab,
    zstack_tab: ZStackTab,
    tick: usize,
}

enum Message {
    Tick,
}

impl AsyncTerminalApp for TourApp {
    type Message = Message;

    fn render(&self) -> impl View {
        let main_view = match self.tab {
            Tab::Frame => frame_tab_view(&self.frame_tab).as_any(),
            Tab::Stack => "STACK".as_any(),
            Tab::ZStack => stack_tab_view(self.tick, &self.zstack_tab).as_any(),
        };

        let tab_title = self.tab.to_string().to_uppercase();
        let current_frame_tick = format!("Frame Tick: {}", self.tick);

        vstack((
            current_frame_tick,
            main_view.border().border_style(BorderStyle::Rounded).title(tab_title),
            tab_bar_view(self.tab),
        ))
    }

    fn update(
        &mut self,
        event: Event<Self::Message>,
        _sender: &tokio::sync::mpsc::UnboundedSender<Self::Message>,
    ) -> bool {
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Tab {
                    self.tab = self.tab.next();
                }

                match self.tab {
                    Tab::Frame => handle_key_frame_tab(&mut self.frame_tab, key_event),
                    Tab::Stack => (),
                    Tab::ZStack => handle_key_zstack_tab(&mut self.zstack_tab, key_event),
                }
            }
            Event::Message(Message::Tick) => {
                self.tick = self.tick.wrapping_add(1);
            }
        }
        true
    }

    fn init(&mut self, sender: &tokio::sync::mpsc::UnboundedSender<Self::Message>) {
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(50));
            loop {
                interval.tick().await;
                sender_clone.send(Message::Tick).unwrap();
            }
        });
    }
}

/// Renders the tabs for the main view. The active tab is green and underlined.
fn tab_bar_view(tab: Tab) -> impl View {
    let tabs = TABS
        .iter()
        .map(|t| {
            let is_active = *t == tab;
            let color = if is_active { Color::Green } else { Color::Reset };
            vstack(t.to_string().to_uppercase().underline_when(is_active).color(color))
        })
        .collect::<Vec<_>>();

    hstack(("PRESS TAB".dim(), hstack(tabs)))
        .fill_horizontally()
        .border()
        .border_style(BorderStyle::Rounded)
        .title(" TABS ")
}

fn frame_tab_view(frame_tab: &FrameTab) -> impl View {
    vstack(("FRAME", format!("Alignment: {}", frame_tab.alignment).green())).frame(
        None,
        None,
        Some(u16::MAX),
        Some(u16::MAX),
        frame_tab.alignment,
    )
}

fn stack_tab_view(tick: usize, tab: &ZStackTab) -> impl View {
    let alignment_string = match tab.alignment {
        Alignment::CENTER => "CENTER",
        Alignment::TOP => "TOP",
        Alignment::BOTTOM => "BOTTOM",
        Alignment::LEFT => "LEFT",
        Alignment::RIGHT => "RIGHT",
        Alignment::TOP_LEFT => "TOP LEFT",
        Alignment::TOP_RIGHT => "TOP RIGHT",
        Alignment::BOTTOM_LEFT => "BOTTOM LEFT",
        Alignment::BOTTOM_RIGHT => "BOTTOM RIGHT",
    };

    let color_for_alignment = match tab.alignment.vertical {
        VerticalAlignment::Top => Color::Green,
        VerticalAlignment::Center => Color::Blue,
        VerticalAlignment::Bottom => Color::Red,
    };

    let overlay = format!("I AM ON {} OF THE WORLD!", alignment_string)
        .bold()
        .padding(1)
        .background(color_for_alignment);

    let background = with_size(move |size| {
        fn make_line(line_number: u16, width: u16, tick: usize) -> String {
            let base_string = format!("{} ", (line_number as usize) + tick);
            let repeat_count = (width as usize + base_string.len() - 1) / base_string.len();
            let repeated_string: String = base_string.repeat(repeat_count);
            repeated_string[..width as usize].to_string()
        }

        vstack(
            (1..=size.height)
                .map(|line| make_line(line, size.width, tick))
                .collect::<Vec<_>>(),
        )
    });

    zstack((background.dim().red(), overlay)).alignment(tab.alignment)
}

#[tokio::main]
async fn main() {
    let mut app = TourApp {
        tab: Tab::ZStack,
        frame_tab: FrameTab {
            alignment: Alignment::CENTER,
        },
        zstack_tab: ZStackTab {
            alignment: Alignment::TOP,
        },
        tick: 0,
    };

    app.execute(true).await;
}

fn handle_key_zstack_tab(tab: &mut ZStackTab, event: KeyEvent) {
    tab.alignment = modify_alignment_with_key(tab.alignment, event.code);
}

fn handle_key_frame_tab(tab: &mut FrameTab, event: KeyEvent) {
    tab.alignment = modify_alignment_with_key(tab.alignment, event.code);
}

fn modify_alignment_with_key(alignment: Alignment, code: KeyCode) -> Alignment {
    match (code, alignment) {
        (KeyCode::Right, Alignment::CENTER) => Alignment::RIGHT,
        (KeyCode::Left, Alignment::CENTER) => Alignment::LEFT,
        (KeyCode::Down, Alignment::CENTER) => Alignment::BOTTOM,
        (KeyCode::Up, Alignment::CENTER) => Alignment::TOP,

        (KeyCode::Left, Alignment::RIGHT) => Alignment::CENTER,
        (KeyCode::Up, Alignment::RIGHT) => Alignment::TOP_RIGHT,
        (KeyCode::Down, Alignment::RIGHT) => Alignment::BOTTOM_RIGHT,

        (KeyCode::Right, Alignment::LEFT) => Alignment::CENTER,
        (KeyCode::Down, Alignment::LEFT) => Alignment::BOTTOM_LEFT,
        (KeyCode::Up, Alignment::LEFT) => Alignment::TOP_LEFT,

        (KeyCode::Down, Alignment::TOP) => Alignment::CENTER,
        (KeyCode::Left, Alignment::TOP) => Alignment::TOP_LEFT,
        (KeyCode::Right, Alignment::TOP) => Alignment::TOP_RIGHT,

        (KeyCode::Up, Alignment::BOTTOM) => Alignment::CENTER,
        (KeyCode::Left, Alignment::BOTTOM) => Alignment::BOTTOM_LEFT,
        (KeyCode::Right, Alignment::BOTTOM) => Alignment::BOTTOM_RIGHT,

        (KeyCode::Right, Alignment::TOP_LEFT) => Alignment::TOP,
        (KeyCode::Down, Alignment::TOP_LEFT) => Alignment::LEFT,

        (KeyCode::Left, Alignment::TOP_RIGHT) => Alignment::TOP,
        (KeyCode::Down, Alignment::TOP_RIGHT) => Alignment::RIGHT,

        (KeyCode::Right, Alignment::BOTTOM_LEFT) => Alignment::BOTTOM,
        (KeyCode::Up, Alignment::BOTTOM_LEFT) => Alignment::LEFT,

        (KeyCode::Left, Alignment::BOTTOM_RIGHT) => Alignment::BOTTOM,
        (KeyCode::Up, Alignment::BOTTOM_RIGHT) => Alignment::RIGHT,

        _ => alignment,
    }
}
