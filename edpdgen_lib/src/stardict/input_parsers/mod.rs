use crate::input::create_csv_reader;
use crate::input::PALI1_CRACKER;
use crate::stardict::StarDictPaliWord;
use pls_core_extras::logger::PlsLogger;
use regex::Captures;
use std::path::Path;

pub mod dpd;
pub mod dps;

pub fn load_words<'a, T: 'a + serde::de::DeserializeOwned + StarDictPaliWord>(
    path: &Path,
    logger: &'a dyn PlsLogger,
) -> Result<impl Iterator<Item = impl StarDictPaliWord> + 'a, String> {
    let rdr = create_csv_reader(path, logger)?;

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

fn make_group_id(id: &str) -> String {
    let gid = PALI1_CRACKER.replace(id, |caps: &Captures| caps[1].to_string());

    gid.into_owned()
}

fn make_toc_id(id: &str, dict_short_name: &str) -> String {
    format!("{}_{}", id.replace(" ", "_"), dict_short_name)
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

    #[test_case(4, "abahulīkata"; "0 digits")]
    #[test_case(5, "abala"; "1 digit")]
    #[test_case(11, "adhikāra"; "2 digits")]
    fn test_group_id(rec_number: usize, expected_gid: &str) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpdPaliWord>(&get_csv_path(), &l).expect("failed to load");

        let gid = recs
            .nth(rec_number)
            .map(|r| make_group_id(r.id()))
            .expect("unexpected");

        assert_eq!(gid, expected_gid);
    }

    #[test_case(4, "abahulīkata_dps"; "0 digits")]
    #[test_case(5, "abala_1_dps"; "1 digit")]
    #[test_case(11, "adhikāra_10_dps"; "2 digits")]
    fn test_toc_id(rec_number: usize, expected_toc_id: &str) {
        let l = TestLogger::new();
        let mut recs = load_words::<DpdPaliWord>(&get_csv_path(), &l).expect("unexpected");

        let toc_id = recs
            .nth(rec_number)
            .map(|r| make_toc_id(r.id(), "dps"))
            .expect("unexpected");

        assert_eq!(toc_id, expected_toc_id);
    }
}
