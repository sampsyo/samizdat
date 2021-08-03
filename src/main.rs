mod datatype;
mod format;
use datatype::DataType;
use format::Format;
use fraction::{BigDecimal, ToPrimitive};
use std::io::{self, Read, BufRead};
use std::string::ToString;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Encode and decode numerical data.
struct Opt {
    /// numerical representation
    #[argh(option, short = 't', long = "type", default = "DataType::Float32")]
    datatype: DataType,

    /// input file format
    #[argh(option, long = "from", default = "Format::Text")]
    from_format: Format,

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

fn read_from_bytes<T: Read>(src: &mut T, typ: DataType) -> io::Result<BigDecimal> {
    match typ {
        DataType::Float32 => {
            let mut buf = [0u8; 4];
            src.read_exact(&mut buf)?;
            Ok(f32::from_be_bytes(buf).into())
        },
        DataType::Float64 => todo!(),
        DataType::Fixed(_, _, _) => panic!("fixed point unimplemented"),
    }
}

fn read_from_hex<T: Read>(src: &mut T, typ: DataType) -> io::Result<Option<BigDecimal>> {
    match typ {
        DataType::Float32 => {
            let mut enc_buf = [0u8; 8];
            match src.read_exact(&mut enc_buf) {
                Ok(()) => {},
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
                Err(e) => return Err(e),
            }
            let mut buf = [0u8; 4];
            hex::decode_to_slice(enc_buf, &mut buf).expect("could not parse hex data");
            Ok(Some(f32::from_be_bytes(buf).into()))
        },
        DataType::Float64 => todo!(),
        DataType::Fixed(_, _, _) => panic!("fixed point unimplemented"),
    }
}

fn read_from_text<T: BufRead>(src: &mut T) -> io::Result<Option<BigDecimal>> {
    let mut buf = String::new();
    Ok(if src.read_line(&mut buf)? != 0 {
        Some(BigDecimal::from_decimal_str(buf.trim()).expect("could not parse number"))
    } else {
        None
    })
}

fn main() -> io::Result<()> {
    let opt: Opt = argh::from_env();

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    loop {
        // Read.
        let res = match opt.from_format {
            Format::Text => read_from_text(&mut stdin_lock)?,
            Format::Hex => read_from_hex(&mut stdin_lock, opt.datatype)?,
        };

        // Write.
        match res {
            Some(num) => {
                match opt.to_format {
                    Format::Hex => {
                        // Dump the binary data as hex.
                        let bytes = to_bytes(num, opt.datatype);
                        for byte in bytes.iter() {
                            print!("{:x}", byte);
                        }
                    },
                    Format::Text => {
                        println!("{}", num);
                    },
                }
            },
            None => break,
        }
    }

    Ok(())
}
