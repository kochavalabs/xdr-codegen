extern crate pest;

use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammars/xdr.pest"]
pub struct XDRParser;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let file = XDRParser::parse(Rule::file, &buffer)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    println!("{:?}", file);
    Ok(())
}
