use chrono::{Datelike, SecondsFormat, Utc};
use edpdgen_lib::input::input_format::InputFormat;
use edpdgen_lib::DictionaryInfo;
use std::fs::File;
use std::io::Read;

mod args;
mod logger;

fn main() -> Result<(), String> {
    let l = logger::ColoredConsoleLogger {};

    let arg_matches = args::parse_args();
    let args = args::get_args(&arg_matches);
    let ts = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let di = create_dictionary_info(&args, &ts);

    print_banner();
    print_dictionary_info(&di);

    if args.what_if {
        println!("Not generating dictionary due to --what-if argument.");
        return Ok(());
    }

    edpdgen_lib::run(&di, &l)
}

fn create_dictionary_info<'a>(args: &'a args::EdpdArgs, time_stamp: &'a str) -> DictionaryInfo<'a> {
    let host_url = env!("CARGO_PKG_NAME");
    let host_version = env!("CARGO_PKG_VERSION");

    match &args.input_format {
        InputFormat::Dpd => {
            DictionaryInfo {
                author: "Digital Pāli Tools <digitalpalitools@gmail.com>",
                input_data_path: args.csv_path,
                input_format: &args.input_format,
                output_format: &args.output_format,
                output_folder: args.output_folder.unwrap_or("dicts"),
                time_stamp,
                host_url,
                host_version,
                feedback_form_url:
                    "https://docs.google.com/forms/d/1hMra0aMz65sYnRlPjGlTYQIHz-3_tKlywu3enqXlpSc/viewform",
                name: args.name.unwrap_or(if args.concise { "Concise Digital Pāli Tools Dictionary (CDPD)" } else { "Digital Pāli Tools Dictionary (DPD)" }),
                short_name: args.short_name.unwrap_or(if args.concise { "cdpd" } else { "dpd" }),
                description: args.description.unwrap_or(if args.concise { "The next generation concise Digital Pāli Dictionary." } else { "The next generation comprehensive Digital Pāli Dictionary." }),
                links_color: args.links_color.unwrap_or("#0006c8"),
                headings_color: args.headings_color.unwrap_or("#747592"),
                icon_path: args.icon_path,
                icon: read_icon_bytes(args.icon_path, &args.input_format),
                inflections_db_path: args.inflections_db_path,
                concise: args.concise,
            }
        }
        InputFormat::Dps => {
            DictionaryInfo {
                author: "Devamitta Bhikkhu",
                input_data_path: args.csv_path,
                input_format: &args.input_format,
                output_format: &args.output_format,
                output_folder: args.output_folder.unwrap_or("dicts"),
                time_stamp,
                host_url,
                host_version,
                feedback_form_url:
                    "https://docs.google.com/forms/d/e/1FAIpQLSc87oKqninpyg01YWdsjdYK6wSeIMoAZpy2jNM7Wu0KYygnHw/viewform",
                name: args.name.unwrap_or(if args.concise { "Concise Devamitta Pāli Study (CDPS)" } else { "Devamitta Pāli Study (DPS)" }),
                short_name: args.short_name.unwrap_or(if args.concise { "cdps" } else { "dps" }),
                description: args.description.unwrap_or(if args.concise { "A concise Pāli language word lookup." } else { "A detailed Pāli language word lookup." }),
                links_color: args.links_color.unwrap_or("orange"),
                headings_color: args.headings_color.unwrap_or("green"),
                icon_path: args.icon_path,
                icon: read_icon_bytes(args.icon_path, &args.input_format),
                inflections_db_path: args.inflections_db_path,
                concise: args.concise,
            }
        }
    }
}

fn print_banner() {
    println!(
        "{} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    println!(
        "(c) 2020 - {}, {}",
        Utc::now().year(),
        env!("CARGO_PKG_AUTHORS")
    );
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("This work is licensed under the {} license (https://creativecommons.org/licenses/by-nc-sa/4.0/)", env!("CARGO_PKG_LICENSE"));
    println!();
}

fn print_dictionary_info(di: &DictionaryInfo) {
    println!("Generating dictionary with following parameters:");
    println!("... Name: {}", di.name);
    println!("... Short name: {}", di.short_name);
    println!("... Description: {}", di.description);
    println!("... Author: {}", di.author);
    println!("... Input data path: {}", di.input_data_path);
    println!("... Input format: {}", di.input_format);
    println!("... Output format: {}", di.output_format);
    println!("... Output folder: {}", di.output_folder);
    println!("... Links color: {}", di.links_color);
    println!("... Headings color: {}", di.headings_color);
    println!(
        "... Icon: {}",
        if let Some(ipath) = di.icon_path {
            ipath.to_string()
        } else {
            format!("<default icon for {}>", di.short_name)
        }
    );
    println!("... Feedback URL: {}", di.feedback_form_url);
    println!(
        "... Inflections: {}",
        if let Some(idb) = di.inflections_db_path {
            idb
        } else {
            "<will not generate>"
        }
    );
    println!();
}

fn read_icon_bytes(ico_path: Option<&str>, input_format: &InputFormat) -> Vec<u8> {
    let res = ico_path
        .ok_or(())
        .and_then(|path| File::open(path).map_err(|_| ()))
        .as_mut()
        .map(|f: &mut File| {
            let mut buffer: Vec<u8> = vec![];
            let _ = f.read_to_end(&mut buffer);
            buffer
        })
        .unwrap_or_else(|_| match input_format {
            InputFormat::Dpd => include_bytes!("dpd.png").to_vec(),
            InputFormat::Dps => include_bytes!("dps.png").to_vec(),
        });

    res
}
