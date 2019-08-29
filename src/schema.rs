use super::ast::{Def, Namespace, Struct};
use mazzaroth_xdr::{BasicColumn, BasicType, Column, Schema, Table};
use std::collections::HashMap;

fn string_to_basic_type(typ: String) -> Result<BasicType, &'static str> {
    match typ.as_str() {
        "boolean" => Ok(BasicType::BOOLEAN),
        "string" => Ok(BasicType::STRING),
        "opaque" => Ok(BasicType::OPAQUE),
        "int" => Ok(BasicType::INT),
        "unsigned int" => Ok(BasicType::UNSIGNED_INT),
        "hyper" => Ok(BasicType::HYPER),
        "unsigned hyper" => Ok(BasicType::UNSIGNED_HYPER),
        "float" => Ok(BasicType::FLOAT),
        "double" => Ok(BasicType::DOUBLE),
        _ => Err("Bad basic type."),
    }
}

fn def_to_column(
    def: Def,
    structs: &HashMap<String, Struct>,
    typedefs: &HashMap<String, Def>,
) -> Result<Column, &'static str> {
    if typedefs.contains_key(&def.name) {
        return Err("Typedef not implemented");
    }

    if structs.contains_key(&def.type_name) {
        return Err("Struct not implemented.");
    }

    if def.array_size != 0 && def.type_name != "string" {
        return Err("Arrays not implemented.");
    }

    Ok(Column::BASIC(BasicColumn {
        name: def.name,
        typ: string_to_basic_type(def.type_name)?,
    }))
}

fn build_schema(
    structs: HashMap<String, Struct>,
    typedefs: HashMap<String, Def>,
    tables: Vec<Struct>,
) -> Result<Schema, &'static str> {
    let mut schema = Schema::default();
    for table in tables {
        let mut tab = Table::default();
        tab.name = table.name;
        for prop in table.props {
            tab.columns.push(def_to_column(prop, &structs, &typedefs)?);
        }
        schema.tables.push(tab);
    }
    println!("{:?}", schema);
    Ok(schema)
}

pub fn generate_schema(namespaces: Vec<Namespace>) -> Result<Schema, &'static str> {
    let mut structs = HashMap::new();
    let mut typedefs = HashMap::new();
    let mut tables = vec![];
    for ns in namespaces {
        for st in ns.structs {
            structs.insert(st.name.clone(), st.clone());
            if st.tag == "table_schema".to_string() {
                tables.push(st);
            }
        }
        for td in ns.typedefs {
            typedefs.insert(td.def.name.clone(), td.def.clone());
        }
    }
    build_schema(structs, typedefs, tables)
}
