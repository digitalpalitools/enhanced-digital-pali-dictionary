#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use crate::inflection_generator::{
    InflectionGenerator, NullInflectionGenerator, PlsInflectionGenerator,
};
use crate::stardict::input_parsers::dpd::DpdPaliWord;
use crate::stardict::input_parsers::dps::DpsPaliWord;
use crate::stardict::StarDictFile;
use crate::OutputFormat::{AnandaJyoti, GoldenDict};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod ajdict;
mod inflection_generator;
mod stardict;

pub trait EdpdLogger {
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
    fn warning(&self, msg: &str);
}

#[derive(Debug)]
pub enum OutputFormat {
    GoldenDict,
    AnandaJyoti,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "goldendict" => Ok(GoldenDict),
            "anandajyoti" => Ok(AnandaJyoti),
            _ => Err("Unknown format".to_string()),
        }
    }
}

pub struct DictionaryInfo<'a> {
    pub name: &'a str,
    pub ods_type: &'a str,
    pub output_format: OutputFormat,
    pub author: &'a str,
    pub description: &'a str,
    pub accent_color: &'a str,
    pub time_stamp: &'a str,
    pub ico: &'a [u8],
    pub feedback_form_url: &'a str,
    pub host_url: &'a str,
    pub host_version: &'a str,
    pub inflections_db_path: Option<&'a str>,
}

pub fn run(
    dict_info: &DictionaryInfo,
    csv_path: &Path,
    logger: &dyn EdpdLogger,
) -> Result<(), String> {
    let igen: Box<dyn InflectionGenerator> =
        if let Some(inflections_db_path) = dict_info.inflections_db_path {
            Box::new(PlsInflectionGenerator::new(inflections_db_path, logger)?)
        } else {
            Box::new(NullInflectionGenerator::new())
        };

    igen.check_inflection_db(logger)?;

    match dict_info.ods_type {
        "dpd" => {
            stardict::run_for_ods_type::<DpdPaliWord>(dict_info, csv_path, igen.as_ref(), logger)
        }
        "dps" => {
            stardict::run_for_ods_type::<DpsPaliWord>(dict_info, csv_path, igen.as_ref(), logger)
        }
        _ => unreachable!(),
    }
}

fn create_base_path(csv_path: &Path, ods_type: &str) -> Result<PathBuf, String> {
    let base_path = csv_path
        .parent()
        .ok_or_else(|| format!("Unable to get parent folder for {:?}.", &csv_path))?
        .join("dicts")
        .join(ods_type);

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
    sd_files: Vec<StarDictFile>,
    logger: &dyn EdpdLogger,
) -> Result<(), String> {
    for sd_file in sd_files {
        let f_name = base_path.with_extension(&sd_file.extension);
        logger.info(&format!("Writing {:?}.", &f_name));
        let mut f = File::create(&f_name).map_err(|e| e.to_string())?;
        f.write_all(&sd_file.data).map_err(|e| e.to_string())?;
        logger.info(&format!(
            "... done ({:.2} MB)...",
            sd_file.data.len() as f32 / 1024.0 / 1024.0
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestLogger {}

    impl TestLogger {
        pub fn new() -> Self {
            TestLogger {}
        }
    }

    impl EdpdLogger for TestLogger {
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
        fn check_inflection_db(&self, _logger: &dyn EdpdLogger) -> Result<(), String> {
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
