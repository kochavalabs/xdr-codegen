use super::ast::*;
use std::collections::HashMap;

pub mod go;
pub mod js;
pub mod rust;
pub mod node;

pub trait CodeGenerator {
    fn code(&self, namespace: Vec<Namespace>) -> Result<String, &'static str>;

    fn language(&self) -> String;
}

pub fn apply_type_map(
    namespaces: Vec<Namespace>,
    type_map: HashMap<&str, &str>,
) -> Result<Vec<Namespace>, &'static str> {
    let mut ret_val = namespaces.clone();
    for n_i in 0..ret_val.len() {
        for td_i in 0..ret_val[n_i].typedefs.len() {
            match type_map.get(&ret_val[n_i].typedefs[td_i].def.type_name[..]) {
                Some(&val) => {
                    ret_val[n_i].typedefs[td_i].def.type_name = val.to_string();
                }
                _ => {}
            }
        }
        for str_i in 0..ret_val[n_i].structs.len() {
            for st_def_i in 0..ret_val[n_i].structs[str_i].props.len() {
                match type_map.get(&ret_val[n_i].structs[str_i].props[st_def_i].type_name[..]) {
                    Some(&val) => {
                        ret_val[n_i].structs[str_i].props[st_def_i].type_name = val.to_string();
                    }
                    _ => {}
                }
            }
        }

        for uni_i in 0..ret_val[n_i].unions.len() {
            for case_i in 0..ret_val[n_i].unions[uni_i].switch.cases.len() {
                match type_map.get(
                    &ret_val[n_i].unions[uni_i].switch.cases[case_i]
                        .ret_type
                        .type_name[..],
                ) {
                    Some(&val) => {
                        ret_val[n_i].unions[uni_i].switch.cases[case_i]
                            .ret_type
                            .type_name = val.to_string();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(ret_val)
}
