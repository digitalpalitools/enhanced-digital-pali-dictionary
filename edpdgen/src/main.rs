use chrono::{Local, SecondsFormat, Utc};
use clap::{App, Arg, ArgMatches};
use colored::*;
use edpdgen_lib::input::input_format::InputFormat;
use edpdgen_lib::output::output_format::OutputFormat;
use edpdgen_lib::{DictionaryInfo, EdpdLogger};
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<(), String> {
    let l = ColoredConsoleLogger {};
    let matches = get_args();

    let csv_path = matches
        .value_of("CSV_FILE")
        .expect("This is a required argument");
    let input_format = InputFormat::from_str(
        matches
            .value_of("INPUT_FORMAT")
            .expect("This is a required argument"),
    )
    .expect("Invalid cases should have been reject by clapp");
    let output_format = OutputFormat::from_str(
        matches
            .value_of("OUTPUT_FORMAT")
            .expect("This is a required argument"),
    )
    .expect("Invalid cases should have been reject by clapp");
    let inflections_db_path = matches.value_of("INFLECTION_DB_PATH");

    l.info(&format!(
        "Using csv file: {} for ods type {:?}. Target dictionary format: {:?}. {}.",
        csv_path,
        input_format,
        output_format,
        if let Some(idb) = inflections_db_path {
            format!("Will generated inflections. Using db: {}", idb)
        } else {
            "Will NOT generate inflections".to_string()
        }
    ));
    let time_stamp = &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    edpdgen_lib::run(
        &get_dictionary_info_for_input_format(
            &input_format,
            &output_format,
            time_stamp,
            inflections_db_path,
        ),
        Path::new(csv_path),
        &l,
    )
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

    fn warning(&self, msg: &str) {
        println!(
            "{} {}",
            get_time_stamp().white(),
            format!("warning: {}", msg).yellow(),
        );
    }
}

fn get_args<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
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
        .arg(
            Arg::with_name("INPUT_FORMAT")
                .short("t")
                .long("type")
                .value_name("INPUT_FORMAT")
                .help("Input data format.")
                .required(true)
                .possible_values(&["dpd", "dps"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_FORMAT")
                .short("f")
                .long("format")
                .value_name("OUTPUT_FORMAT")
                .help("Target dictionary format.")
                .required(true)
                .possible_values(&["stardict", "ajdict"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INFLECTION_DB_PATH")
                .short("i")
                .long("inflection-db")
                .value_name("INFLECTION_DB_PATH")
                .help("The path to inflections.db.")
                .required(false)
                .validator(|x| {
                    if Path::new(&x).is_file() {
                        Ok(())
                    } else {
                        Err(format!("{} does not exist.", x))
                    }
                })
                .takes_value(true),
        )
        .get_matches()
}

fn get_dictionary_info_for_input_format<'a>(
    input_format: &'a InputFormat,
    output_format: &'a OutputFormat,
    time_stamp: &'a str,
    inflections_db_path: Option<&'a str>,
) -> DictionaryInfo<'a> {
    let host_url = env!("CARGO_PKG_NAME");
    let host_version = env!("CARGO_PKG_VERSION");

    match input_format {
        InputFormat::DigitalPaliDictionary => {
            DictionaryInfo {
                name: "Digital Pāli Tools Dictionary (DPD)",
                input_format,
                output_format,
                author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
                description: "The next generation comprehensive digital Pāli dictionary.",
                accent_color: "#7986CB",
                time_stamp,
                ico: include_bytes!("dpd.png"),
                feedback_form_url:
                    "https://docs.google.com/forms/d/1hMra0aMz65sYnRlPjGlTYQIHz-3_tKlywu3enqXlpSc/viewform",
                host_url,
                host_version,
                inflections_db_path,
            }
        }
        InputFormat::DevamittaPaliStudy => {
            DictionaryInfo {
                name: "Devamitta Pāli Study (DPS)",
                input_format,
                output_format,
                author: "Devamitta Bhikkhu",
                description: "A detailed Pāli language word lookup.",
                accent_color: "green",
                time_stamp,
                ico: include_bytes!("dps.png"),
                feedback_form_url:
                    "https://docs.google.com/forms/d/e/1FAIpQLSc87oKqninpyg01YWdsjdYK6wSeIMoAZpy2jNM7Wu0KYygnHw/viewform",
                host_url,
                host_version,
                inflections_db_path,
            }
        }
    }
}
