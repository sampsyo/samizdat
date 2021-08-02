use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Format {
    Text,
    Hex,
}

#[derive(Debug, Clone)]
pub struct ParseFormatError;

impl fmt::Display for ParseFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown format")
    }
}

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "txt" {
            Ok(Format::Text)
        } else if s == "hex" {
            Ok(Format::Hex)
        } else {
            Err(ParseFormatError)
        }
    }
}
