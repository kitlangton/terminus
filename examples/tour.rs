use std::fmt::Display;

use altar::*;

#[tokio::main]
async fn main() {
    let mut app = TourApp::default();
    app.run(true).await;
}

#[derive(Clone, Copy, PartialEq, Hash)]
enum Tab {
    ZStack,
    Frame,
    List,
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tab::Frame => "Frame",
            Tab::List => "List",
            Tab::ZStack => "ZStack",
        };
        write!(f, "{}", s)
    }
}

impl Tab {
    const ALL: [Tab; 3] = [Tab::Frame, Tab::List, Tab::ZStack];

    fn next(&self) -> Self {
        let index = Self::ALL.iter().position(|x| x == self).unwrap();
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    fn prev(&self) -> Self {
        let index = Self::ALL.iter().position(|x| x == self).unwrap();
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
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
    list_tab: ListTab,
    tick: usize,
}

impl Default for TourApp {
    fn default() -> Self {
        Self {
            tab: Tab::List,
            frame_tab: FrameTab {
                alignment: Alignment::CENTER,
            },
            zstack_tab: ZStackTab {
                alignment: Alignment::TOP,
            },
            list_tab: ListTab {
                selected_index: 0,
                items: vec![
                    "Buy Milk".to_string(),
                    "Buy Bread".to_string(),
                    "Buy Cheese".to_string(),
                    "Buy Butter".to_string(),
                    "Buy Eggs".to_string(),
                    "Buy Flour".to_string(),
                    "Buy Sugar".to_string(),
                    "Buy Salt".to_string(),
                    "Buy Water".to_string(),
                ],
            },
            tick: 0,
        }
    }
}

enum Message {
    Tick,
}

impl AsyncTerminalApp for TourApp {
    type Message = Message;

    fn render(&self) -> impl View {
        let main_view = match self.tab {
            Tab::Frame => frame_tab_view(&self.frame_tab).as_any(),
            Tab::List => list_tab_view(&self.list_tab).as_any(),
            Tab::ZStack => stack_tab_view(self.tick, &self.zstack_tab).as_any(),
        };

        vstack((main_view, tab_bar_view(self.tab)))
    }

    fn update(
        &mut self,
        event: Event<Self::Message>,
        _sender: &tokio::sync::mpsc::UnboundedSender<Self::Message>,
    ) -> bool {
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    return false;
                }

                if key_event.code == KeyCode::Tab {
                    self.tab = self.tab.next();
                } else if key_event.code == KeyCode::BackTab {
                    self.tab = self.tab.prev();
                }

                match self.tab {
                    Tab::Frame => handle_key_frame_tab(&mut self.frame_tab, key_event),
                    Tab::List => handle_key_list_tab(&mut self.list_tab, key_event),
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
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(17));
            loop {
                interval.tick().await;
                sender_clone.send(Message::Tick).unwrap();
            }
        });
    }
}

/// Renders the tabs for the main view. The active tab is green and underlined.
fn tab_bar_view(tab: Tab) -> impl View {
    let tabs = Tab::ALL
        .iter()
        .map(|t| {
            let is_active = *t == tab;
            let color = if is_active {
                Color::Green
            } else {
                Color::Reset
            };
            t.to_string()
                .to_uppercase()
                .underline_when(is_active)
                .color(color)
                .id(t)
        })
        .collect::<Vec<_>>();

    hstack(("PRESS TAB".dim(), hstack(tabs)))
        .fill_horizontally()
        .border()
        .border_style(BorderStyle::Rounded)
        .title(" TABS ")
}

/// FRAME TAB

fn handle_key_frame_tab(tab: &mut FrameTab, event: KeyEvent) {
    tab.alignment = modify_alignment_with_key(tab.alignment, event.code);
}

fn frame_tab_view(frame_tab: &FrameTab) -> impl View {
    vstack((
        "FRAME",
        format!("Alignment: {}", frame_tab.alignment).green(),
    ))
    .alignment(frame_tab.alignment.horizontal)
    .frame(
        None,
        None,
        Some(u16::MAX),
        Some(u16::MAX),
        frame_tab.alignment,
    )
    .border()
    .border_style(BorderStyle::Rounded)
    .title(" FRAME ")
}

/// LIST TAB

pub struct ListTab {
    selected_index: usize,
    items: Vec<String>,
}

pub fn handle_key_list_tab(tab: &mut ListTab, event: KeyEvent) {
    match event.code {
        KeyCode::Down => {
            tab.selected_index = tab
                .selected_index
                .saturating_add(1)
                .clamp(0, tab.items.len() - 1)
        }
        KeyCode::Up => {
            tab.selected_index = tab
                .selected_index
                .saturating_sub(1)
                .clamp(0, tab.items.len() - 1)
        }
        _ => (),
    }
}

pub fn list_tab_view(list_tab: &ListTab) -> impl View {
    let items = list_tab
        .items
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let is_active = i == list_tab.selected_index;
            let color = if is_active {
                Color::Green
            } else {
                Color::Reset
            };
            let message = if is_active { ">" } else { " " };
            hstack((
                message.color(color),
                format!("ITEM {}", i).color(color),
                text(x),
            ))
            .id(i)
        })
        .collect::<Vec<_>>();

    let selected_item = list_tab.items[list_tab.selected_index].clone();

    let selected_item_info_view = vstack((
        hstack(("SELECTED ITEM", selected_item.clone())),
        format!("THE CURRENT ITEM IS {}", selected_item).green(),
    ))
    .fill_vertically()
    .fill_horizontally();

    hstack((
        scroll_view(items, list_tab.selected_index)
            .fill_vertically()
            .border()
            .border_style(BorderStyle::Rounded)
            .title(" LIST "), //
        selected_item_info_view
            .border()
            .border_style(BorderStyle::Rounded)
            .title(" INFO "),
    ))
}

// TODO: Improve this and pull it into the library.
pub fn scroll_view<V: View + Clone>(
    views: Vec<IdentifiedView<V>>,
    selected_index: usize,
) -> impl View {
    with_size(move |available_size| {
        let sizes = views
            .iter()
            .map(|view| view.value.size(available_size))
            .collect::<Vec<_>>();

        let mut start_offset = 0;
        let mut end_offset = 0;

        let mut includes_selected = false;

        while !includes_selected {
            let mut total_height = 0;
            for (i, size) in sizes.iter().enumerate().skip(start_offset) {
                total_height += size.height;
                if total_height > available_size.height {
                    break;
                }
                end_offset = i;
            }

            if (start_offset..=end_offset).contains(&selected_index) {
                includes_selected = true;
            } else {
                start_offset += 1;
            }
        }

        vstack(views[start_offset..=end_offset].to_vec())
    })
}

/// ZSTACK TAB

fn handle_key_zstack_tab(tab: &mut ZStackTab, event: KeyEvent) {
    tab.alignment = modify_alignment_with_key(tab.alignment, event.code);
}

fn stack_tab_view(tick: usize, tab: &ZStackTab) -> impl View {
    let alignment_string = tab.alignment.to_string().to_uppercase();

    let color_for_alignment = match tab.alignment.vertical {
        VerticalAlignment::Top => Color::Green,
        VerticalAlignment::Center => Color::Blue,
        VerticalAlignment::Bottom => Color::Red,
    };

    let logo = vstack((
        text("████████ ███████ ██████  ███    ███ ██ ███    ██ ██    ██ ███████"),
        text("   ██    ██      ██   ██ ████  ████ ██ ████   ██ ██    ██ ██     "),
        text("   ██    █████   ██████  ██ ████ ██ ██ ██ ██  ██ ██    ██ ███████"),
        text("   ██    ██      ██   ██ ██  ██  ██ ██ ██  ██ ██ ██    ██      ██"),
        text("   ██    ███████ ██   ██ ██      ██ ██ ██   ████  ██████  ███████"),
    ))
    .bold();

    let overlay = hstack(("I AM ON ", alignment_string.underline(), " OF THE WORLD!"))
        .spacing(0)
        .bold();

    let background = with_size(move |size| {
        fn make_line(line_number: u16, width: u16, tick: usize) -> impl View {
            let base_string = format!("{} ", (line_number as usize) + tick);
            let repeat_count = (width as usize + base_string.len() - 1) / base_string.len();
            let repeated_string: String = base_string.repeat(repeat_count);
            repeated_string[..width as usize].to_string()
        }

        vstack(
            (1..=size.height)
                .map(|line| make_line(line, size.width, tick).id(line))
                .collect::<Vec<_>>(),
        )
    });

    zstack((
        background.dim().red(),
        vstack((logo, overlay))
            .alignment(tab.alignment.horizontal)
            .color(color_for_alignment),
    ))
    .alignment(tab.alignment)
    .border()
    .border_style(BorderStyle::Rounded)
    .title(" ZSTACK ")
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
