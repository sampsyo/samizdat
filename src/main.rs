use samizdat::datatype::DataType;
use samizdat::format::Format;
use std::io;
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

fn main() -> io::Result<()> {
    let opt: Opt = argh::from_env();
    samizdat::convert(&mut io::stdin().lock(), &mut io::stdout(), opt.datatype, opt.from_format, opt.to_format)
}
