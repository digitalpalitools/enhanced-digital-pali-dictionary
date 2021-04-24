use std::str::FromStr;

#[derive(Debug)]
pub enum OutputFormat {
    StarDict,
    AnandaJyotiDictionary,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stardict" => Ok(OutputFormat::StarDict),
            "ajdict" => Ok(OutputFormat::AnandaJyotiDictionary),
            _ => Err("Unknown output format".to_string()),
        }
    }
}
