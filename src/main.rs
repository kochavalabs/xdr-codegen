extern crate pest;
extern crate structopt;

#[macro_use]
extern crate pest_derive;


use std::path::PathBuf;
use structopt::StructOpt;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/xdr.pest"]
pub struct XDRParser;


use std::io::{self, Read};
use std::fs::File;


#[derive(Debug, StructOpt)]
#[structopt(name = "xdrgen", about = "CLI tool for generating xdr code.")]
struct Opt {
    /// Input files, stdin if not present
    #[structopt(parse(from_os_str))]
    input: Vec<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,

    /// Output language
    #[structopt(short = "l", long = "language")]
    language: Option<String>,
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let opt = Opt::from_args();

    println!("{:?}", opt.input);

    match opt.input.len() {
        0 => {
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer)?;
        },
        _ => {
            for file in opt.input.iter() {
                let mut f = File::open(file)?;
                f.read_to_string(&mut buffer)?;
            }
            println!("{:?}", opt);
        }
    }

    let file_ast = XDRParser::parse(Rule::file, &buffer)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    println!("{:?}", file_ast);
    Ok(())
}
