pub mod dpd;
pub mod dps;
pub mod input_format;
use csv::Reader;
use pls_core_extras::logger::PlsLogger;
use regex::{Captures, Regex};
use std::fs::File;
use std::path::Path;

lazy_static! {
    pub static ref PALI1_CRACKER: Regex =
        Regex::new(r"(.*)( )(\d+)$").expect("Malformed regex string");
}

pub fn create_csv_reader(path: &Path, logger: &dyn PlsLogger) -> Result<Reader<File>, String> {
    logger.info(&format!("Loading words from {:?}.", path));

    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .from_reader(file);

    Ok(rdr)
}

pub fn make_sort_key(id: &str) -> String {
    let sk = PALI1_CRACKER.replace(id, |caps: &Captures| {
        // NOTE: Best case effort. Not sweating it.
        let n = &caps[3].parse::<i32>().unwrap_or(0);
        format!("{} {:03}", &caps[1], n)
    });

    sk.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ābādha", "ābādha"; "0 digits")]
    #[test_case("adhikāra 1", "adhikāra 001"; "1 digit")]
    #[test_case("adhikāra 10", "adhikāra 010"; "2 digits")]
    fn test_sort_key(id: &str, expected_sk: &str) {
        let sk = make_sort_key(id);

        assert_eq!(sk, expected_sk)
    }
}
