extern crate pest;
extern crate structopt;
extern crate xdr_rs_serialize;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;
use structopt::StructOpt;
use xdr_rs_serialize::ser::*;

use std::fs::File;
use std::io::{self, Read};

mod ast;
mod generator;

#[derive(Debug, StructOpt)]
#[structopt(name = "xdr-codegen", about = "CLI tool for generating xdr code.")]
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

    /// Include macro_use for xdr_rs_serialize_derive crate in Rust
    #[structopt(short = "m", long = "macro")]
    include_macro: bool,
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

    let namespaces = ast::build_namespaces(buffer).unwrap();

    let rust_generator;
    let generator: &dyn generator::CodeGenerator = match opt.language {
        Some(language) => match language.as_ref() {
            "go" => &generator::go::GoGenerator {},
            "js" => &generator::js::JsGenerator {},
            "rust" => {
                rust_generator = generator::rust::RustGenerator {
                    include_macro: opt.include_macro,
                };
                &rust_generator
            }
            "commonjs" => &generator::commonjs::CommonJsGenerator {},
            _ => panic!("Invalid language selection. Options: go, js, rust commonjs"),
        },
        _ => &generator::go::GoGenerator {},
    };

    let code = generator.code(namespaces).unwrap();
    match opt.output {
        None => {
            println!("{}", code);
        }
        Some(path) => {
            let mut file = File::create(path.to_str().unwrap())?;
            file.write_all(code.as_bytes())?;
        }
    }

    Ok(())
}
