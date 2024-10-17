mod interpreter;
mod lexer;
mod parser;

use crate::query::parser::ASTNode;
use interpreter::RuntimeType;
use serde_json::Value;

#[derive(Debug)]
pub struct Query {
    ast: ASTNode,
}

impl Query {
    pub fn new(query: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut tokens = lexer::tokenize(query);
        let ast = parser::parse_ast(&mut tokens)?;
        Ok(Self { ast })
    }

    pub fn eval(&self, obj: &Value) -> Result<bool, Box<dyn std::error::Error>> {
        if let RuntimeType::Bool(result) = interpreter::eval_ast_stmt(obj, &self.ast)? {
            Ok(result)
        } else {
            Ok(false)
        }
    }
}
