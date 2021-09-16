use show_image::event::{ModifiersState, VirtualKeyCode, WindowEvent};
use show_image::WindowProxy;

pub fn run_until_closed(window: WindowProxy) {
    // Wait for the window to be closed or Escape to be pressed.
    for event in window.event_channel().unwrap() {
        if let WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(VirtualKeyCode::Escape) {
                break;
            } else if event.input.key_code == Some(VirtualKeyCode::C)
                && event.input.modifiers == ModifiersState::CTRL
            {
                break;
            }
        }
    }
}
