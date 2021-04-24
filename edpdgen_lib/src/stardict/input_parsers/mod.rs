use crate::inflection_generator::InflectionGenerator;
use crate::{EdpdLogger, InputFormat};
use regex::{Captures, Regex};
use std::path::Path;

pub mod dpd;
pub mod dps;

lazy_static! {
    static ref PALI1_CRACKER: Regex = Regex::new(r"(.*)( )(\d+)$").expect("Malformed regex string");
}

pub trait PaliWord {
    fn id(&self) -> &str;
    fn sort_key(&self) -> String;
    fn group_id(&self) -> String;
    fn toc_id(&self, input_format: &InputFormat) -> String;
    fn toc_entry(&self, input_format: &InputFormat) -> Result<String, String>;
    fn word_data_entry(
        &self,
        input_format: &InputFormat,
        feedback_form_url: &str,
        host_url: &str,
        host_version: &str,
        igen: &dyn InflectionGenerator,
    ) -> Result<String, String>;
}

pub fn load_words<'a, T: 'a + serde::de::DeserializeOwned + PaliWord>(
    path: &Path,
    logger: &'a dyn EdpdLogger,
) -> Result<impl Iterator<Item = impl PaliWord> + 'a, String> {
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

fn make_sort_key(id: &str) -> String {
    let sk = PALI1_CRACKER.replace(id, |caps: &Captures| {
        // NOTE: Best case effort. Not sweating it.
        let n = &caps[3].parse::<i32>().unwrap_or(0);
        format!("{} {:03}", &caps[1], n)
    });

    sk.into_owned()
}

fn make_group_id(id: &str) -> String {
    let gid = PALI1_CRACKER.replace(id, |caps: &Captures| caps[1].to_string());

    gid.into_owned()
}

fn make_toc_id(id: &str, input_format: &InputFormat) -> String {
    format!("{}_{}", id.replace(" ", "_"), input_format.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_file_in_manifest_dir;
    use crate::stardict::input_parsers::dpd::DpdPaliWord;
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
            .map(|r| make_toc_id(r.id(), &InputFormat::DevamittaPaliStudy))
            .expect("unexpected");

        assert_eq!(toc_id, expected_toc_id);
    }
}
