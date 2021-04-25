use std::str::FromStr;

#[derive(Debug)]
pub enum OutputFormat {
    /// StarDict and GoldenDict formats.
    StarDict,
    /// Ven. Anandajyoti Dictionary format.
    AjDict,
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
