

mod parsed_sites;

use std::error::Error;

use rust_scrap_israel_nutri::{maayan_2000::Maayan2000, LINK_KING_STORE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: preper directories
    prepare_directories().await?;
    // Get Data
    get_data(Maayan2000::<()>::URL).await?;
    // TODO: parse all the data into one big json or a database and saveit and delete the files after
    
    // TODO: analize the data and create a report create a ui based on this data using leptos
    Ok(())
}

async fn get_data(url: &str) -> Result<(), Box<dyn Error>> {

    match url {
        LINK_KING_STORE => {
            println!("King Store");
        }
        Maayan2000::<()>::URL => {
            Maayan2000::<()>::get_data_maayan_2000().await?;
        }
        _ => {
            println!("Unsupported URL: {}", url);
        }
    }

    Ok(())
}

async fn prepare_directories() -> Result<(), Box<dyn Error>> {
    Maayan2000::<()>::create_dir_all()?;

    Ok(())
}
