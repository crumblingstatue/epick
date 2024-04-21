mod app;
mod color;
mod color_picker;
mod context;
mod display_picker;
mod error;
mod keybinding;
mod math;
mod render;
mod screen_size;
mod settings;
mod ui;
mod zoom_picker;

pub use app::App as Epick;

use anyhow::{Context, Error};

fn save_to_clipboard(text: String) -> Result<(), Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard
        .set_text(text)
        .context("failed to save to clipboard")
}

fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1)
}

fn elapsed(timestamp: u64) -> u64 {
    get_timestamp() - timestamp
}
