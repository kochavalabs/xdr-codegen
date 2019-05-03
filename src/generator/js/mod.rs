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
{{#each ns.structs as |st| ~}}
xdr.struct("{{st.name}}", [
{{#each st.props as |prop|}}
  ["{{prop.name}}", {{#typeconv prop.name prop.type_name prop.array_size prop.fixed_array}}{{/typeconv}}],
{{/each}}
]);
{{/each}}

// End struct section
"#;

static ENUM_T: &str = r#"
// Start enum section
{{#each ns.enums as |enum|}}
xdr.enum("{{enum.name}}", {
{{#each enum.values as |val|}}
  {{val.name}}: {{val.index}},
{{/each~}}
});
{{/each}}

// End enum section
"#;

static UNION_T: &str = r#"
// Start union section

{{#each ns.unions as |uni|}}
xdr.union("{{uni.name}}", {
  switchOn: xdr.lookup("{{uni.switch.enum_type}}"),
  switchName: "{{uni.switch.enum_name}}",
  switches: [
    {{#each uni.switch.cases as |case|~}}
    {{#if (not (isvoid case.ret_type.name))}}
        ["{{case.value}}", "{{case.value}}"],
    {{else}}
        ["{{case.value}}", xdr.void()], 
    {{/if}}
    {{/each~}}
  ],
  arms: {
    {{#each uni.switch.cases as |case| ~}}
    {{#if (not (isvoid case.ret_type.name)) ~}}
        {{case.value}}: {{#typeconv case.ret_type.name case.ret_type.type_name case.ret_type.array_size case.ret_type.fixed_array}}{{/typeconv}},
    {{/if}}
    {{/each~}}
  },
});
{{/each}}

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

fn is_array_type(def_type: &str) -> bool {
    match def_type {
        "string" => true,
        "opaque" => true,
        _ => false,
    }
}

fn is_built_in(def_type: &str) -> bool {
    match def_type {
        "void" => true,
        "bool" => true,
        "int" => true,
        "hyper" => true,
        "uint" => true,
        "uhyper" => true,
        "float" => true,
        "double" => true,
        "quadruple" => true,
        _ => false,
    }
}

fn is_built_in_single(def_type: &str) -> bool {
    !is_array_type(def_type) && is_built_in(def_type)
}

impl CodeGenerator for JsGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let mut type_map = HashMap::new();
        type_map.insert("boolean", "bool");
        type_map.insert("integer", "int");
        type_map.insert("unsigned integer", "uint");
        type_map.insert("unsigned hyper", "uhyper");
        let processed = apply_type_map(namespaces, type_map)?;
        let mut reg = Handlebars::new();
        let file_t = build_file_template();
        handlebars_helper!(typeconv: |name: str, typ: str, size: i64, fixed: bool| match (name, typ, size, fixed) {
            (_, typ, size, _) if is_built_in_single(typ) && size == 0 => format!("xdr.{}()", typ),
            (_, typ, size, fixed) if is_built_in_single(typ) && size > 0 && fixed => format!("xdr.varArray(xdr.{}(), {})", typ, size),
            (_, typ, size, _) if is_built_in_single(typ) && size > 0 => format!("xdr.array(xdr.{}(), {})", typ, size),
            (_, typ, size, _) if !is_array_type(typ) && size == 0 => format!("xdr.lookup(\"{}\")", typ),
            (_, typ, size, fixed) if !is_array_type(typ) && size > 0 && fixed => format!("xdr.varArray(xdr.lookup(\"{}\"), {})", typ, size),
            (_, typ, size, _) if !is_array_type(typ) && size > 0 => format!("xdr.array(xdr.lookup(\"{}\"), {})", typ, size),
            _ => format!("xdr.{}({})", typ, size)
        });
        handlebars_helper!(isvoid: |x: str| x == "");
        reg.register_helper("isvoid", Box::new(isvoid));
        reg.register_helper("typeconv", Box::new(typeconv));
        let result = reg
            .render_template(file_t.into_boxed_str().as_ref(), &processed)
            .unwrap();

        return Ok(result);
    }

    fn language(&self) -> String {
        "go".to_string()
    }
}
