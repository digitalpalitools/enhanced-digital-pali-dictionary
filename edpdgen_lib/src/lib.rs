#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Write;

mod input_parsers;
mod output_generators;

pub trait EdpdLogger {
    fn info(&self, msg: &str);
    fn error(&self, error: &str);
}

pub fn run(csv_path: &str, logger: &impl EdpdLogger) -> Result<(), String> {
    let words = input_parsers::read_words(csv_path, logger)?;
    let (dict, idx) = output_generators::write_dictionary("dpd", words, logger)?;

    let mut dict_file = File::create("d:/delme/dicts-rust/a.dict").map_err(|e| e.to_string())?;
    dict_file.write_all(&dict).map_err(|e| e.to_string())?;
    logger.info(&format!(">>> {}", dict.len()));

    let mut idx_file = File::create("d:/delme/dicts-rust/a.idx").map_err(|e| e.to_string())?;
    logger.info(&format!(">>> {}", idx.len()));
    idx_file.write_all(&idx).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn resolve_file_in_manifest_dir(file_name: &str) -> Result<String, String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let p1 = root.join(file_name);
    let file_path = if p1.exists() {
        p1.to_str().map(|x| x.to_owned())
    } else {
        let p1 = root.parent().ok_or("")?;
        p1.join(file_name).to_str().map(|x| x.to_owned())
    };

    Ok(file_path.ok_or_else(|| format!("Unable to resolve {}", file_name))?)
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
