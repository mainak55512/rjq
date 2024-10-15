mod interpreter;
mod lexer;
mod parser;

use crate::query::parser::ASTNode;
use interpreter::RuntimeType;
use parser::parse_ast;
use serde_json::Value;

#[derive(Debug)]
pub struct Query {
    ast: ASTNode,
}

impl Query {
    pub fn new(query: &str) -> Self {
        let mut tokens = lexer::tokenize(query);
        let ast = parse_ast(&mut tokens);
        Self { ast }
    }

    pub fn eval(&self, obj: &Value) -> bool {
        if let RuntimeType::Bool(result) = interpreter::eval_ast_stmt(obj, &self.ast) {
            result
        } else {
            false
        }
    }
}
