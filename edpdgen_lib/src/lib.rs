#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use crate::input::input_format::InputFormat;
use crate::output::output_format::OutputFormat;
use pls_core_extras::inflection_generator::{
    InflectionGenerator, NullInflectionGenerator, PlsInflectionGenerator,
};
use pls_core_extras::logger::PlsLogger;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod ajdict;
mod glib;
pub mod input;
pub mod output;
mod stardict;

pub struct DictionaryInfo<'a> {
    pub name: &'a str,
    pub input_data_path: &'a str,
    pub input_format: &'a InputFormat,
    pub output_format: &'a OutputFormat,
    pub output_folder: &'a str,
    pub short_name: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub links_color: &'a str,
    pub headings_color: &'a str,
    pub time_stamp: &'a str,
    pub icon_path: Option<&'a str>,
    pub icon: Vec<u8>,
    pub feedback_form_url: &'a str,
    pub host_url: &'a str,
    pub host_version: &'a str,
    pub inflections_db_path: Option<&'a str>,
}

pub struct DictionaryFile {
    pub extension: String,
    pub bom: Vec<u8>,
    pub data: Vec<u8>,
    pub can_be_empty: bool,
}

pub trait DictionaryBuilder<'a> {
    fn new(
        dict_info: &'a DictionaryInfo,
        input_data_path: &'a Path,
        igen: &'a dyn InflectionGenerator,
        logger: &'a dyn PlsLogger,
    ) -> Self;
    fn build_files(&self) -> Result<Vec<DictionaryFile>, String>;
}

pub fn run(dict_info: &DictionaryInfo, logger: &dyn PlsLogger) -> Result<(), String> {
    let igen: Box<dyn InflectionGenerator> =
        if let Some(inflections_db_path) = dict_info.inflections_db_path {
            Box::new(PlsInflectionGenerator::new(
                "en",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_NAME"),
                inflections_db_path,
                logger,
            )?)
        } else {
            Box::new(NullInflectionGenerator::new())
        };

    igen.check_inflection_db(logger)?;

    let input_data_path = Path::new(dict_info.input_data_path);
    let dict_files = match dict_info.output_format {
        OutputFormat::StarDict => {
            stardict::StarDict::new(dict_info, input_data_path, igen.as_ref(), logger)
                .build_files()?
        }
        OutputFormat::AjDict => {
            ajdict::AjDict::new(dict_info, input_data_path, igen.as_ref(), logger).build_files()?
        }
    };

    validate_dictionary_files(&dict_files, logger)?;

    let base_path = create_base_path(
        input_data_path,
        &dict_info.output_folder,
        &dict_info.short_name,
    )?;
    write_dictionary(&base_path, &dict_files, logger)
}

fn validate_dictionary_files(
    dict_files: &[DictionaryFile],
    logger: &dyn PlsLogger,
) -> Result<(), String> {
    for dict_file in dict_files {
        if dict_file.can_be_empty {
            continue;
        }

        if dict_file.data.is_empty() {
            let msg = format!(
                "'{}' file cannot be empty. Check that input csv has required columns and data.",
                &dict_file.extension,
            );
            logger.info(&msg);
            return Err(msg);
        }
    }

    Ok(())
}

fn create_base_path(
    input_data_path: &Path,
    output_folder: &str,
    dict_short_name: &str,
) -> Result<PathBuf, String> {
    let base_path = input_data_path
        .parent()
        .ok_or_else(|| format!("Unable to get parent folder for {:?}.", &input_data_path))?
        .join(output_folder)
        .join(dict_short_name);

    let parent_dir = base_path
        .parent()
        .ok_or_else(|| format!("Unable to get parent folder for {:?}.", &base_path))?;
    fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;

    Ok(base_path)
}

pub fn resolve_file_in_manifest_dir(file_name: &str) -> Result<PathBuf, String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let p1 = root.join(file_name);
    let file_path = if p1.exists() {
        p1
    } else {
        let p1 = root.parent().ok_or("")?;
        p1.join(file_name)
    };

    Ok(file_path)
}

fn write_dictionary(
    base_path: &Path,
    dict_files: &[DictionaryFile],
    logger: &dyn PlsLogger,
) -> Result<(), String> {
    for dict_file in dict_files {
        let f_name = base_path.with_extension(&dict_file.extension);
        logger.info(&format!("Writing {:?}.", &f_name));
        let mut f = File::create(&f_name).map_err(|e| e.to_string())?;
        if !&dict_file.bom.is_empty() {
            f.write_all(&dict_file.bom).map_err(|e| e.to_string())?;
        }
        f.write_all(&dict_file.data).map_err(|e| e.to_string())?;
        logger.info(&format!(
            "... done ({:.2} MB)...",
            dict_file.data.len() as f32 / 1024.0 / 1024.0
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pls_core_extras::logger::PlsLogger;

    pub struct TestLogger {}

    impl TestLogger {
        pub fn new() -> Self {
            TestLogger {}
        }
    }

    impl PlsLogger for TestLogger {
        fn info(&self, _msg: &str) {}
        fn error(&self, _msg: &str) {}
        fn warning(&self, _msg: &str) {}
    }

    pub struct TestInflectionGenerator {}

    impl TestInflectionGenerator {
        pub fn new() -> TestInflectionGenerator {
            TestInflectionGenerator {}
        }
    }

    impl InflectionGenerator for TestInflectionGenerator {
        fn check_inflection_db(&self, _logger: &dyn PlsLogger) -> Result<(), String> {
            Ok(())
        }

        fn generate_inflection_table_html(&self, pali1: &str) -> String {
            format!("[ITABLE: {}]", pali1)
        }

        fn generate_all_inflections(&self, pali1: &str) -> Vec<String> {
            vec![format!("{}_1", pali1), format!("{}_2", pali1)]
        }
    }
}
