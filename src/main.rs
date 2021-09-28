use show_image::event::{ModifiersState, VirtualKeyCode, WindowEvent};
use show_image::{create_window, WindowProxy};

fn run_until_closed(window: WindowProxy) {
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

const API_URL: &str = "http://shibe.online/api/shibes";

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&API_URL.to_string())?.json::<Vec<String>>()?;

    let img_bytes = reqwest::blocking::get(&response[0])?.bytes()?;
    let img = image::load_from_memory(&img_bytes).expect("Image could not be decoded.");

    let window = create_window("Yet Another Shiba", Default::default())?;
    window.set_image("shiba-image", img)?;

    run_until_closed(window);

    Ok(())
}
