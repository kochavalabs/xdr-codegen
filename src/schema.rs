use mazzaroth_xdr::Schema;
use super::ast::Namespace;


pub fn generate_schema(namespaces: Vec<Namespace>) -> Result<Schema, &'static str> {
    Ok(Schema::default())
}
