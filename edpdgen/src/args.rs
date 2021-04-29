use clap::{App, Arg, ArgMatches};
use edpdgen_lib::input::input_format::InputFormat;
use edpdgen_lib::output::output_format::OutputFormat;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub(crate) struct EdpdArgs<'a> {
    pub csv_path: &'a str,
    pub input_format: InputFormat,
    pub output_format: OutputFormat,
    pub output_folder: Option<&'a str>,
    pub name: Option<&'a str>,
    pub short_name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub links_color: Option<&'a str>,
    pub headings_color: Option<&'a str>,
    pub icon_path: Option<&'a str>,
    pub inflections_db_path: Option<&'a str>,
    pub what_if: bool,
}

pub(crate) fn get_args<'a>(args: &'a ArgMatches) -> EdpdArgs<'a> {
    EdpdArgs {
        csv_path: args
            .value_of("CSV_FILE")
            .expect("This is a required argument"),
        input_format: InputFormat::from_str(
            args.value_of("INPUT_FORMAT")
                .expect("This is a required argument"),
        )
        .expect("Invalid cases should have been reject by clapp"),
        output_format: OutputFormat::from_str(
            args.value_of("OUTPUT_FORMAT")
                .expect("This is a required argument"),
        )
        .expect("Invalid cases should have been reject by clapp"),
        output_folder: args.value_of("OUTPUT_FOLDER"),
        name: args.value_of("NAME"),
        short_name: args.value_of("SHORT_NAME"),
        description: args.value_of("DESCRIPTION"),
        links_color: args.value_of("LINKS_COLOR"),
        headings_color: args.value_of("HEADINGS_COLOR"),
        icon_path: args.value_of("ICON_PATH"),
        inflections_db_path: args.value_of("INFLECTION_DB_PATH"),
        what_if: args.is_present("WHAT_IF"),
    }
}

pub(crate) fn parse_args<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(create_csv_file_arg())
        .arg(create_input_format_arg())
        .arg(create_output_format_arg())
        .arg(create_output_folder_arg())
        .arg(create_name_arg())
        .arg(create_short_name_arg())
        .arg(create_description_arg())
        .arg(create_links_color_arg())
        .arg(create_headings_color_arg())
        .arg(create_icon_path_arg())
        .arg(create_inflection_db_path_arg())
        .arg(create_what_if_arg())
        .get_matches()
}

fn create_output_folder_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("OUTPUT_FOLDER")
        .short("o")
        .long("output-folder")
        .value_name("OUTPUT_FOLDER")
        .help("The output directory (will be created if it does not exist).")
        .required(false)
        .takes_value(true)
}

fn create_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("NAME")
        .short("n")
        .long("name")
        .value_name("NAME")
        .help("The name of the dictionary (64 characters max.).")
        .required(false)
        .validator(validate_name)
        .takes_value(true)
}

fn create_short_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("SHORT_NAME")
        .short("s")
        .long("short-name")
        .value_name("SHORT_NAME")
        .help("The short name of the dictionary (10 alphanumeric characters max. starting with alphabet).")
        .required(false)
        .validator(validate_short_name)
        .takes_value(true)
}

fn create_description_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("DESCRIPTION")
        .short("d")
        .long("description")
        .value_name("DESCRIPTION")
        .help("The description of the dictionary (256 characters max.).")
        .required(false)
        .validator(validate_description)
        .takes_value(true)
}

fn create_links_color_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("LINKS_COLOR")
        .short("l")
        .long("links-color")
        .value_name("LINKS_COLOR")
        .help("The color of the links (#XXXXXX, html rgb format).")
        .validator(validate_html_color)
        .required(false)
        .takes_value(true)
}

fn create_headings_color_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("HEADINGS_COLOR")
        .short("h")
        .long("headings-color")
        .value_name("HEADINGS_COLOR")
        .help("The color of the headings.")
        .required(false)
        .validator(validate_html_color)
        .takes_value(true)
}

fn create_icon_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ICON_PATH")
        .short("p")
        .long("icon-path")
        .value_name("ICON_PATH")
        .help(
            "The path to the dictionary icon (png, 4KB max, use https://favicon.io/ to generate).",
        )
        .required(false)
        .validator(validate_icon_file)
        .takes_value(true)
}

fn create_what_if_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("WHAT_IF")
        .short("w")
        .long("what-if")
        .help("Print everything but dont generate.")
}

fn create_inflection_db_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("INFLECTION_DB_PATH")
        .short("i")
        .long("inflection-db")
        .value_name("INFLECTION_DB_PATH")
        .help("The path to inflections.db.")
        .required(false)
        .validator(|s| validate_file_exists(&s))
        .takes_value(true)
}

fn create_output_format_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("OUTPUT_FORMAT")
        .short("f")
        .long("format")
        .value_name("OUTPUT_FORMAT")
        .help("Target dictionary format.")
        .required(true)
        .possible_values(&["stardict", "ajdict"])
        .takes_value(true)
}

fn create_input_format_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("INPUT_FORMAT")
        .short("t")
        .long("type")
        .value_name("INPUT_FORMAT")
        .help("Input data format.")
        .required(true)
        .possible_values(&["dpd", "dps"])
        .takes_value(true)
}

fn create_csv_file_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("CSV_FILE")
        .short("c")
        .long("csv")
        .value_name("CSV_FILE")
        .help("CSV with all words.")
        .required(true)
        .validator(|s| validate_file_exists(&s))
        .takes_value(true)
}

fn validate_html_color(s: String) -> Result<(), String> {
    let re = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").expect("is valid regex");
    if re.is_match(&s) {
        Ok(())
    } else {
        Err(format!(
            "'{}' has invalid format, run --help to see format.",
            s
        ))
    }
}

fn validate_description(s: String) -> Result<(), String> {
    if s.len() < 257 {
        Ok(())
    } else {
        Err(format!(
            "'{}' has invalid format, run --help to see format.",
            s
        ))
    }
}

fn validate_short_name(s: String) -> Result<(), String> {
    let re = Regex::new(r"^[a-z]\w{2,9}$").expect("is valid regex");
    if re.is_match(&s) {
        Ok(())
    } else {
        Err(format!(
            "'{}' has invalid format, run --help to see format.",
            s
        ))
    }
}

fn validate_name(s: String) -> Result<(), String> {
    if s.len() < 65 {
        Ok(())
    } else {
        Err(format!(
            "'{}' has invalid format, run --help to see format.",
            s
        ))
    }
}

fn validate_icon_file(s: String) -> Result<(), String> {
    validate_file_exists(&s)?;

    let extn = Path::new(&s)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let md = fs::metadata(&s).map_err(|e| e.to_string())?;

    if extn == "png" && md.len() < 4097 {
        Ok(())
    } else {
        Err(format!(
            "'{}' has invalid format, run --help to see format.",
            s
        ))
    }
}

fn validate_file_exists(s: &str) -> Result<(), String> {
    if Path::new(&s).is_file() {
        Ok(())
    } else {
        Err(format!("'{}' does not exist.", s))
    }
}
