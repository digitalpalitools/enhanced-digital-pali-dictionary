use crate::inflection_generator::InflectionGenerator;
use crate::input::dpd::DpdPaliWord;
use crate::input::dps::DpsPaliWord;
use crate::input::input_format::InputFormat;
use crate::{DictionaryBuilder, DictionaryFile, DictionaryInfo, EdpdLogger};
use std::path::Path;

mod input_parsers;
mod output_generators;

pub struct StarDict<'a> {
    dict_info: &'a DictionaryInfo<'a>,
    input_data_path: &'a Path,
    igen: &'a dyn InflectionGenerator,
    logger: &'a dyn EdpdLogger,
}

impl<'a> DictionaryBuilder<'a> for StarDict<'a> {
    fn new(
        dict_info: &'a DictionaryInfo,
        input_data_path: &'a Path,
        igen: &'a dyn InflectionGenerator,
        logger: &'a dyn EdpdLogger,
    ) -> Self {
        StarDict {
            dict_info,
            input_data_path,
            igen,
            logger,
        }
    }

    fn build_files(&self) -> Result<Vec<DictionaryFile>, String> {
        match self.dict_info.input_format {
            InputFormat::Dpd => run_for_ods_type::<DpdPaliWord>(
                self.dict_info,
                self.input_data_path,
                self.igen,
                self.logger,
            ),
            InputFormat::Dps => run_for_ods_type::<DpsPaliWord>(
                self.dict_info,
                self.input_data_path,
                self.igen,
                self.logger,
            ),
        }
    }
}

pub trait StarDictPaliWord {
    fn id(&self) -> &str;
    fn sort_key(&self) -> String;
    fn group_id(&self) -> String;
    fn toc_id(&self, dict_short_name: &str) -> String;
    fn toc_entry(&self, dict_short_name: &str) -> Result<String, String>;
    fn word_data_entry(
        &self,
        dict_short_name: &str,
        feedback_form_url: &str,
        host_url: &str,
        host_version: &str,
        igen: &dyn InflectionGenerator,
    ) -> Result<String, String>;
}

pub fn run_for_ods_type<'a, T: 'a + serde::de::DeserializeOwned + StarDictPaliWord>(
    dict_info: &DictionaryInfo,
    input_data_path: &Path,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<Vec<DictionaryFile>, String> {
    let words = input_parsers::load_words::<T>(input_data_path, logger)?;
    let sd_files = output_generators::create_dictionary(&dict_info, words, igen, logger)?;

    Ok(sd_files)
}
