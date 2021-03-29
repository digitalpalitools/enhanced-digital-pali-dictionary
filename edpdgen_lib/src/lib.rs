#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use chrono::{SecondsFormat, Utc};
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
    name: &'a str,
    short_name: &'a str,
    author: &'a str,
    description: &'a str,
    accent_color: &'a str,
    time_stamp: &'a str,
    ico: &'a [u8],
}

pub fn run(csv_path: &Path, logger: &impl EdpdLogger) -> Result<(), String> {
    let dict_info = StartDictInfo {
        name: "Digital Pāli Tools Dictionary (DPD)",
        short_name: "dpd",
        author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
        description: "The next generation comprehensive digital Pāli dictionary.",
        accent_color: "orange",
        time_stamp: &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        ico: include_bytes!("dpd.png"),
    };

    let words = input_parsers::load_words(csv_path, logger)?;
    let sd_files = output_generators::create_dictionary(&dict_info, words, logger)?;

    let base_path = create_base_path(csv_path, "dpd")?;
    write_dictionary(&base_path, sd_files, logger)
}

fn create_base_path(csv_path: &Path, ods_type: &str) -> Result<PathBuf, String> {
    let base_path = csv_path
        .parent()
        .ok_or_else(|| format!("Unable to get parent folder for {:?}.", &csv_path))?
        .join("dict")
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
    logger: &impl EdpdLogger,
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
