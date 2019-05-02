use super::*;
use handlebars::Handlebars;
use std::collections::HashMap;

static HEADER: &str = r#"

import * as XDR from 'js-xdr';


var types = XDR.config(xdr => {
{{#each this as |ns| ~}}
// Namspace start {{ns.name}}
"#;

static TYPEDEFS_T: &str = r#"
// Start typedef section
{{#each ns.typedefs as |td| ~}}

xdr.typedef("{{td.def.name}}", xdr.{{td.def.type_name}}(32));

{{/each~}}
// End typedef section
"#;

static STRUCTS_T: &str = r#"
// Start struct section

// End struct section
"#;

static ENUM_T: &str = r#"
// Start enum section

// End enum section
"#;

static UNION_T: &str = r#"
// Start union section

// End union section
"#;

static FOOTER: &str = r#"
// End namespace {{ns.name}}

{{/each~}}
});
export default types;
"#;

#[derive(Debug, Default)]
pub struct JsGenerator {}

fn build_file_template() -> String {
    format!(
        "{}{}{}{}{}{}",
        HEADER, TYPEDEFS_T, STRUCTS_T, ENUM_T, UNION_T, FOOTER
    )
}

fn process_namespaces(namespaces: Vec<Namespace>) -> Result<Vec<Namespace>, &'static str> {
    let mut type_map = HashMap::new();
    type_map.insert("xasdfe", "bool");
    let ret_val = apply_type_map(namespaces, type_map)?;
    Ok(ret_val)
}

impl CodeGenerator for JsGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let reg = Handlebars::new();
        let file_t = build_file_template();
        let processed_ns = process_namespaces(namespaces)?;
        let result = reg
            .render_template(file_t.into_boxed_str().as_ref(), &processed_ns)
            .unwrap();

        return Ok(result);
    }

    fn language(&self) -> String {
        "go".to_string()
    }
}
