use std::fs::File;
use std::io::{self, Write};
use serde_json;
use std::error::Error;

use crate::maayan_2000::FileInfo;

/// Saves data to a JSON file.
/// Returns a `Result` with an empty tuple on success or an error.
pub fn save_json(data: &Vec<FileInfo>, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let json = serde_json::to_string_pretty(&data)?;
    let mut writer = io::BufWriter::new(file);
    writer.write_all(json.as_bytes())?;
    println!("Data saved to {}", filename);
    Ok(())
}