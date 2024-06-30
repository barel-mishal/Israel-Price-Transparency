use std::error::Error;
use std::io::{BufReader, Cursor};
use flate2::read::GzDecoder;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::de::from_str as xml_from_str;


use crate::create_json_file::create_json_file;
use crate::maayan_2000::{FileInfo, Items, Maayan2000, Promotions};

pub fn parse_gzip_file(file_info: &FileInfo, download_dir: &str, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("File is gzip-compressed, attempting to decompress");
    let gz_decoder = GzDecoder::new(&bytes[..]);
    let mut gz_reader = BufReader::new(gz_decoder);
    let mut contents = String::new();
    gz_reader.read_to_string(&mut contents)?;
    println!("Decompressed file contents: {}", contents);
    if let Err(e) = parse_and_save_xml_as_json(&file_info.file_name, download_dir, &contents) {
        eprintln!("Error: {:?}", e);
    }
    Ok(())
}

pub fn parse_zip_file(download_dir: &str, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("File is ZIP-compressed, attempting to decompress");
    let cursor = Cursor::new(&bytes[..]);
    let mut archive = ZipArchive::new(cursor)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        println!("Found file in ZIP: {}", file.name());
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines: Vec<_> = contents.lines().take(10).collect();
        println!("Contents of file {}", file.name());
        if let Err(e) = parse_and_save_xml_as_json(file.name(), download_dir, &contents) {
            println!("Contents of file {}: {}", file.name(), lines.join("\n"));
            eprintln!("Error: {:?}", e);
        }
    }
    Ok(())
}

pub fn parse_and_save_xml_as_json(file_info: &str, download_dir: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let json_filename = format!("{}.json", file_info);
    let json_path = format!("{}/{}", download_dir, json_filename);
    if file_info.ends_with(".xml") {
        let lines = contents.lines().take(30).collect::<Vec<_>>();
        println!("First 30 lines of XML file: \n{}\n", lines.join("\n"));
        
        match xml_from_str::<Maayan2000<Promotions>>(&contents) {
            Ok(root) => {
                println!("Root: {:?}", root);
                create_json_file(&json_path, &root)?;
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error parsing Root: {:?}", e);
            }
        }

        match xml_from_str::<Maayan2000<Items>>(&contents) {
            Ok(root_price) => {
                println!("RootPrice: {:?}", root_price);
                create_json_file(&json_path, &root_price)?;
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error parsing RootPrice: {:?}", e);
            }
        }
        
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported XML structure")));
    }
    Ok(())
}
