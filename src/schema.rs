use super::ast::{Def, Namespace, Struct};
use mazzaroth_xdr::{
    ArrayColumn, BasicColumn, BasicType, Column, Schema, StructColumn, Table, TypedefColumn,
};
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
    match typedefs.get(&def.type_name) {
        Some(td) => {
            let mut result = TypedefColumn::default();
            result.name = def.name.clone();
            if def.array_size != 0 {
                let mut array_def = def.clone();
                array_def.array_size = 0;
                result.child = vec![Column::ARRAY(ArrayColumn {
                    name: def.name.clone(),
                    fixed: def.fixed_array,
                    length: def.array_size as u32,
                    column: vec![def_to_column(array_def, structs, typedefs)?],
                })];
            } else {
                result.child = vec![def_to_column(td.clone(), structs, typedefs)?];
            }
            return Ok(Column::TYPEDEF(result));
        }
        None => {}
    }

    match structs.get(&def.type_name) {
        Some(val) => {
            let mut prop_cols = vec![];
            for prop in &val.props {
                prop_cols.push(def_to_column(prop.clone(), structs, typedefs)?);
            }
            return Ok(Column::STRUCT(StructColumn {
                name: def.name.clone(),
                columns: prop_cols,
            }));
        }
        None => {}
    };

    if def.array_size != 0 && def.type_name != "string" && def.type_name != "opaque" {
        let mut array_def = def.clone();
        array_def.array_size = 0;
        return Ok(Column::ARRAY(ArrayColumn {
            name: def.name.clone(),
            fixed: def.fixed_array,
            length: def.array_size as u32,
            column: vec![def_to_column(array_def, structs, typedefs)?],
        }));
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
