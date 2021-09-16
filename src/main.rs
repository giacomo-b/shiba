use shiba::run_until_closed;
use show_image::create_window;

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
