mod datatype;
use structopt::StructOpt;
use std::io::{self, BufRead};
use std::string::ToString;
use fraction::{BigDecimal, ToPrimitive};
use datatype::DataType;

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
