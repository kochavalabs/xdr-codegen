use super::*;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError};
use std::collections::HashMap;

static HEADER: &str = r#"
// Package xdr is automatically generated
// DO NOT EDIT or your changes may be overwritten
package xdr

import (
  "bytes"
  "encoding"
  "encoding/json"
  "io"
  "fmt"

  "github.com/stellar/go-xdr/xdr3"
)

// Unmarshal reads an xdr element from `r` into `v`.
func Unmarshal(r io.Reader, v interface{}) (int, error) {
  // delegate to xdr package's Unmarshal
      return xdr.Unmarshal(r, v)
}

// Marshal writes an xdr element `v` into `w`.
func Marshal(w io.Writer, v interface{}) (int, error) {
  // delegate to xdr package's Marshal
  return xdr.Marshal(w, v)
}
{{#each this as |ns| ~}}
// Namspace start {{ns.name}}
"#;

static TYPEDEFS_T: &str = r#"
// Start typedef section

{{#each ns.typedefs as |td| ~}}
{{#if td.def.array_size}}
{{#if td.def.fixed_array}}
// {{td.def.name}} generated typedef
type {{td.def.name}} {{#if (neqstr td.def.type_name) }}[{{td.def.array_size}}]{{/if}}{{td.def.type_name}}
// XDRMaxSize implements the Sized interface for {{td.def.name}}
func (s {{td.def.name}}) XDRMaxSize() int {
  return {{td.def.array_size}}
}
{{else}}
// {{td.def.name}} generated typedef
type {{td.def.name}} {{#if (neqstr td.def.type_name) }}[]{{/if}}{{td.def.type_name}}
{{/if}}
{{/if}}

// MarshalBinary implements encoding.BinaryMarshaler.
func (s {{td.def.name}}) MarshalBinary() ([]byte, error) {
  b := new(bytes.Buffer)
  _, err := Marshal(b, s)
  return b.Bytes(), err
}

// UnmarshalBinary implements encoding.BinaryUnmarshaler.
func (s *{{td.def.name}}) UnmarshalBinary(inp []byte) error {
  _, err := Unmarshal(bytes.NewReader(inp), s)
  return err
}

var (
  _ encoding.BinaryMarshaler   = (*{{td.def.name}})(nil)
  _ encoding.BinaryUnmarshaler = (*{{td.def.name}})(nil)
)
{{/each~}}
// End typedef section
"#;

static STRUCTS_T: &str = r#"
// Start struct section

{{#each ns.structs as |st| ~}}

// {{st.name}} generated struct
type {{st.name}} struct {
{{#each st.props as |prop|}}
{{#if (eqstr prop.type_name)}}
  {{#if (and (ne prop.array_size 0) (ne prop.array_size 2147483647))}}
    {{prop.name}} string `xdrmaxsize:"{{prop.array_size}}" json:"{{lower prop.name}}"`
  {{else}}
    {{prop.name}} string `json:"{{lower prop.name}}"`
  {{/if}}
{{else}} {{#if prop.fixed_array}}
  {{#if (eqstruct prop.type_name)}}
    {{prop.name}} [{{prop.array_size}}]*{{prop.type_name}} `json:"{{lower prop.name}}"`
  {{else}}
    {{prop.name}} [{{prop.array_size}}]{{prop.type_name}} `json:"{{lower prop.name}}"`
  {{/if}}
{{else}} {{#if prop.array_size}}
  {{#if (ne prop.array_size 2147483647)}}
    {{#if (eqstruct prop.type_name)}}
      {{prop.name}} []*{{prop.type_name}} `xdrmaxsize:"{{prop.array_size}}" json:"{{lower prop.name}}"`
    {{else}}
      {{prop.name}} []{{prop.type_name}} `xdrmaxsize:"{{prop.array_size}}" json:"{{lower prop.name}}"`
    {{/if}}
  {{else}}
    {{#if (eqstruct prop.type_name)}}
      {{prop.name}} []*{{prop.type_name}} `json:"{{lower prop.name}}"`
    {{else}}
      {{prop.name}} []{{prop.type_name}} `json:"{{lower prop.name}}"`
    {{/if}}
  {{/if}}
{{else}} {{#if (bignum prop.type_name)}}
  {{prop.name}} {{prop.type_name}} `json:"{{lower prop.name}},string"`
{{else}} {{#if (eqstruct prop.type_name)}}
  {{prop.name}} *{{prop.type_name}} `json:"{{lower prop.name}}"`
{{else}}
  {{prop.name}} {{prop.type_name}} `json:"{{lower prop.name}}"`
{{/if}}
{{/if}}
{{/if}}
{{/if}}
{{/if}}
{{/each~}}
}

// MarshalBinary implements encoding.BinaryMarshaler.
func (s {{st.name}}) MarshalBinary() ([]byte, error) {
  b := new(bytes.Buffer)
  _, err := Marshal(b, s)
  return b.Bytes(), err
}

// UnmarshalBinary implements encoding.BinaryUnmarshaler.
func (s *{{st.name}}) UnmarshalBinary(inp []byte) error {
  _, err := Unmarshal(bytes.NewReader(inp), s)
  return err
}

var (
  _ encoding.BinaryMarshaler   = (*{{st.name}})(nil)
  _ encoding.BinaryUnmarshaler = (*{{st.name}})(nil)
)

{{/each~}}
// End struct section
"#;

static ENUM_T: &str = r#"
// Start enum section

{{#each ns.enums as |enum|}}
// {{enum.name}} generated enum
type {{enum.name}} int32
const (
{{#each enum.values as |val|}}
  // {{enum.name}}{{val.name}} enum value {{val.index}}
  {{enum.name}}{{val.name}} {{enum.name}} = {{val.index}}
{{/each~}}
)
// {{enum.name}}Map generated enum map
var {{enum.name}}Map = map[int32]string{
{{#each enum.values as |val|}}
  {{val.index}}: "{{enum.name}}{{val.name}}",
{{/each~}}
}

// ValidEnum validates a proposed value for this enum.  Implements
// the Enum interface for {{enum.name}}
func (s {{enum.name}}) ValidEnum(v int32) bool {
  _, ok := {{enum.name}}Map[v]
  return ok
}
// String returns the name of `e`
func (s {{enum.name}}) String() string {
  name := {{enum.name}}Map[int32(s)]
  return name
}

// MarshalBinary implements encoding.BinaryMarshaler.
func (s {{enum.name}}) MarshalBinary() ([]byte, error) {
  b := new(bytes.Buffer)
  _, err := Marshal(b, s)
  return b.Bytes(), err
}

// UnmarshalBinary implements encoding.BinaryUnmarshaler.
func (s *{{enum.name}}) UnmarshalBinary(inp []byte) error {
  _, err := Unmarshal(bytes.NewReader(inp), s)
  return err
}

var (
  _ encoding.BinaryMarshaler   = (*{{enum.name}})(nil)
  _ encoding.BinaryUnmarshaler = (*{{enum.name}})(nil)
)
{{/each~}}
// End enum section
"#;

static UNION_T: &str = r#"
// Start union section

{{#each ns.unions as |uni|}}
// {{uni.name}} generated union
type {{uni.name}} struct{
  {{uni.switch.enum_name}} {{uni.switch.enum_type}}
{{#each uni.switch.cases as |case|}}
{{#if (not (isvoid case.ret_type.name))}}
    {{#if (eqstr case.ret_type.type_name)}}
        {{case.ret_type.name}} *{{case.ret_type.type_name}}
    {{else}} {{#if case.ret_type.array_size}}
        {{case.ret_type.name}} *[]{{case.ret_type.type_name}}
    {{else}}
        {{case.ret_type.name}} *{{case.ret_type.type_name}}
    {{/if}} {{/if}}
{{/if}}
{{/each~}}
}

// SwitchFieldName returns the field name in which this union's
// discriminant is stored
func (u {{uni.name}}) SwitchFieldName() string {
  return "{{uni.switch.enum_name}}"
}

// ArmForSwitch returns which field name should be used for storing
// the value for an instance of {{uni.name}}
func (u {{uni.name}}) ArmForSwitch(sw int32) (string, bool) {
switch {{uni.switch.enum_type}}(sw) {
{{#each uni.switch.cases as |case|}}
  case {{uni.switch.enum_type}}{{case.value}}:
    return "{{case.ret_type.name}}", true
{{/each~}}
}
return "-", false
}

// New{{uni.name}} creates a new  {{uni.name}}.
func New{{uni.name}}(aType {{uni.switch.enum_type}}, value interface{}) (result {{uni.name}}, err error) {
  result.Type = aType
switch {{uni.enum_type}}(aType) {
{{#each uni.switch.cases as |case|}}
  case {{uni.switch.enum_type}}{{case.value}}:
{{#if (not (isvoid case.ret_type.name))}}
    {{#if (eqstr case.ret_type.type_name)}}
        tv, ok := value.({{case.ret_type.type_name}})
    {{else}}{{#if case.ret_type.array_size}}
        tv, ok := value.([]{{case.ret_type.type_name}})
    {{else}}
        tv, ok := value.({{case.ret_type.type_name}})
    {{/if}} {{/if}}
    if !ok {
        err = fmt.Errorf("invalid value, must be {{case.ret_type}}")
        return
    }
    result.{{case.ret_type.name}} = &tv
{{/if}}
{{/each~}}
}
  return
}

{{#each uni.switch.cases as |case|}}
{{#if (not (isvoid case.ret_type.name))}}
// Must{{case.ret_type.name}} retrieves the {{case.ret_type.name}} value from the union,
// panicing if the value is not set.
    {{#if (eqstr case.ret_type.type_name)}} func (u {{uni.name}}) Must{{case.ret_type.name}}() {{case.ret_type.type_name}} {
    {{else}}{{#if case.ret_type.array_size}} func (u {{uni.name}}) Must{{case.ret_type.name}}() []{{case.ret_type.type_name}} {
    {{else}} func (u {{uni.name}}) Must{{case.ret_type.name}}() {{case.ret_type.type_name}} {
    {{/if}} {{/if}}
  val, ok := u.Get{{case.ret_type.name}}()
  if !ok {
    panic("arm {{case.ret_type.name}} is not set")
  }

  return val
}

// Get{{case.ret_type.name}} retrieves the {{case.ret_type.name}} value from the union,
// returning ok if the union's switch indicated the value is valid.
    {{#if (eqstr case.ret_type.type_name)}} func (u {{uni.name}}) Get{{case.ret_type.name}}() (result {{case.ret_type.type_name}}, ok bool) {
    {{else}}{{#if case.ret_type.array_size}} func (u {{uni.name}}) Get{{case.ret_type.name}}() (result []{{case.ret_type.type_name}}, ok bool) {
    {{else}} func (u {{uni.name}}) Get{{case.ret_type.name}}() (result {{case.ret_type.type_name}}, ok bool) {
    {{/if}}{{/if}}
  armName, _ := u.ArmForSwitch(int32(u.Type))

  if armName == "{{case.ret_type.name}}" {
    result = *u.{{case.ret_type.name}}
    ok = true
  }

  return
}
{{/if}}
{{/each~}}

// MarshalBinary implements encoding.BinaryMarshaler.
func (u {{uni.name}}) MarshalBinary() ([]byte, error) {
  b := new(bytes.Buffer)
  _, err := Marshal(b, u)
  return b.Bytes(), err
}

// UnmarshalBinary implements encoding.BinaryUnmarshaler.
func (u *{{uni.name}}) UnmarshalBinary(inp []byte) error {
  _, err := Unmarshal(bytes.NewReader(inp), u)
  return err
}

var (
  _ encoding.BinaryMarshaler   = (*{{uni.name}})(nil)
  _ encoding.BinaryUnmarshaler = (*{{uni.name}})(nil)
)

// MarshalJSON implements json.Marshaler.
func (u {{uni.name}}) MarshalJSON() ([]byte, error) {
  temp := struct {
		Type int32       `json:"type"`
		Data interface{} `json:"data"`
	}{}

  temp.Type = int32(u.Type)
  temp.Data = ""
  switch u.Type {
  {{#each uni.switch.cases as |case|}} case {{uni.switch.enum_type}}{{case.value}}:
    {{#if (not (isvoid case.ret_type.name))}} temp.Data = u.{{case.ret_type.name}}
    {{/if}}{{/each~}}
  default:
      return nil, fmt.Errorf("invalid union type")
  }

  return json.Marshal(temp)
}

// UnmarshalJSON implements json.Unmarshaler.
func (u *{{uni.name}}) UnmarshalJSON(data []byte) error {
  temp := struct {
		Type int32 `json:"type"`
	}{}
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}

  u.Type = {{uni.switch.enum_type}}(temp.Type)
	switch u.Type {
  {{#each uni.switch.cases as |case|}} case {{uni.switch.enum_type}}{{case.value}}:
    {{#if (not (isvoid case.ret_type.name))}} response := struct {
      {{#if (eqstr case.ret_type.type_name)}}
        {{case.ret_type.name}} {{case.ret_type.type_name}} `json:"data"`
      {{else}} {{#if case.ret_type.array_size}}
          {{case.ret_type.name}} []{{case.ret_type.type_name}} `json:"data"`
               {{else}}
          {{case.ret_type.name}} {{case.ret_type.type_name}} `json:"data"`
               {{/if}}
      {{/if}}
      }{}
      err := json.Unmarshal(data, &response)
      if err != nil {
        return err
      }
      u.{{case.ret_type.name}} = &response.{{case.ret_type.name}}
    {{/if}}
  {{/each~}}
  default:
    return fmt.Errorf("invalid union type")
  }

  return nil
}

{{/each~}}
// End union section
"#;

static FOOTER: &str = r#"
// Namespace end {{ns.name}}
{{/each~}}
var fmtTest = fmt.Sprint("this is a dummy usage of fmt")
"#;

#[derive(Debug, Default)]
pub struct GoGenerator {}

fn build_file_template() -> String {
    format!("{}{}{}{}{}{}", HEADER, TYPEDEFS_T, STRUCTS_T, ENUM_T, UNION_T, FOOTER)
}

fn process_namespaces(namespaces: Vec<Namespace>) -> Result<Vec<Namespace>, &'static str> {
    let mut type_map = HashMap::new();
    type_map.insert("boolean", "bool");
    type_map.insert("opaque", "byte");
    type_map.insert("int", "int32");
    type_map.insert("unsigned int", "uint32");
    type_map.insert("hyper", "int64");
    type_map.insert("unsigned hyper", "uint64");
    type_map.insert("float", "float32");
    type_map.insert("double", "float64");
    let mut ret_val = apply_type_map(namespaces, &type_map)?;
    for namespace in &mut ret_val {
        for typedef_ in &mut namespace.typedefs {
            typedef_.def.name = typedef_
                .def
                .name
                .chars()
                .enumerate()
                .map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c })
                .collect();
        }
        for struct_ in &mut namespace.structs {
            for prop in &mut struct_.props {
                prop.name = prop
                    .name
                    .chars()
                    .enumerate()
                    .map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c })
                    .collect();
            }
        }

        for union_ in &mut namespace.unions {
            for switch_case in &mut union_.switch.cases {
                switch_case.ret_type.name = switch_case
                    .ret_type
                    .name
                    .chars()
                    .enumerate()
                    .map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c })
                    .collect();
            }
        }
    }

    Ok(ret_val)
}

fn to_first_lower(value: &str) -> String {
    let mut c = value.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

// implement by a structure impls HelperDef
#[derive(Clone)]
struct StructHelper {
    struct_map: HashMap<String, bool>,
}

impl HelperDef for StructHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h
            .param(0)
            .ok_or_else(|| RenderError::new("Param not found for helper \"eqstruct\""))?;

        // Remove beginning and ending quote from json string to get key
        let input = &param.value().to_string();
        let key = &input[1..input.len() - 1];

        if self.struct_map.contains_key(key) {
            out.write("true")?;
        }

        Ok(())
    }
}

impl CodeGenerator for GoGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let mut reg = Handlebars::new();
        let file_t = build_file_template();

        // Get a map of struct names used
        let mut struct_map: HashMap<String, bool> = HashMap::new();
        for namespace in namespaces.clone() {
            for struct_ in namespace.structs {
                struct_map.insert(struct_.name, true);
            }
        }

        let struct_helper = StructHelper { struct_map: struct_map };

        handlebars_helper!(neqstr: |x: str| x != "string");
        handlebars_helper!(eqstr: |x: str| x == "string");
        handlebars_helper!(bignum: |x: str| x == "uint64" || x =="int64");
        handlebars_helper!(isvoid: |x: str| x == "");
        handlebars_helper!(lower: |x: str| to_first_lower(x));
        reg.register_helper("neqstr", Box::new(neqstr));
        reg.register_helper("eqstr", Box::new(eqstr));
        reg.register_helper("eqstruct", Box::new(struct_helper));
        reg.register_helper("bignum", Box::new(bignum));
        reg.register_helper("isvoid", Box::new(isvoid));
        let processed_ns = process_namespaces(namespaces)?;
        reg.register_helper("lower", Box::new(lower));
        let result = reg.render_template(file_t.into_boxed_str().as_ref(), &processed_ns).unwrap();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_namespace() {
        let input_test = vec![Namespace {
            enums: Vec::new(),
            structs: Vec::new(),
            typedefs: Vec::new(),
            unions: Vec::new(),
            name: String::from("test"),
        }];
        let res = GoGenerator {}.code(input_test);
        assert!(res.is_ok());
    }
    #[test]
    fn typedef_namespace() {
        let input_test = vec![Namespace {
            enums: Vec::new(),
            structs: Vec::new(),
            typedefs: vec![Typedef {
                def: Def {
                    name: String::from("testt"),
                    type_name: String::from("string"),
                    array_size: 0,
                    fixed_array: false,
                    tag: String::new(),
                },
            }],
            unions: Vec::new(),
            name: String::from("test"),
        }];
        let res = GoGenerator {}.code(input_test);
        assert!(res.is_ok());
        let generated_code = res.unwrap();
        assert!(generated_code.contains("func (s Testt) MarshalBinary() ([]byte, error)"));
        assert!(generated_code.contains("func (s *Testt) UnmarshalBinary(inp []byte) error"));
    }
    #[test]
    fn struct_namespace() {
        let input_test = vec![Namespace {
            enums: Vec::new(),
            typedefs: Vec::new(),
            structs: vec![
                Struct {
                    name: String::from("TestStruct"),
                    props: vec![
                        Def {
                            name: String::from("stringTest"),
                            type_name: String::from("string"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("BooleanTest"),
                            type_name: String::from("boolean"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("float_test"),
                            type_name: String::from("float"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("int_test"),
                            type_name: String::from("int"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("unsigned_int_test"),
                            type_name: String::from("unsigned int"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("hyper_test"),
                            type_name: String::from("hyper"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("unsigned_hyper_test"),
                            type_name: String::from("unsigned hyper"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                            name: String::from("struct_test"),
                            type_name: String::from("TestStruct2"),
                            array_size: 0,
                            fixed_array: false,
                            tag: String::new(),
                        },
                        Def {
                          name: String::from("array_test"),
                          type_name: String::from("unsigned hyper"),
                          array_size: 5,
                          fixed_array: false,
                          tag: String::new(),
                        },
                        Def {
                          name: String::from("array_struct_test"),
                          type_name: String::from("TestStruct2"),
                          array_size: 5,
                          fixed_array: false,
                          tag: String::new(),
                       },
                       Def {
                        name: String::from("fixed_array_test"),
                        type_name: String::from("unsigned hyper"),
                        array_size: 5,
                        fixed_array: true,
                        tag: String::new(),
                      },
                      Def {
                        name: String::from("fixed_array_struct_test"),
                        type_name: String::from("TestStruct2"),
                        array_size: 5,
                        fixed_array: true,
                        tag: String::new(),
                     },
                    ],
                    tag: String::new(),
                },
                Struct {
                    name: String::from("TestStruct2"),
                    props: vec![Def {
                        name: String::from("stringTest"),
                        type_name: String::from("string"),
                        array_size: 0,
                        fixed_array: false,
                        tag: String::new(),
                    }],
                    tag: String::new(),
                },
            ],
            unions: Vec::new(),
            name: String::from("test"),
        }];
        let res = GoGenerator {}.code(input_test);
        assert!(res.is_ok());
        let generated_code = res.unwrap();
        println!("{}", generated_code);
        assert!(generated_code.contains("type TestStruct struct {"));
        assert!(generated_code.contains("StringTest string `json:\"stringTest\"`"));
        assert!(generated_code.contains("BooleanTest bool `json:\"booleanTest\"`"));
        assert!(generated_code.contains("Float_test float32 `json:\"float_test\"`"));
        assert!(generated_code.contains("Int_test int32 `json:\"int_test\"`"));
        assert!(generated_code.contains("Unsigned_int_test uint32 `json:\"unsigned_int_test\"`"));
        assert!(generated_code.contains("Hyper_test int64 `json:\"hyper_test,string\"`"));
        assert!(generated_code.contains("Unsigned_hyper_test uint64 `json:\"unsigned_hyper_test,string\"`"));
        assert!(generated_code.contains("Struct_test *TestStruct2 `json:\"struct_test\"`"));
        assert!(generated_code.contains("Array_test []uint64 `xdrmaxsize:\"5\" json:\"array_test\"`"));
        assert!(generated_code.contains("Array_struct_test []*TestStruct2 `xdrmaxsize:\"5\" json:\"array_struct_test\"`"));
        assert!(generated_code.contains("Fixed_array_test [5]uint64 `json:\"fixed_array_test\"`"));
        assert!(generated_code.contains("Fixed_array_struct_test [5]*TestStruct2 `json:\"fixed_array_struct_test\"`"));
    }
}
