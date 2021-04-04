#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use crate::input_parsers::dpd::DpdPaliWord;
use crate::input_parsers::dps::DpsPaliWord;
use crate::input_parsers::PaliWord;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod glib;
mod input_parsers;
mod output_generators;

pub trait EdpdLogger {
    fn info(&self, msg: &str);
    fn error(&self, error: &str);
}

pub struct StarDictFile {
    extension: String,
    data: Vec<u8>,
}

pub struct StartDictInfo<'a> {
    pub name: &'a str,
    pub short_name: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub accent_color: &'a str,
    pub time_stamp: &'a str,
    pub ico: &'a [u8],
    pub feedback_form_url: &'a str,
    pub host_url: &'a str,
    pub host_version: &'a str,
}

pub fn run(
    dict_info: &StartDictInfo,
    csv_path: &Path,
    logger: &dyn EdpdLogger,
) -> Result<(), String> {
    match dict_info.short_name {
        "dpd" => run_for_ods_type::<DpdPaliWord>(dict_info, csv_path, logger),
        "dps" => run_for_ods_type::<DpsPaliWord>(dict_info, csv_path, logger),
        _ => unreachable!(),
    }
}

fn run_for_ods_type<'a, T: 'a + serde::de::DeserializeOwned + PaliWord>(
    dict_info: &StartDictInfo,
    csv_path: &Path,
    logger: &dyn EdpdLogger,
) -> Result<(), String> {
    let words = input_parsers::load_words::<T>(csv_path, logger)?;
    let sd_files = output_generators::create_dictionary(&dict_info, words, logger)?;

    let base_path = create_base_path(csv_path, dict_info.short_name)?;
    write_dictionary(&base_path, sd_files, logger)
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
    }
}
