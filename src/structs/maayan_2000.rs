#![allow(non_snake_case)]
use std::{error::Error, fs::create_dir_all, path::Path};

use serde::{Deserialize, Serialize};

use crate::{get_req_maayan_2000::{download_file_with_spath, download_maayan_2000_file}, parse_compress_files::{parse_and_save_xml_as_json, parse_gzip_file, parse_zip_file}, save_json::save_json, setup_browser::setup_browser, DIR_ROOT_DATA};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub file_name: String,
    branch: String,
    file_type: String,
    extension: String,
    date: String,
    button_id: String,
}

impl FileInfo {
    pub fn new(file_name: &str, branch: &str, file_type: &str, extension: &str, date: &str, button_id: &str) -> Self {
        FileInfo {
            file_name: file_name.to_string(),
            branch: branch.to_string(),
            file_type: file_type.to_string(),
            extension: extension.to_string(),
            date: date.to_string(),
            button_id: button_id.to_string(),
        }
    }
}

impl<T> Maayan2000<T> {
    pub const PATH: &'static str = "data/Maayan2000";
    pub const URL: &'static str = "https://maayan2000.binaprojects.com/Main.aspx";

    pub fn create_dir_all() -> Result<(), Box<dyn Error>> {
        if !Path::new(Self::PATH).exists() {
            create_dir_all(Self::PATH)?;
        }
        Ok(())
    }

    pub async fn get_data_maayan_2000() -> Result<(), Box<dyn Error>> {
        println!("Fetching Maayan 2000");
    
        // Set up the browser options with the custom download path
        let browser = setup_browser(&Maayan2000::<()>::PATH)?;
        let tab = browser.new_tab()?;
        tab.navigate_to(Maayan2000::<()>::URL)?;
        tab.wait_until_navigated()?;
    
        let html = tab.get_content()?;
        let file_infos = crate::parse_html_maayan_2000::parse_html(&html)?;
    
        save_json(&file_infos, DIR_ROOT_DATA)?;
    
        for file_info in &file_infos {
            Self::download_and_parse_file(Maayan2000::<()>::URL, Maayan2000::<()>::PATH, file_info).await?;
        }
    
        Ok(())
    }

    async fn download_and_parse_file(base_url: &str, download_dir: &str, file_info: &FileInfo) -> Result<(), Box<dyn Error>> {
        let response_text = download_maayan_2000_file(base_url, file_info).await?;
        // Try to parse the response as JSON
        let data: Vec<SPathResponse> = serde_json::from_str(&response_text)?;
        println!("Data: {:?}", data);
    
        match data.first() {
            Some(SPathResponse { SPath }) => {
                Self::parse_response_file(SPath, download_dir, file_info).await?; 
            }
            None => {
                println!("No SPath found in response");
            },
        }
        Ok(())
    }

    async fn parse_response_file(spath: &str, download_dir: &str, file_info: &FileInfo) -> Result<(), Box<dyn Error>> {
        let bytes = download_file_with_spath(spath).await?;
        let file_path = format!("{}/{}", download_dir, file_info.file_name);
    
        if let Some(parent) = Path::new(&file_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let header_bytes = &bytes[..std::cmp::min(10, bytes.len())];
        
        let is_xml = header_bytes.starts_with(b"<?xml");
        let is_gzip = header_bytes.starts_with(&[0x1F, 0x8B]);
        let is_zip = header_bytes.starts_with(b"PK\x03\x04");
        
        println!(r#"
        2. File header bytes: {:?}, is_xml: {}, is_gzip: {}, is_zip: {}
        "#, header_bytes, is_xml, is_gzip, is_zip);
    
        if is_xml {
            let contents = std::str::from_utf8(&bytes)?;
            parse_and_save_xml_as_json(&file_info.file_name, download_dir, contents)?;
        } else if is_gzip {
            parse_gzip_file(&file_info, download_dir, &bytes)?;
        } else if is_zip {
            parse_zip_file(download_dir, &bytes)?;
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported file format")));
        }
    
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Maayan2000<T> {
    ChainId: String,
    SubChainId: String,
    StoreId: String,
    BikoretNo: String,
    data: T,
}

impl Maayan2000<Promotions> {
    pub fn new(chain_id: &str, sub_chain_id: &str, store_id: &str, bikoret_no: &str, promotions: Promotions) -> Self {
        Maayan2000 {
            ChainId: chain_id.to_string(),
            SubChainId: sub_chain_id.to_string(),
            StoreId: store_id.to_string(),
            BikoretNo: bikoret_no.to_string(),
            data: promotions,
        }
    }
}

impl Maayan2000<Items> {
    pub fn new(chain_id: &str, sub_chain_id: &str, store_id: &str, bikoret_no: &str, items: Items) -> Self {
        Maayan2000 {
            ChainId: chain_id.to_string(),
            SubChainId: sub_chain_id.to_string(),
            StoreId: store_id.to_string(),
            BikoretNo: bikoret_no.to_string(),
            data: items,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Promotions {
    #[serde(rename = "Promotion")]
    promotions: Vec<Promotion>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Promotion {
    PromotionId: String,
    PromotionDescription: String,
    PromotionUpdateDate: String,
    PromotionStartDate: String,
    PromotionStartHour: String,
    PromotionEndDate: String,
    PromotionEndHour: String,
    RewardType: String,
    DiscountType: String,
    DiscountRate: String,
    AllowMultipleDiscounts: String,
    MinQty: String,
    MaxQty: String,
    DiscountedPrice: String,
    DiscountedPricePerMida: String,
    MinNoOfItemOfered: String,
    AdditionalRestrictions: AdditionalRestrictions,
    PromotionItems: PromotionItems,
    GiftsItems: GiftsItems,
    Remarks: String,
    MinPurchaseAmnt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalRestrictions {
    AdditionalIsCoupon: String,
    AdditionalGiftCount: String,
    Clubs: Clubs,
    AdditionalIsTotal: String,
    AdditionalIsActive: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Clubs {
    ClubId: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionItems {
    #[serde(rename = "Item")]
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GiftsItems {
    #[serde(rename = "Item", default)]
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    ItemCode: String,
    IsGiftItem: String,
    ItemType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SPathResponse {
    pub SPath: String,
}

impl SPathResponse {
    pub fn new(spath: &str) -> Self {
        SPathResponse {
            SPath: spath.to_string(),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Items {
    #[serde(rename = "Item")]
    items: Vec<ItemPrice>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ItemPrice {
    PriceUpdateDate: String,
    ItemCode: String,
    ItemType: String,
    ItemNm: String,
    ManufacturerName: String,
    ManufactureCountry: String,
    ManufacturerItemDescription: String,
    UnitQty: String,
    Quantity: String,
    UnitOfMeasure: String,
    bIsWeighted: Option<String>,
    QtyInPackage: String,
    ItemPrice: String,
    UnitOfMeasurePrice: String,
    AllowDiscount: String,
    ItemStatus: String,
}