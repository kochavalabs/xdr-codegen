use super::*;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

static HEADER: &str = r#"
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
{{macro-use}}
#[allow(unused_imports)]
use xdr_rs_serialize::de::{
    read_fixed_array, read_fixed_array_json, read_fixed_opaque, read_fixed_opaque_json,
    read_var_array, read_var_array_json, read_var_opaque, read_var_opaque_json, read_var_string,
    read_var_string_json, XDRIn,
};
use xdr_rs_serialize::error::Error;
#[allow(unused_imports)]
use xdr_rs_serialize::ser::{
    write_fixed_array, write_fixed_array_json, write_fixed_opaque, write_fixed_opaque_json,
    write_var_array, write_var_array_json, write_var_opaque, write_var_opaque_json,
    write_var_string, write_var_string_json, XDROut,
};
#[allow(unused_imports)]
use std::io::Write;

extern crate json;

{{#each this as |ns| ~}}
// Namespace start {{ns.name}}
"#;

static TYPEDEFS_T: &str = r#"
// Start typedef section

{{#each ns.typedefs as |td| ~}}
#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct {{td.def.name}} {
{{#if td.def.array_size}}
{{#if td.def.fixed_array}}
  #[array(fixed = {{td.def.array_size}})]
{{else}}
  #[array(var = {{td.def.array_size}})]
{{/if}}
  pub t: {{#if (neqstr td.def.type_name) }}Vec<{{td.def.type_name}}>{{else}} {{td.def.type_name}} {{/if}},
{{else}}
  pub t:  {{td.def.type_name}},
{{/if}}
}
{{/each}}
// End typedef section
"#;

static STRUCTS_T: &str = r#"
// Start struct section
{{#each ns.structs as |st|}}

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct {{st.name}} {
{{#each st.props as |prop|}}
{{#if prop.array_size}}
{{#if prop.fixed_array}}
  #[array(fixed = {{prop.array_size}})]
{{else}}
  #[array(var = {{prop.array_size}})]
{{/if}}
  pub {{prop.name}}: {{#if (neqstr prop.type_name) }}Vec<{{prop.type_name}}>{{else}} {{prop.type_name}} {{/if}},
{{else}}
  pub {{prop.name}}:  {{prop.type_name}},
{{/if}}
{{/each~}}
}
{{/each}}
// End struct section
"#;

static ENUM_T: &str = r#"
{{#each ns.enums as |enum|}}
#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum {{enum.name}} {
{{#each enum.values as |val|~}}
    {{val.name}} = {{val.index}},
{{/each~}}
}

impl Default for {{enum.name}} {
    fn default() -> Self {
        {{enum.name}}::{{enum.values.0.name}}
    }
}
{{/each~}}
"#;

static UNION_T: &str = r#"
// Start union section

{{#each ns.unions as |uni|}}
#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum {{uni.name}} {
{{#each uni.switch.cases as |case|}}
{{#if (not (isvoid case.ret_type.name))}}
    {{#if (eqstr case.ret_type.type_name)}}
        {{case.value}}({{case.ret_type.type_name}}),
    {{else}} {{#if case.ret_type.array_size}}
        {{#if case.ret_type.fixed_array}}
            #[array(fixed = {{case.ret_type.array_size}})]
        {{else}}
            #[array(var = {{case.ret_type.array_size}})]
        {{/if}}
        {{case.value}}(Vec<{{case.ret_type.type_name}}>),
    {{else}}
        {{case.value}}({{case.ret_type.type_name}}),
    {{/if}} {{/if}}
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
      {{uni.name}}::{{uni.switch.cases.0.value}}(())
    {{/if}}
    }
}
{{/each~}}
// End union section
"#;

static FOOTER: &str = r#"
// Namespace end {{ns.name}}
{{/each~}}"#;

fn build_file_template() -> String {
    format!("{}{}{}{}{}{}", HEADER, TYPEDEFS_T, STRUCTS_T, ENUM_T, UNION_T, FOOTER)
}

pub struct RustGenerator {
    pub include_macro: bool,
}

fn process_namespaces(namespaces: Vec<Namespace>) -> Result<Vec<Namespace>, &'static str> {
    let mut type_map = HashMap::new();
    type_map.insert("boolean", "bool");
    type_map.insert("opaque", "u8");
    type_map.insert("int", "i32");
    type_map.insert("unsigned int", "u32");
    type_map.insert("hyper", "i64");
    type_map.insert("unsigned hyper", "u64");
    type_map.insert("float", "f32");
    type_map.insert("double", "f64");
    type_map.insert("string", "String");
    apply_type_map(namespaces, &type_map)
}

impl CodeGenerator for RustGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let mut reg = Handlebars::new();
        let file_t = build_file_template();
        handlebars_helper!(neqstr: |x: str| x != "String");
        handlebars_helper!(eqstr: |x: str| x == "String");
        handlebars_helper!(isvoid: |x: str| x == "");
        reg.register_helper("neqstr", Box::new(neqstr));
        reg.register_helper("eqstr", Box::new(eqstr));
        reg.register_helper("isvoid", Box::new(isvoid));
        reg.register_helper(
            "macro-use",
            Box::new(
                |_h: &Helper, _r: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut dyn Output| -> HelperResult {
                    if self.include_macro {
                        out.write(
                            "#[macro_use]
extern crate xdr_rs_serialize_derive;",
                        )?;
                    }
                    Ok(())
                },
            ),
        );
        let processed_ns = process_namespaces(namespaces)?;
        let result = reg.render_template(file_t.into_boxed_str().as_ref(), &processed_ns).unwrap();

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn without_macro() {
        let input_test = vec![Namespace {
            enums: Vec::new(),
            structs: Vec::new(),
            typedefs: Vec::new(),
            unions: Vec::new(),
            name: String::from("test"),
        }];
        let res = RustGenerator { include_macro: false }.code(input_test);
        assert!(res.is_ok());
        let generated_code = res.unwrap();
        println!("{}", generated_code);
        assert!(!generated_code.contains("#[macro_use]"));
    }

    #[test]
    fn with_macro() {
        let input_test = vec![Namespace {
            enums: Vec::new(),
            structs: Vec::new(),
            typedefs: Vec::new(),
            unions: Vec::new(),
            name: String::from("test"),
        }];
        let res = RustGenerator { include_macro: true }.code(input_test);
        assert!(res.is_ok());
        let generated_code = res.unwrap();
        println!("{}", generated_code);
        assert!(generated_code.contains(
            "#[macro_use]
extern crate xdr_rs_serialize_derive;"
        ));
    }
}
