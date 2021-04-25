use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum InputFormat {
    /// Digital Pali Dictionary
    Dpd,
    /// Devmitta Pali Study
    Dps,
}

impl fmt::Display for InputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputFormat::Dpd => write!(f, "dpd"),
            InputFormat::Dps => write!(f, "dps"),
        }
    }
}

impl FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dpd" => Ok(InputFormat::Dpd),
            "dps" => Ok(InputFormat::Dps),
            _ => Err("Unknown input format".to_string()),
        }
    }
}
