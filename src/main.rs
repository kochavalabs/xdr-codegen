extern crate pest;
extern crate structopt;

#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;
use structopt::StructOpt;

use pest::iterators::Pair;
use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/xdr.pest"]
pub struct XDRParser;

use std::fs::File;
use std::io::{self, Read};

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

#[derive(Debug, Default)]
struct Def {
    name: String,

    type_name: String,

    fixed_array: bool,

    array_size: i32,
}

#[derive(Debug, Default)]
struct Struct {
    name: String,

    props: Vec<Def>,
}

#[derive(Debug, Default)]
struct EnumValue {
    name: String,

    index: i32,
}

#[derive(Debug, Default)]
struct Enum {
    name: String,

    values: Vec<EnumValue>,
}

#[derive(Debug, Default)]
struct Union {}

#[derive(Debug, Default)]
struct Typedef {
    def: Def,
}

#[derive(Debug, Default)]
struct Namespace {
    name: String,

    typedefs: Vec<Typedef>,

    //    unions: Vec<Union>,
    enums: Vec<Enum>,

    structs: Vec<Struct>,
}

trait CodeGenerator {
    fn gen_code(&self, namespaces: Vec<Namespace>) -> String;

    fn gen_language(&self) -> String;
}

fn name_from_bracket_start(bs: Pair<Rule>) -> Result<String, &'static str> {
    for node in bs.into_inner() {
        if node.as_rule() == Rule::identifier {
            return Ok(node.as_str().to_string());
        }
    }
    Err("bracket_start did not parse")
}

fn get_array_info(d: Pair<Rule>) -> Result<(bool, i32), &'static str> {
    let mut fixed_array: bool = false;
    let mut array_size: i32 = std::i32::MAX;
    for node in d.into_inner() {
        match node.as_rule() {
            Rule::var_array => {
                fixed_array = false;
                let len = node.as_str();
                if len != "<>" {
                    array_size = len[1..len.len() - 1].parse::<i32>().unwrap();
                }
            }
            Rule::fixed_array => {
                let len = node.as_str();
                fixed_array = true;
                array_size = len[1..len.len() - 1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    Ok((fixed_array, array_size))
}

fn build_def(d: Pair<Rule>) -> Result<Def, &'static str> {
    let mut name: String = "".to_string();
    let mut type_name: String = "".to_string();
    let mut fixed_array: bool = false;
    let mut array_size: i32 = 0;
    let mut id_count = 0;
    for node in d.into_inner() {
        match node.as_rule() {
            Rule::types | Rule::identifier => {
                if id_count == 0 {
                    type_name = node.as_str().to_string();
                } else {
                    name = node.as_str().to_string();
                }
                id_count += 1;
            }
            Rule::array_def => {
                let (f, a) = get_array_info(node)?;
                fixed_array = f;
                array_size = a;
            }

            _ => {}
        }
    }

    Ok(Def {
        name: name,
        type_name: type_name,
        fixed_array: fixed_array,
        array_size: array_size,
    })
}

fn build_typedef(td: Pair<Rule>) -> Result<Typedef, &'static str> {
    let mut def = Def::default();
    for node in td.into_inner() {
        match node.as_rule() {
            Rule::type_decl => {
                let built_def = build_def(node)?;
                def = built_def;
            }
            _ => {}
        }
    }
    Ok(Typedef { def: def })
}

fn build_struct(st: Pair<Rule>) -> Result<Struct, &'static str> {
    let mut name: String = "".to_string();
    let mut props: Vec<Def> = Vec::new();
    for node in st.into_inner() {
        match node.as_rule() {
            Rule::bracket_start => {
                name = name_from_bracket_start(node)?;
            }
            Rule::type_decl => {
                let decl = build_def(node)?;
                props.push(decl);
            }
            _ => {}
        }
    }

    Ok(Struct {
        name: name,
        props: props,
    })
}

fn build_enum_val(en: Pair<Rule>) -> Result<EnumValue, &'static str> {
    let mut name: String = "".to_string();
    let mut index: i32 = 0;
    for node in en.into_inner() {
        match node.as_rule() {
            Rule::identifier => {
                name = node.as_str().to_string();
            }
            Rule::num_p => {
                index = node.as_str().parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    Ok(EnumValue {
        name: name,
        index: index,
    })
}

fn build_enum(en: Pair<Rule>) -> Result<Enum, &'static str> {
    let mut name: String = "".to_string();
    let mut values: Vec<EnumValue> = Vec::new();
    for node in en.into_inner() {
        match node.as_rule() {
            Rule::bracket_start => {
                name = name_from_bracket_start(node)?;
            }
            Rule::enum_decl => {
                let val = build_enum_val(node)?;
                values.push(val);
            }
            _ => {}
        }
    }

    Ok(Enum {
        name: name,
        values: values,
    })
}

fn build_namespace(ns: Pair<Rule>) -> Result<Namespace, &'static str> {
    let mut name: String = "".to_string();
    let mut typedefs: Vec<Typedef> = Vec::new();
    let mut structs: Vec<Struct> = Vec::new();
    let mut enums: Vec<Enum> = Vec::new();
    for node in ns.into_inner() {
        match node.as_rule() {
            Rule::bracket_start => {
                name = name_from_bracket_start(node)?;
            }
            Rule::typedef => {
                let def = build_typedef(node)?;
                typedefs.push(def)
            }
            Rule::Struct => {
                let stru = build_struct(node)?;
                structs.push(stru);
            }
            Rule::Enum => {
                let enu = build_enum(node)?;
                enums.push(enu);
            }
            _ => {}
        }
    }

    Ok(Namespace {
        name: name,
        typedefs: typedefs,
        structs: structs,
        enums: enums,
    })
}

fn build_namespaces(raw_idl: String) -> Result<Vec<Namespace>, &'static str> {
    let mut namespaces: Vec<Namespace> = Vec::new();
    let file = XDRParser::parse(Rule::file, &raw_idl)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();
    for node in file.into_inner() {
        match node.as_rule() {
            Rule::namespace => {
                let namespace = build_namespace(node)?;
                namespaces.push(namespace);
            }
            _ => {}
        }
    }
    Ok(namespaces)
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

    println!("{:?}", build_namespaces(buffer));
    Ok(())
}
