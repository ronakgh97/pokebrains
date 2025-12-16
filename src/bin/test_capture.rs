use anyhow::Result;
use xcap::Window;

#[tokio::main]
async fn main() -> Result<()> {
    let windows = Window::all()?;

    //Find Pokémon showdown window
    let showdown_window = windows.iter().find(|w| {
        w.title()
            .unwrap_or("Could able to find Showdown Window".to_string())
            .contains("Showdown")
    });

    let image = match showdown_window {
        Some(window) => {
            let img = window.capture_image()?;
            img
        }
        None => {
            panic!("Could not find Pokémon Showdown window");
        }
    };

    image.save("./capture/showdown_capture.png")?;

    Ok(())
}
