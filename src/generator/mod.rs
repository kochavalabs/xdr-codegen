use super::ast::*;
use std::collections::HashMap;

pub mod commonjs;
pub mod go;
pub mod js;
pub mod rust;

pub trait CodeGenerator {
    fn code(&self, namespace: Vec<Namespace>) -> Result<String, &'static str>;
}

pub fn apply_type_map(mut namespaces: Vec<Namespace>, type_map: &HashMap<&str, &str>) -> Result<Vec<Namespace>, &'static str> {
    for namespace in &mut namespaces {
        for typedef in &mut namespace.typedefs {
            if let Some(&val) = type_map.get(typedef.def.type_name.as_str()) {
                typedef.def.type_name = val.to_string();
            }
        }
        for struct_ in &mut namespace.structs {
            for prop in &mut struct_.props {
                if let Some(&val) = type_map.get(prop.type_name.as_str()) {
                    prop.type_name = val.to_string();
                }
            }
        }
        for union_ in &mut namespace.unions {
            for switch_case in &mut union_.switch.cases {
                if let Some(&val) = type_map.get(switch_case.ret_type.type_name.as_str()) {
                    switch_case.ret_type.type_name = val.to_string();
                }
            }
        }
    }
    Ok(namespaces)
}
