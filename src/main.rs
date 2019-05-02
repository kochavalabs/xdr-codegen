extern crate pest;
extern crate structopt;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{self, Read};

mod ast;
mod generator;

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

    match opt.input.len() {
        0 => {
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer)?;
        }
        _ => {
            for file in opt.input.iter() {
                let mut f = File::open(file)?;
                f.read_to_string(&mut buffer)?;
            }
        }
    }
    let generator: &generator::CodeGenerator = match opt.language {
        Some(language) => match language.as_ref() {
            "go" => &generator::go::GoGenerator {},
            "js" => &generator::js::JsGenerator {},
            _ => &generator::go::GoGenerator {},
        },
        _ => &generator::go::GoGenerator {},
    };

    println!(
        "{}",
        generator
            .code(ast::build_namespaces(buffer).unwrap())
            .unwrap()
    );
    Ok(())
}
