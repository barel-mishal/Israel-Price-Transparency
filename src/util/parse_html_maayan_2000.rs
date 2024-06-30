use std::error::Error;

use scraper::{Html, Selector};

/// Parses the HTML content and extracts file information.
/// Returns a `Result` with a vector of `FileInfo` or an error.
pub fn parse_html(body: &str) -> Result<Vec<crate::maayan_2000::FileInfo>, Box<dyn Error>> {
    println!("Parsing Maayan 2000");
    let document = Html::parse_document(body);
    let selector = Selector::parse("#myTable tbody tr").expect("Error while parsing");

    let mut file_infos = Vec::new();

    let data = document.select(&selector).enumerate();

    for (index, element) in data {
        let columns: Vec<_> = element.select(&Selector::parse("td").unwrap()).collect();
        println!("Columns found: {}", columns.len());


        if columns.len() == 6 {
            let file_name = columns[0].inner_html().trim().to_string();
            let branch = columns[1].inner_html().trim().to_string();
            let file_type = columns[2].inner_html().trim().to_string();
            let extension = columns[3].inner_html().trim().to_string();
            let date = columns[4].inner_html().trim().to_string();
            let button_id = format!("button{}", index);

            let file_info = crate::maayan_2000::FileInfo::new(
                &file_name, &branch, &file_type, &extension, &date, &button_id
            );

            file_infos.push(file_info);
        }
    }

    Ok(file_infos)
}