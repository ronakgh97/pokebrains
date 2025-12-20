use anyhow::Result;
use xcap::Window;

#[tokio::main]
async fn main() -> Result<()> {
    let windows = Window::all()?;

    // Print the titles of all captured windows
    for window in windows {
        if let Ok(title) = window.title() {
            println!("Window Title: {}", title);
        }
    }

    Ok(())
}
#[tokio::test]
async fn test_capture() {
    let windows = Window::all().unwrap();

    let rust_rover = windows.iter().find(|w| {
        w.title()
            .unwrap_or("Could able to find Rust Rover Window".to_string())
            .contains("pokebrains")
    });

    assert!(rust_rover.clone().is_some());
}
