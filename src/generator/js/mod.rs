use super::*;
use handlebars::Handlebars;
use std::collections::HashMap;

static HEADER: &str = r#"
import types from 'js-xdr'

{{#each this as |ns| ~}}
// Namspace start {{ns.name}}
"#;

static TYPEDEFS_T: &str = r#"
// Start typedef section
{{#each ns.typedefs as |td|}}
export function {{td.def.name}}() {
    return {{#typeconv td.def.name td.def.type_name td.def.array_size td.def.fixed_array}}{{/typeconv}}
}
{{/each~}}
// End typedef section
"#;

static STRUCTS_T: &str = r#"
// Start struct section
{{#each ns.structs as |st| ~}}
export function {{st.name}}() {
    return new types.Struct(
        [{{#each st.props as |prop| ~}}"{{prop.name}}",{{/each ~}}],
        [{{#each st.props as |prop| ~}}{{#typeconv prop.name prop.type_name prop.array_size prop.fixed_array}}{{/typeconv}},{{/each ~}}]
    )
}
{{/each}}
// End struct section
"#;

static ENUM_T: &str = r#"
// Start enum section
{{#each ns.enums as |enum|}}
export function {{enum.name}}() {
    return new types.Enum(
        {{#each enum.values as |val| ~}}
          {{val.index}}: {{val.name}},
        {{/each}}
    )
}
{{/each}}

// End enum section
"#;

static UNION_T: &str = r#"
// Start union section

{{#each ns.unions as |uni|}}
export function {{uni.name}}() {
    return new types.Union(
        {{uni.switch.enum_type}}(),
        {
            {{#each uni.switch.cases as |case|~}}
                {{#if (not (isvoid case.ret_type.name))}}
                    "{{case.value}}":  {{#typeconv case.ret_type.name case.ret_type.type_name case.ret_type.array_size case.ret_type.fixed_array}}{{/typeconv}},
                {{else}}
                    "{{case.value}}": new types.Void(),
                {{/if}}
            {{/each}}
        }
    )
}
{{/each}}
// End union section
"#;

static FOOTER: &str = r#"
// End namespace {{ns.name}}
{{/each~}}
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
        "Str" => true,
        "opaque" => true,
        _ => false,
    }
}

fn is_built_in(def_type: &str) -> bool {
    match def_type {
        "Void" => true,
        "Bool" => true,
        "Int" => true,
        "Hyper" => true,
        "UInt" => true,
        "UHyper" => true,
        "Float" => true,
        "Double" => true,
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
        type_map.insert("boolean", "Bool");
        type_map.insert("int", "Int");
        type_map.insert("unsigned int", "UInt");
        type_map.insert("unsigned hyper", "UHyper");
        type_map.insert("hyper", "Hyper");
        type_map.insert("string", "Str");
        type_map.insert("float", "Float");
        type_map.insert("double", "Double");
        type_map.insert("void", "Void");
        let processed = apply_type_map(namespaces, type_map)?;
        let mut reg = Handlebars::new();
        let file_t = build_file_template();
        handlebars_helper!(typeconv: |name: str, typ: str, size: i64, fixed: bool| match (name, typ, size, fixed) {
            (_, "opaque", _, false) => format!("new types.VarOpaque({})", size),
            (_, "opaque", _, true) => format!("new types.FixedOpaque({})", size),
            (_, typ, size, false) if is_built_in_single(typ) && size > 0 => format!("new types.VarArray({}, () => new {}())", size, typ),
            (_, typ, size, false) if !is_array_type(typ) && size > 0 => format!("new types.VarArray({}, {})", size, typ),
            (_, typ, size, _) if is_built_in_single(typ) && size == 0 => format!("new types.{}()", typ),
            (_, typ, size, _) if is_built_in_single(typ) && size > 0 => format!("new types.FixedArray({}, () => new {}())", size, typ),
            (_, typ, size, _) if !is_array_type(typ) && size == 0 => format!("{}()", typ),
            (_, typ, size, _) if !is_array_type(typ) && size > 0 => format!("new types.FixedArray({}, {})", size, typ),
            (_, typ, size, _) if !is_array_type(typ) && size > 0 => format!("new types.FixedArray({}, {})", size, typ),
            _ => format!("new types.{}({})", typ, size)
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
        "js".to_string()
    }
}
