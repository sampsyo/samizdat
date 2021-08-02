use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Float32,
    Float64,
    Fixed(bool, usize, usize),
}

#[derive(Debug, Clone)]
pub struct ParseTypeError;

impl From<std::num::ParseIntError> for ParseTypeError {
    fn from(_err: std::num::ParseIntError) -> ParseTypeError {
        ParseTypeError
    }
}

/// Parse precision specifications that look like "I.F", where I and F are the integer and
/// fractional bits, or just "I", which is shorthand for "I.0".
fn parse_precision(s: &str) -> Result<(usize, usize), ParseTypeError> {
    match s.find(".") {
        Some(dot) => {
            let (left, right) = s.split_at(dot);
            Ok((left.parse()?, right[1..].parse()?))
        },
        None => {
            Ok((s.parse()?, 0))
        },
    }
}

impl FromStr for DataType {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "f32" {
            Ok(DataType::Float32)
        } else if s == "f64" {
            Ok(DataType::Float64)
        } else if s.starts_with("s") {
            let (i, f) = parse_precision(&s[1..])?;
            Ok(DataType::Fixed(true, i, f))
        } else if s.starts_with("u") {
            let (i, f) = parse_precision(&s[1..])?;
            Ok(DataType::Fixed(false, i, f))
        } else {
            Err(ParseTypeError)
        }
    }
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Fixed(s, i, f) => {
                let sign = if *s { "s" } else { "u" };
                format!("{}{}.{}", sign, i, f)
            }
        }
    }
}

impl fmt::Display for ParseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown data type")
    }
}
