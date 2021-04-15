use chrono::{Local, SecondsFormat, Utc};
use clap::{App, Arg, ArgMatches};
use colored::*;
use edpdgen_lib::{EdpdLogger, StartDictInfo};
use std::path::Path;

fn main() -> Result<(), String> {
    let l = ColoredConsoleLogger {};
    let matches = get_args();

    let csv_path = matches
        .value_of("CSV_FILE")
        .ok_or_else(|| "This is a required argument".to_string())?;
    let ods_type = matches
        .value_of("ODS_TYPE")
        .ok_or_else(|| "This is a required argument".to_string())?;
    let gen_inflections = matches.is_present("GENERATE_INFLECTION");

    l.info(&format!(
        "Using csv file: {} for ods type {}. Will {}generate inflections.",
        csv_path,
        ods_type,
        if gen_inflections { "" } else { "NOT " }
    ));
    let time_stamp = &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    edpdgen_lib::run(
        &get_stardict_info_from_ods_type(ods_type, time_stamp, gen_inflections),
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
            Arg::with_name("ODS_TYPE")
                .short("t")
                .long("type")
                .value_name("ODS_TYPE")
                .help("Type of ODS.")
                .required(true)
                .possible_values(&["dpd", "dps"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("GENERATE_INFLECTION")
                .short("i")
                .long("inflection")
                .help("Generate inflection tables and syn file."),
        )
        .get_matches()
}

fn get_stardict_info_from_ods_type<'a>(
    ods_type: &'a str,
    time_stamp: &'a str,
    gen_inflections: bool,
) -> StartDictInfo<'a> {
    let host_url = env!("CARGO_PKG_NAME");
    let host_version = env!("CARGO_PKG_VERSION");
    let short_name = ods_type;

    match short_name {
        "dpd" => {
            StartDictInfo {
                name: "Digital Pāli Tools Dictionary (DPD)",
                short_name,
                author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
                description: "The next generation comprehensive digital Pāli dictionary.",
                accent_color: "orange",
                time_stamp,
                ico: include_bytes!("dpd.png"),
                feedback_form_url:
                    "https://docs.google.com/forms/d/1hMra0aMz65sYnRlPjGlTYQIHz-3_tKlywu3enqXlpSc/viewform",
                host_url,
                host_version,
                generate_inflections: gen_inflections,
            }
        }
        "dps" => {
            StartDictInfo {
                name: "Devamitta Pāli Study (DPS)",
                short_name,
                author: "Devamitta Bhikkhu",
                description: "A detailed Pāli language word lookup.",
                accent_color: "green",
                time_stamp,
                ico: include_bytes!("dps.png"),
                feedback_form_url:
                    "https://docs.google.com/forms/d/e/1FAIpQLSc87oKqninpyg01YWdsjdYK6wSeIMoAZpy2jNM7Wu0KYygnHw/viewform",
                host_url,
                host_version,
                generate_inflections: gen_inflections,
            }
        }
        _ => unreachable!()
    }
}
