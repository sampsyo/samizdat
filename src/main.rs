use structopt::StructOpt;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::string::ToString;
use std::fmt;
use fraction::{BigDecimal, ToPrimitive};

#[derive(Debug, Clone, Copy)]
enum DataType {
    Float32,
    Float64,
    Fixed(bool, usize, usize),
}

#[derive(Debug, Clone)]
struct ParseTypeError;

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
            DataType::Fixed(s, i, f) =>
                format!("{}{}.{}", if *s { "s" } else { "u" }, i, f),
        }
    }
}

impl fmt::Display for ParseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown data type")
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "t", long = "type", default_value = "f32")]
    datatype: DataType,
}

fn to_bytes(num: BigDecimal, typ: DataType) -> Box<[u8]> {
    match typ {
        DataType::Float32 =>
            Box::new(num.to_f32().unwrap().to_le_bytes()),
        DataType::Float64 =>
            Box::new(num.to_f64().unwrap().to_le_bytes()),
        DataType::Fixed(_, _, _) => panic!("unimplemented"),
    }
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let numstr = line.trim();
        let num = BigDecimal::from_decimal_str(numstr)
            .expect("could not parse number");
        let bytes = to_bytes(num, opt.datatype);

        // Dump the binary data as hex. Eventually we should make the output
        // format configurable, I guess.
        for byte in bytes.iter() {
            print!("{:x}", byte);
        }
    }

    Ok(())
}
