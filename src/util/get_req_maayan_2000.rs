use std::error::Error;

use reqwest::{Client, Url};

use crate::maayan_2000::FileInfo;

pub async fn download_maayan_2000_file(base_url: &str, file_info: &FileInfo) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let download_url = Url::parse(base_url)?.join(&format!("Download.aspx?FileNm={}", file_info.file_name))?;
    
    let body = "{}"; // Since we are sending an empty JSON object
    let response = client.post(download_url)
        .header("Content-Type", "application/json")
        .header("Content-Length", body.len().to_string())
        .body(body)
        .send()
        .await?;

    let response_text = response.text().await?;

    // Try to parse the response as JSON
    Ok(response_text)
}

pub async fn download_file_with_spath(spath: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(spath).send().await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
