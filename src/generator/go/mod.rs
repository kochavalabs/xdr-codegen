use super::*;
use handlebars::Handlebars;
use std::collections::HashMap;

static HEADER: &str = r#"
// Package xdr is automatically generated
// DO NOT EDIT or your changes may be overwritten
package xdr

import (
  "bytes"
  "encoding"
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
{{#if prop.array_size}}
{{#if prop.fixed_array}}
  {{prop.name}}  {{#if (neqstr prop.type_name) }}[{{prop.array_size}}]{{/if}}{{prop.type_name}}
{{else}}
  {{prop.name}} {{#if (neqstr prop.type_name)}}[]{{/if}}{{prop.type_name ~}}
  {{#if (ne prop.array_size 2147483647) }}  `xdrmaxsize:"{{prop.array_size}}"` {{/if}}
{{/if}}
{{else}}
  {{prop.name}}  {{prop.type_name}}
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
  name, _ := {{enum.name}}Map[int32(s)]
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
  {{case.ret_type.name}} *{{case.ret_type.type_name}}
{{/if}}
{{/each~}}
}

// SwitchFieldName returns the field name in which this union's
// discriminant is stored
func (s {{uni.name}}) SwitchFieldName() string {
  return "{{uni.switch.enum_name}}"
}

// ArmForSwitch returns which field name should be used for storing
// the value for an instance of {{uni.name}}
func (s {{uni.name}}) ArmForSwitch(sw int32) (string, bool) {
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
    tv, ok := value.({{case.ret_type.type_name}})
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
func (s {{uni.name}}) Must{{case.ret_type.name}}() {{case.ret_type.type_name}} {
  val, ok := s.Get{{case.ret_type.name}}()

  if !ok {
    panic("arm {{case.ret_type.name}} is not set")
  }

  return val
}

// Get{{case.ret_type.name}} retrieves the {{case.ret_type.name}} value from the union,
// returning ok if the union's switch indicated the value is valid.
func (s {{uni.name}}) Get{{case.ret_type.name}}() (result {{case.ret_type.type_name}}, ok bool) {
  armName, _ := s.ArmForSwitch(int32(s.Type))

  if armName == "{{case.ret_type.name}}" {
    result = *s.{{case.ret_type.name}}
    ok = true
  }

  return
}
{{/if}}
{{/each~}}

// MarshalBinary implements encoding.BinaryMarshaler.
func (s {{uni.name}}) MarshalBinary() ([]byte, error) {
  b := new(bytes.Buffer)
  _, err := Marshal(b, s)
  return b.Bytes(), err
}

// UnmarshalBinary implements encoding.BinaryUnmarshaler.
func (s *{{uni.name}}) UnmarshalBinary(inp []byte) error {
  _, err := Unmarshal(bytes.NewReader(inp), s)
  return err
}

var (
  _ encoding.BinaryMarshaler   = (*{{uni.name}})(nil)
  _ encoding.BinaryUnmarshaler = (*{{uni.name}})(nil)
)
{{/each~}}
// End union section
"#;

static FOOTER: &str = r#"
// Namspace end {{ns.name}}
{{/each~}}
var fmtTest = fmt.Sprint("this is a dummy usage of fmt")
"#;

#[derive(Debug, Default)]
pub struct GoGenerator {}

fn build_file_template() -> String {
    format!(
        "{}{}{}{}{}{}",
        HEADER, TYPEDEFS_T, STRUCTS_T, ENUM_T, UNION_T, FOOTER
    )
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
    let mut ret_val = apply_type_map(namespaces, type_map)?;
    for n_i in 0..ret_val.len() {
        for td_i in 0..ret_val[n_i].typedefs.len() {
            ret_val[n_i].typedefs[td_i].def.name = format!(
                "{}{}",
                &ret_val[n_i].typedefs[td_i].def.name[0..1].to_uppercase(),
                &ret_val[n_i].typedefs[td_i].def.name
                    [1..ret_val[n_i].typedefs[td_i].def.name.len()]
            );
        }
        for str_i in 0..ret_val[n_i].structs.len() {
            for st_def_i in 0..ret_val[n_i].structs[str_i].props.len() {
                ret_val[n_i].structs[str_i].props[st_def_i].name = format!(
                    "{}{}",
                    &ret_val[n_i].structs[str_i].props[st_def_i].name[0..1].to_uppercase(),
                    &ret_val[n_i].structs[str_i].props[st_def_i].name
                        [1..ret_val[n_i].structs[str_i].props[st_def_i].name.len()]
                );
            }
        }

        for uni_i in 0..ret_val[n_i].unions.len() {
            for case_i in 0..ret_val[n_i].unions[uni_i].switch.cases.len() {
                if ret_val[n_i].unions[uni_i].switch.cases[case_i]
                    .ret_type
                    .name
                    == "".to_string()
                {
                    continue;
                }
                ret_val[n_i].unions[uni_i].switch.cases[case_i]
                    .ret_type
                    .name = format!(
                    "{}{}",
                    &ret_val[n_i].unions[uni_i].switch.cases[case_i]
                        .ret_type
                        .name[0..1]
                        .to_uppercase(),
                    &ret_val[n_i].unions[uni_i].switch.cases[case_i]
                        .ret_type
                        .name[1..ret_val[n_i].unions[uni_i].switch.cases[case_i]
                        .ret_type
                        .name
                        .len()]
                );
            }
        }
    }

    Ok(ret_val)
}

impl CodeGenerator for GoGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let mut reg = Handlebars::new();
        let file_t = build_file_template();
        handlebars_helper!(neqstr: |x: str| x != "string");
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
        "go".to_string()
    }
}
