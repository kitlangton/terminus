use super::*;

pub trait SyncTerminalApp {
    fn render(&self) -> impl View;
    fn update(&mut self, event: KeyEvent);
    fn handle_exit(&mut self) -> Option<impl View> {
        None as Option<EmptyView>
    }
}

pub trait SyncTerminalAppExt: SyncTerminalApp {
    fn run(&mut self, use_full_screen: bool) {
        let mut renderer = create_renderer(use_full_screen);
        let _guard = RawModeGuard::new(use_full_screen);

        loop {
            renderer.render(&self.render());
            let event = crossterm::event::read().unwrap();
            match event {
                CrosstermEvent::Key(KeyEvent {
                    code: KeyCode::Char('c' | 'd'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break,
                CrosstermEvent::Mouse(MouseEvent { kind, .. }) => match kind {
                    MouseEventKind::ScrollDown => {
                        self.update(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
                    }
                    MouseEventKind::ScrollUp => {
                        self.update(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE))
                    }
                    _ => {}
                },
                CrosstermEvent::Resize(width, height) => {
                    renderer.resize(width, height);
                }
                CrosstermEvent::Key(event) => {
                    self.update(event);
                }
                _ => {}
            }
        }

        if let Some(view) = self.handle_exit() {
            renderer.render(&view);
        }
        renderer.move_cursor_to_bottom_of_current_view();
    }
}

impl<T: SyncTerminalApp> SyncTerminalAppExt for T {}
