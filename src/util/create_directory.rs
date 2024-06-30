
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Creates a directory if it doesn't already exist.
/// Returns a `Result` with an empty tuple on success or an error.
pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
        println!("Directory created: {:?}", path);
    } else {
        println!("Directory already exists: {:?}", path);
    }
    Ok(())
}