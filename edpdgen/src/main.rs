use chrono::Local;
use clap::{App, Arg};
use colored::*;
use edpdgen_lib::EdpdLogger;
use std::path::Path;

fn main() -> Result<(), String> {
    let l = ColoredConsoleLogger {};
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("CSV_FILE")
                .short("c")
                .long("csv")
                .value_name("CSV_FILE")
                .help("CSV with all words.")
                .required(true)
                .validator(|x| {
                    if Path::new(&x).is_file() {
                        Ok(())
                    } else {
                        Err(format!("{} does not exist.", x))
                    }
                })
                .takes_value(true),
        )
        .get_matches();

    let csv_path = matches
        .value_of("CSV_FILE")
        .ok_or(|| "This is a required argument")?;
    l.info(&format!("Using csv file: {}", csv_path));

    edpdgen_lib::run(Path::new(csv_path), &l)
}

fn get_time_stamp() -> String {
    Local::now().format("%y-%m-%d %H:%M:%S").to_string()
}

struct ColoredConsoleLogger;

impl EdpdLogger for ColoredConsoleLogger {
    fn info(&self, msg: &str) {
        println!(
            "{} {}",
            get_time_stamp().white(),
            format!("info: {}", msg).green(),
        );
    }

    fn error(&self, msg: &str) {
        println!(
            "{} {}",
            get_time_stamp().white(),
            format!("error: {}", msg).red(),
        );
    }
}
