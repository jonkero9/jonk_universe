use raylib::ffi::MouseButton;
use raylib::RaylibHandle;

#[derive(Debug, Clone)]
pub enum ScreenState {
    UniMap,
    StarSystemMap,
}

#[derive(Debug)]
pub struct MainWindow {
    pub screen_state: ScreenState,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            screen_state: ScreenState::UniMap,
        }
    }

    pub fn handle_screen_state_click(rl: &RaylibHandle, screen_state: &ScreenState) -> ScreenState {
        match screen_state {
            ScreenState::UniMap => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    return swap_screen_state(screen_state);
                }
                screen_state.clone()
            }
            ScreenState::StarSystemMap => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
                    return swap_screen_state(screen_state);
                }
                screen_state.clone()
            }
        }
    }
}

fn swap_screen_state(screen_state: &ScreenState) -> ScreenState {
    match screen_state {
        ScreenState::UniMap => ScreenState::StarSystemMap,
        ScreenState::StarSystemMap => ScreenState::UniMap,
    }
}
