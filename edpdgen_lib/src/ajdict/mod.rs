use crate::input::dpd::DpdPaliWord;
use crate::input::input_format::InputFormat;
use crate::{DictionaryBuilder, DictionaryFile, DictionaryInfo};
use pls_core_extras::inflection_generator::InflectionGenerator;
use pls_core_extras::logger::PlsLogger;
use std::path::Path;

mod input_parsers;
mod output_generators;

pub struct AjDict<'a> {
    dict_info: &'a DictionaryInfo<'a>,
    input_data_path: &'a Path,
    logger: &'a dyn PlsLogger,
}

impl<'a> DictionaryBuilder<'a> for AjDict<'a> {
    fn new(
        dict_info: &'a DictionaryInfo,
        input_data_path: &'a Path,
        _igen: &'a dyn InflectionGenerator,
        logger: &'a dyn PlsLogger,
    ) -> Self {
        AjDict {
            dict_info,
            input_data_path,
            logger,
        }
    }

    fn build_files(&self) -> Result<Vec<DictionaryFile>, String> {
        match self.dict_info.input_format {
            InputFormat::Dpd => run_for_ods_type::<DpdPaliWord>(
                self.input_data_path,
                self.logger,
                self.dict_info.concise,
            ),
            InputFormat::Dps => {
                let msg = format!(
                    "Dictionary format '{}' has not yet been implemented for '{}'.",
                    self.dict_info.output_format, self.dict_info.short_name
                );
                self.logger.error(&msg);
                Err(msg)
            }
        }
    }
}

pub trait AjDictPaliWord {
    fn id(&self) -> &str;
    fn sort_key(&self) -> String;
    fn concise_word_data_entry(&self) -> Result<String, String>;
    fn word_data_entry(&self) -> Result<String, String>;
}

pub fn run_for_ods_type<'a, T: 'a + serde::de::DeserializeOwned + AjDictPaliWord>(
    input_data_path: &Path,
    logger: &dyn PlsLogger,
    concise: bool,
) -> Result<Vec<DictionaryFile>, String> {
    let words = input_parsers::load_words::<T>(input_data_path, logger)?;
    let sd_files = output_generators::create_dictionary(words, logger, concise)?;

    Ok(sd_files)
}
