use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum OutputFormat {
    /// StarDict and GoldenDict formats.
    StarDict,
    /// Ven. Anandajyoti Dictionary format.
    AjDict,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::StarDict => write!(f, "stardict"),
            OutputFormat::AjDict => write!(f, "ajdict"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stardict" => Ok(OutputFormat::StarDict),
            "ajdict" => Ok(OutputFormat::AjDict),
            _ => Err("Unknown output format".to_string()),
        }
    }
}
