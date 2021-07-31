use structopt::StructOpt;
use std::io::{self, BufRead};
use fraction::{BigDecimal, ToPrimitive};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "t", long = "type")]
    datatype: Option<String>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let numstr = line.trim();
        dbg!(numstr);
        let num = BigDecimal::from_decimal_str(numstr)
            .expect("could not parse number");
        dbg!(num.to_f64().unwrap());
    }

    Ok(())
}
