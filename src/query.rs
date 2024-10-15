use crate::interpreter::RuntimeType;
use crate::lexer;
use crate::lexer::Token;
use crate::parser::parse_ast;
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Query {
    tokens: VecDeque<Token>,
}

impl Query {
    pub fn new(query: &str) -> Self {
        let tokens = lexer::tokenize(query);
        Self { tokens }
    }

    pub fn eval(&self, obj: &Value) -> bool {
        let mut tokens = self.tokens.clone();
        let ast = parse_ast(&mut tokens);
        if let RuntimeType::Bool(result) = interpreter::eval_ast_stmt(obj, &ast) {
            result
        } else {
            false
        }
    }
}
