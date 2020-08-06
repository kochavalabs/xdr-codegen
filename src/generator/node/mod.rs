use super::*;
use std::sync::Arc;

use swc::{
    self,
    config::{Config, ModuleConfig, Options},
    Compiler,
};

use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};

#[derive(Debug, Default)]
pub struct NodeGenerator {}

impl CodeGenerator for NodeGenerator {
    fn code(&self, namespaces: Vec<Namespace>) -> Result<String, &'static str> {
        let gen = js::JsGenerator {};
        let cm = Arc::<SourceMap>::default();
        let handler = Arc::new(Handler::with_tty_emitter(
            ColorConfig::Auto,
            true,
            false,
            Some(cm.clone()),
        ));
        let c = Compiler::new(cm.clone(), handler);
        let src = gen.code(namespaces)?;
        let fm = cm.new_source_file(FileName::Real("input.js".into()), src.into());
        match c.process_js_file(
            fm,
            &Options {
                config: Some(Config {
                    module: Some(ModuleConfig::CommonJs(Default::default())),
                    ..Default::default()
                }),
                swcrc: false,
                is_module: true,
                ..Default::default()
            },
        ) {
            Ok(v) => {
                if c.handler.has_errors() {
                    Err("Error processing js handler")
                } else {
                    Ok(v.code.into())
                }
            }
            Err(_) => Err("Error processing js process"),
        }
    }

    fn language(&self) -> String {
        "commonjs".to_string()
    }
}
