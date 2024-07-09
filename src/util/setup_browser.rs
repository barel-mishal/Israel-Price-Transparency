use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::{error::Error, path::PathBuf};


/// Sets up the headless browser with custom user data directory.
/// Returns a `Result` with the browser instance or an error.
pub fn setup_browser(user_data_dir: &str) -> Result<Browser, Box<dyn Error>> {
    let user_data_dir = PathBuf::from(user_data_dir);

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .window_size(Some((800, 600)))
            .user_data_dir(Some(user_data_dir.into()))
            .build()
            .expect("Failed to launch browser"),
    )?;
    Ok(browser)
}