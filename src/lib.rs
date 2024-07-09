mod structs;
mod util;

pub use structs::maayan_2000;
pub use util::{parse_html_maayan_2000, save_json, setup_browser, create_json_file, get_req_maayan_2000, parse_compress_files};
pub const DIR_ROOT_DATA: &str = "data";

pub const LINK_KING_STORE: &str = "https://kingstore.binaprojects.com/Main.aspx";
pub const DIR_KING_STORE: &str = "data/king_store";