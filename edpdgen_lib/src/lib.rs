#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod input_parsers;

use crate::input_parsers::PaliWord;

fn get_csv_path() -> String {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("Pali_English_Dictionary_10_rows-full.csv");

    d.to_str().unwrap_or("Not able to resolve path!").to_owned()
}

pub fn run() -> Result<(), String> {
    let mut recs = input_parsers::read_records(&get_csv_path())?;

    let sk = recs
        .nth(11)
        .expect("unexpected")
        .map(|r| r.sort_key())
        .expect("unexpected");

    println!("!>>> {}", sk);

    Ok(())
}
