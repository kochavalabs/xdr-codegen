use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/xdr.pest"]
pub struct XDRParser;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Def {
    pub name: String,

    pub type_name: String,

    pub fixed_array: bool,

    pub array_size: i32,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Struct {
    pub name: String,

    pub props: Vec<Def>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct EnumValue {
    pub name: String,

    pub index: i32,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Enum {
    pub name: String,

    pub values: Vec<EnumValue>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Switch {
    pub enum_name: String,

    pub enum_type: String,

    pub cases: Vec<Case>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Union {
    pub name: String,

    pub switch: Switch,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Case {
    pub value: String,

    pub ret_type: Def,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Typedef {
    pub def: Def,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Namespace {
    pub name: String,

    pub typedefs: Vec<Typedef>,

    pub unions: Vec<Union>,

    pub enums: Vec<Enum>,

    pub structs: Vec<Struct>,
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

fn build_case(ca: Pair<Rule>) -> Result<Case, &'static str> {
    let mut value: String = "".to_string();
    let mut def = Def::default();
    def.name = "void".to_string();
    for node in ca.into_inner() {
        match node.as_rule() {
            Rule::identifier => {
                value = node.as_str().to_string();
            }
            Rule::type_decl => {
                def = build_def(node)?;
            }
            _ => {}
        }
    }
    Ok(Case {
        value: value,
        ret_type: def,
    })
}

fn build_switch(sw: Pair<Rule>) -> Result<Switch, &'static str> {
    let mut enum_name: String = "".to_string();
    let mut enum_type: String = "".to_string();
    let mut cases: Vec<Case> = Vec::new();
    for node in sw.into_inner() {
        match node.as_rule() {
            Rule::single_param => {
                let type_id = type_id_from_single_param(node)?;
                enum_type = type_id.0;
                enum_name = type_id.1;
            }
            Rule::case_statement => {
                let cas = build_case(node)?;
                cases.push(cas);
            }
            _ => {}
        }
    }

    Ok(Switch {
        enum_name: enum_name,
        enum_type: enum_type,
        cases: cases,
    })
}

fn type_id_from_single_param(pa: Pair<Rule>) -> Result<(String, String), &'static str> {
    let mut sw_type: String = "".to_string();
    let mut id: String = "".to_string();
    let mut id_count = 0;
    for node in pa.into_inner() {
        match (node.as_rule(), id_count) {
            (Rule::identifier, 0) => {
                sw_type = node.as_str().to_string();
                id_count += 1;
            }
            (Rule::identifier, 1) => {
                id = node.as_str().to_string();
            }
            _ => {}
        }
    }
    Ok((sw_type, id))
}

fn build_union(un: Pair<Rule>) -> Result<Union, &'static str> {
    let mut name: String = "".to_string();
    let mut switch: Switch = Switch::default();
    for node in un.into_inner() {
        match node.as_rule() {
            Rule::identifier => {
                name = node.as_str().to_string();
            }
            Rule::switch => {
                switch = build_switch(node)?;
            }
            Rule::enum_decl => {}
            _ => {}
        }
    }

    Ok(Union {
        name: name,
        switch: switch,
    })
}

fn build_namespace(ns: Pair<Rule>) -> Result<Namespace, &'static str> {
    let mut name: String = "".to_string();
    let mut typedefs: Vec<Typedef> = Vec::new();
    let mut structs: Vec<Struct> = Vec::new();
    let mut enums: Vec<Enum> = Vec::new();
    let mut unions: Vec<Union> = Vec::new();
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
            Rule::union => {
                let uni = build_union(node)?;
                unions.push(uni);
            }
            _ => {}
        }
    }

    Ok(Namespace {
        name: name,
        typedefs: typedefs,
        structs: structs,
        enums: enums,
        unions: unions,
    })
}

pub fn build_namespaces(raw_idl: String) -> Result<Vec<Namespace>, &'static str> {
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
