use super::ast::*;

pub mod go;

pub trait CodeGenerator {
    fn code(&self, namespace: Vec<Namespace>) -> Result<String, &'static str>;

    fn language(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang() {
        let mut t_gen = BasicTemplateGenerator::default();
        t_gen.lang = "golang".to_string();
        assert_eq!(t_gen.language(), "golang");
    }

    #[test]
    fn test_code() {
        let mut t_gen = BasicTemplateGenerator::default();
        let expected = "\nheader.\n\nbody.\n\nfooter.\n";
        let ns: Vec<Namespace> = Vec::new();
        t_gen.header = "header.".to_string();
        t_gen.body_t = "body.".to_string();
        t_gen.footer = "footer.".to_string();

        assert_eq!(t_gen.code(ns).unwrap(), expected);
    }
}
