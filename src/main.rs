mod datatype;
mod format;
use datatype::DataType;
use format::Format;
use fraction::{BigDecimal, ToPrimitive};
use std::io::{self, BufRead};
use std::string::ToString;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Encode and decode numerical data.
struct Opt {
    /// numerical representation
    #[argh(option, short = 't', long = "type", default = "DataType::Float32")]
    datatype: DataType,

    /// output file format
    #[argh(option, long = "to", default = "Format::Hex")]
    to_format: Format,
}

fn to_bytes(num: BigDecimal, typ: DataType) -> Box<[u8]> {
    match typ {
        DataType::Float32 => Box::new(num.to_f32().unwrap().to_le_bytes()),
        DataType::Float64 => Box::new(num.to_f64().unwrap().to_le_bytes()),
        DataType::Fixed(_, _, _) => panic!("fixed point unimplemented"),
    }
}

fn main() -> io::Result<()> {
    let opt: Opt = argh::from_env();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let num =
            BigDecimal::from_decimal_str(line.unwrap().trim()).expect("could not parse number");
        let bytes = to_bytes(num, opt.datatype);

        match opt.to_format {
            Format::Hex => {
                // Dump the binary data as hex.
                for byte in bytes.iter() {
                    print!("{:x}", byte);
                }
            },
            Format::Text => {
                panic!("text output unimplemented");
            },
        }
    }

    Ok(())
}
