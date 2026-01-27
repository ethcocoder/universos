pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;

use anyhow::Result;

pub fn compile(source: &str) -> Result<Vec<u8>> {
    let mut parser = parser::Parser::new(source);
    let program = parser.parse()?;
    
    let mut codegen = codegen::CodeGen::new();
    codegen.generate(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_basic() {
        let source = r#"
            universe ping_pong {
                energy: 50.0;
                func start(x) {
                    return x + 1;
                }
                start(10);
            }
        "#;
        let bytecode = compile(source).unwrap();
        assert!(!bytecode.is_empty());
    }
}
