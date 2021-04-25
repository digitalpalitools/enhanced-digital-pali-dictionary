use crate::ajdict::AjDictPaliWord;
use crate::EdpdLogger;
use regex::{Captures, Regex};
use std::path::Path;

pub mod dpd;

lazy_static! {
    static ref PALI1_CRACKER: Regex = Regex::new(r"(.*)( )(\d+)$").expect("Malformed regex string");
}

//
//
//
//
//  DUP code
//
//
//
//
//
//
pub fn load_words<'a, T: 'a + serde::de::DeserializeOwned + AjDictPaliWord>(
    path: &Path,
    logger: &'a dyn EdpdLogger,
) -> Result<impl Iterator<Item = impl AjDictPaliWord> + 'a, String> {
    logger.info(&format!("Loading words from {:?}.", path));

    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .from_reader(file);

    let words = rdr
        .into_deserialize::<T>()
        .enumerate()
        .filter_map(move |(i, r)| match r {
            Ok(w) => Some(w),
            Err(e) => {
                logger.error(&format!(
                    "Unable to deserialize record #{}. Error: {}.",
                    i, e
                ));
                None
            }
        });

    logger.info(&format!("... done loading words from {:?}.", &path));
    Ok(words)
}

//
//
//
//
//  DUP code
//
//
//
//
//
//
fn make_sort_key(id: &str) -> String {
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
    use crate::input::dpd::DpdPaliWord;
    use crate::resolve_file_in_manifest_dir;
    use crate::tests::TestLogger;
    use std::path::PathBuf;
    use test_case::test_case;

    pub fn get_csv_path() -> PathBuf {
        resolve_file_in_manifest_dir("Pali_English_Dictionary_10_rows-full.csv")
            .expect("must exist!")
    }

    #[test_case(0, "ābādha"; "0 digits")]
    #[test_case(9, "adhikāra 001"; "1 digit")]
    #[test_case(11, "adhikāra 010"; "2 digits")]
    fn test_sort_key(rec_number: usize, expected_sk: &str) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpdPaliWord>(&get_csv_path(), &l).expect("failed to load");

        let sk = recs
            .nth(rec_number)
            .map(|r| make_sort_key(r.id()))
            .expect("unexpected");

        assert_eq!(sk, expected_sk)
    }
}
