use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum InputFormat {
    DigitalPaliDictionary,
    DevamittaPaliStudy,
}

impl fmt::Display for InputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputFormat::DigitalPaliDictionary => write!(f, "dpd"),
            InputFormat::DevamittaPaliStudy => write!(f, "dps"),
        }
    }
}

impl FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dpd" => Ok(InputFormat::DigitalPaliDictionary),
            "dps" => Ok(InputFormat::DevamittaPaliStudy),
            _ => Err("Unknown input format".to_string()),
        }
    }
}
