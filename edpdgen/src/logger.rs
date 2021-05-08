use chrono::Local;
use colored::*;
use pls_core_extras::logger::PlsLogger;

fn get_time_stamp() -> String {
    Local::now().format("%y-%m-%d %H:%M:%S").to_string()
}

pub(crate) struct ColoredConsoleLogger;

impl PlsLogger for ColoredConsoleLogger {
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
