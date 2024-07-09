use polars::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut file = std::fs::File::open("docs/data/path.json").unwrap();
    let df = JsonReader::new(&mut file).finish().unwrap();
}
