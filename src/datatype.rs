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

impl FromStr for DataType {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "f32" {
            Ok(DataType::Float32)
        } else if s == "f64" {
            Ok(DataType::Float64)
        } else {
            panic!("unimplemented");
        }
    }
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Fixed(s, i, f) => format!("{}{}.{}", if *s { "s" } else { "u" }, i, f),
        }
    }
}

impl fmt::Display for ParseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown data type")
    }
}
