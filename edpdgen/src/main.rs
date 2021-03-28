use chrono::Local;
use colored::*;
use edpdgen_lib::EdpdLogger;

fn get_time_stamp() -> String {
    Local::now().format("%y-%m-%d %H:%M:%3f").to_string()
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

fn main() -> Result<(), String> {
    // let path = "D:/delme/dicts/Pali English Dictionary-full.csv";
    let path = "D:/delme/dicts/Pali_English_Dictionary_10_rows-full.csv";
    edpdgen_lib::run(path, &ColoredConsoleLogger {})
}
