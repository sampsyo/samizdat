pub mod datatype;
pub mod format;

use datatype::DataType;
use format::Format;
use num::rational::BigRational;
use num::bigint::BigInt;
use num::{ToPrimitive, FromPrimitive, Zero};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str::FromStr;
use std::convert::TryInto;

fn to_bytes(num: BigRational, typ: DataType) -> Box<[u8]> {
    match typ {
        DataType::Float32 => Box::new(num.to_f32().unwrap().to_le_bytes()),
        DataType::Float64 => Box::new(num.to_f64().unwrap().to_le_bytes()),
        DataType::Fixed(_, _, _) => {
            panic!("fixed point unimplemented");
        }
    }
}

fn hex_read<T: Read, const LEN: usize>(src: &mut T, buf: &mut [u8; LEN]) -> io::Result<bool> {
    let mut enc_buf = vec![0u8; LEN * 2];
    match src.read_exact(&mut enc_buf) {
        Ok(()) => {}
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(false),
        Err(e) => return Err(e),
    }
    hex::decode_to_slice(enc_buf, buf).expect("could not parse hex data");
    Ok(true)
}

fn read_from_hex<T: Read>(src: &mut T, typ: DataType) -> io::Result<Option<BigRational>> {
    match typ {
        DataType::Float32 => {
            let mut buf = [0u8; 4];
            if hex_read(src, &mut buf)? {
                Ok(BigRational::from_f32(f32::from_le_bytes(buf)))
            } else {
                Ok(None)
            }
        }
        DataType::Float64 => {
            let mut buf = [0u8; 8];
            if hex_read(src, &mut buf)? {
                Ok(BigRational::from_f64(f64::from_le_bytes(buf)))
            } else {
                Ok(None)
            }
        }
        DataType::Fixed(_, _, _) => panic!("fixed point unimplemented"),
    }
}

/// Parse decimal numbers of the format [-]I[.F], where I and F are strings of decimals.
/// TODO: Should return a Result.
fn parse_decimal(s: &str) -> BigRational {
    let mut split = s.splitn(2, '.');

    let int: BigInt = FromStr::from_str(
        split.next().expect("missing integer part")
    ).expect("could not parse integer part");

    let frac = match split.next() {
        Some(frac_s) => {
            let mant = BigInt::from_str(frac_s).expect("could not parse fractional part");
            let exp = frac_s.len();
            let mag = BigInt::from(10u8).pow(exp.try_into().unwrap());  // 10^exp
            BigRational::new(mant, mag)  // mant * 10^-exp
        },
        None => BigRational::zero()
    };

    BigRational::from_integer(int) + frac
}

/// Format a number for text output. Currently not arbitrary-precision; we just format the f64
/// approximation.
fn fmt_decimal(r: &BigRational) -> String {
    format!("{}", r.to_f64().unwrap())
}

fn read_from_text<T: BufRead>(src: &mut T) -> io::Result<Option<BigRational>> {
    let mut buf = String::new();
    Ok(if src.read_line(&mut buf)? != 0 {
        Some(parse_decimal(buf.trim()))
    } else {
        None
    })
}

pub fn convert<I: BufRead, O: Write>(
    input: &mut I,
    output: &mut O,
    datatype: DataType,
    from_format: Format,
    to_format: Format,
) -> io::Result<()> {
    loop {
        // Read.
        let res = match from_format {
            Format::Text => read_from_text(input)?,
            Format::Hex => read_from_hex(input, datatype)?,
        };

        // Write.
        match res {
            Some(num) => {
                match to_format {
                    Format::Hex => {
                        // Dump the binary data as hex.
                        let bytes = to_bytes(num, datatype);
                        write!(output, "{}", hex::encode(bytes))?;
                    }
                    Format::Text => {
                        writeln!(output, "{}", fmt_decimal(&num))?;
                    }
                }
            }
            None => break,
        }
    }

    Ok(())
}

pub fn convert_string(
    input: &str,
    datatype: DataType,
    from_format: Format,
    to_format: Format,
) -> io::Result<String> {
    let mut inp = BufReader::new(input.as_bytes());
    let mut out = Vec::<u8>::new();
    convert(&mut inp, &mut out, datatype, from_format, to_format)?;
    Ok(String::from_utf8(out).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::convert_string;
    use crate::datatype::DataType;
    use crate::format::Format;

    fn round_trip_hex(text: &str, datatype: DataType) -> String {
        let hex = convert_string(text, datatype, Format::Text, Format::Hex).unwrap();
        convert_string(&hex, datatype, Format::Hex, Format::Text).unwrap()
    }

    const NUMBERS: &str = r#"1.23
        1.234567890123456789012345678901
        -5"#;

    #[test]
    fn text_to_hex_f32() {
        insta::assert_snapshot!(round_trip_hex(NUMBERS, DataType::Float32), @r###"
        1.2300000190734863
        1.2345678806304932
        -5
        "###);
    }

    #[test]
    fn text_to_hex_f64() {
        insta::assert_snapshot!(round_trip_hex(NUMBERS, DataType::Float64), @r###"
        1.23
        1.2345678901234567
        -5
        "###);
    }
}
