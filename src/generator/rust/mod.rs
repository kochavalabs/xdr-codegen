use super::*;
use handlebars::Handlebars;

static HEADER: &str = r#"
use rust_xdr::de::from_bytes;
use rust_xdr::error::Error;
use rust_xdr::ser::to_bytes;

pub trait XDR<T = Self> {
    fn to_xdr(&self) -> Result<Vec<u8>, Error>;
    fn from_xdr(&self, xdr: &[u8]) -> Result<T, Error>;
}

{{#each this as |ns| ~}}
// Namspace start {{ns.name}}
"#;

static TYPEDEFS_T: &str = r#"
// Start typedef section

{{#each ns.typedefs as |td| ~}}
#[derive(Default, Debug, Serialize, Deserialize)]
{{#if td.def.array_size~}}
{{#if td.def.fixed_array~}}
type {{td.def.name}} = {{#if (neqstr td.def.type_name) }}[{{td.def.type_name}}; {{td.def.array_size}}]{{/if}};
{{else~}}
type {{td.def.name}} = {{#if (neqstr td.def.type_name) }}Vec<{{td.def.type_name}}>{{else}} {{td.def.type_name}} {{/if}};
{{/if~}}
{{/if}}

impl XDR for {{td.def.name}} {
    fn to_xdr(&self) -> Result<Vec<u8>, Error> {
        to_bytes(&self)
    }

    fn from_xdr(&self, xdr: &[u8]) -> Result<{{td.def.name}}, Error> {
        from_bytes(xdr)
    }
}

{{/each}}
// End typedef section
"#;

static STRUCTS_T: &str = r#"
// Start struct section
{{#each ns.structs as |st|}}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct {{st.name}} {
{{#each st.props as |prop|}}
{{#if prop.array_size}}
{{#if prop.fixed_array}}
  {{prop.name}}: {{#if (neqstr prop.type_name) }}[{{prop.type_name}}; {{prop.array_size}}]{{/if}},
{{else}}
  {{prop.name}}: {{#if (neqstr prop.type_name) }}Vec<{{prop.type_name}}>{{else}} {{prop.type_name}} {{/if}},
{{/if}}
{{else}}
  {{prop.name}}:  {{prop.type_name}},
{{/if}}
{{/each~}}
}

impl XDR for {{st.name}} {
    fn to_xdr(&self) -> Result<Vec<u8>, Error> {
        to_bytes(&self)
    }

    fn from_xdr(&self, xdr: &[u8]) -> Result<{{st.name}}, Error> {
        from_bytes(xdr)
    }
}
{{/each}}
// End struct section
"#;

static ENUM_T: &str = r#"
{{#each ns.enums as |enum|}}
#[derive(Debug, Serialize, Deserialize)]
enum {{enum.name}} {
{{#each enum.values as |val|~}}
    {{val.name}},
{{/each~}}
}

impl Default for {{enum.name}} {
    fn default() -> Self {
        {{enum.name}}::{{enum.values.0.name}}
    }
}

impl XDR for {{enum.name}} {
    fn to_xdr(&self) -> Result<Vec<u8>, Error> {
        to_bytes(&self)
    }

    fn from_xdr(&self, xdr: &[u8]) -> Result<{{enum.name}}, Error> {
        from_bytes(xdr)
    }
}

{{/each~}}
"#;

static UNION_T: &str = r#"
// Start union section

{{#each ns.unions as |uni|}}
#[derive(Debug, Serialize, Deserialize)]
enum {{uni.name}} {
{{#each uni.switch.cases as |case|}}
{{#if (not (isvoid case.ret_type.name))}}
  {{case.value}}({{case.ret_type.type_name}}),
{{else}}
  {{case.value}}(()),
{{/if}}
{{/each~}}
}

impl Default for {{uni.name}} {
    fn default() -> Self {
    {{#if (not (isvoid uni.switch.cases.0.ret_type.name))}}
      {{uni.name}}::{{uni.switch.cases.0.value}}({{uni.switch.cases.0.ret_type.type_name}}::default())
    {{else}}
      {{uni.name}}::{{uni.switch.cases.0.value}}
    {{/if}}
    }
}

impl XDR for {{uni.name}} {
    fn to_xdr(&self) -> Result<Vec<u8>, Error> {
        to_bytes(&self)
    }

    fn from_xdr(&self, xdr: &[u8]) -> Result<{{uni.name}}, Error> {
        from_bytes(xdr)
    }
}
{{/each~}}
// End union section
"#;

static FOOTER: &str = r#"
// Namspace end {{ns.name}}
{{/each~}}"#;

fn build_file_template() -> String {
    format!(
        "{}{}{}{}{}{}",
        HEADER, TYPEDEFS_T, STRUCTS_T, ENUM_T, UNION_T, FOOTER
    )
}

pub struct RustGenerator {}

fn process_namespaces(namespaces: Vec<Namespace>) -> Result<Vec<Namespace>, &'static str> {
    let mut type_map = HashMap::new();
    type_map.insert("boolean", "bool");
    type_map.insert("opaque", "u8");
    type_map.insert("integer", "i32");
    type_map.insert("unsigned integer", "u32");
    type_map.insert("hyper", "i64");
    type_map.insert("unsigned hyper", "u64");
    type_map.insert("float", "f32");
    type_map.insert("double", "f64");
    type_map.insert("string", "String");
    apply_type_map(namespaces, type_map)
}

impl CodeGenerator for RustGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let mut reg = Handlebars::new();
        let file_t = build_file_template();
        handlebars_helper!(neqstr: |x: str| x != "String");
        handlebars_helper!(isvoid: |x: str| x == "");
        reg.register_helper("neqstr", Box::new(neqstr));
        reg.register_helper("isvoid", Box::new(isvoid));
        let processed_ns = process_namespaces(namespaces)?;
        let result = reg
            .render_template(file_t.into_boxed_str().as_ref(), &processed_ns)
            .unwrap();

        return Ok(result);
    }

    fn language(&self) -> String {
        "rust".to_string()
    }
}
