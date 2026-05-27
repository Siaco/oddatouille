mod app;
mod ui;
mod api;
mod cache;
mod events;

use app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    // Placeholder for terminal initialization
    println!("oddatui starting...");

    while app.running {
        // Main application loop
        app.quit(); // Immediately quit for now since loop isn't fully implemented
    }

    Ok(())
}
