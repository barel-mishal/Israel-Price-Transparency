use std::{error::Error, fs::File};

pub fn create_json_file<T>(file_path: &str, data: &T) -> Result<(), Box<dyn Error>>
where T: serde::Serialize {
    let json_file = File::create(file_path)?;
    serde_json::to_writer_pretty(json_file, data)?;
    println!("Parsed data saved to {}", file_path);
    Ok(())
}