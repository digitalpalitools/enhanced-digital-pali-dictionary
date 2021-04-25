use crate::ajdict::AjDictPaliWord;
use crate::input::create_csv_reader;
use crate::EdpdLogger;
use std::path::Path;

pub mod dpd;

pub fn load_words<'a, T: 'a + serde::de::DeserializeOwned + AjDictPaliWord>(
    path: &Path,
    logger: &'a dyn EdpdLogger,
) -> Result<impl Iterator<Item = impl AjDictPaliWord> + 'a, String> {
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
