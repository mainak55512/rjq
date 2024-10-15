use std::collections::VecDeque;

use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum LiteralType {
    NumericLiteral,
    StringLiteral,
    BinaryOperator,
    BinaryExpr,
    LogicalExpr,
    NoneType,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    PrimarySymbol(PrimarySymbol),
    BinaryExpr(Box<BinaryExpr>),
    NoneType,
}

#[derive(Debug, Clone)]
pub struct PrimarySymbol {
    pub kind: LiteralType,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: LiteralType,
    pub left: ASTNode,
    pub right: ASTNode,
    pub operator: String,
}

fn parse_primary_expr(token_array: &mut VecDeque<Token>) -> ASTNode {
    let tk = &token_array[0].token_type;
    match tk {
        TokenType::Number => ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::NumericLiteral,
            symbol: token_array.pop_front().expect("NaN").val,
        }),
        TokenType::String => ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::StringLiteral,
            symbol: token_array.pop_front().expect("Invalid String").val,
        }),
        TokenType::Binary => ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::BinaryOperator,
            symbol: token_array.pop_front().expect("Invalid Operator").val,
        }),
        TokenType::Paren => {
            token_array.pop_front();
            let value = parse_ast(token_array);
            token_array.pop_front();
            value
        }
    }
}

fn parse_binary_expr(token_array: &mut VecDeque<Token>) -> ASTNode {
    let mut left = parse_primary_expr(token_array);
    while !token_array.is_empty()
        && (token_array[0].val == "="
            || token_array[0].val == ">="
            || token_array[0].val == "<="
            || token_array[0].val == ">"
            || token_array[0].val == "<"
            || token_array[0].val == "!=")
    {
        let operator = token_array.pop_front().expect("Invalid Operator").val;
        let right = parse_primary_expr(token_array);
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::BinaryExpr,
            left,
            right,
            operator,
        }))
    }
    left
}

pub fn parse_ast(tokens: &mut VecDeque<Token>) -> ASTNode {
    let mut left = parse_binary_expr(tokens);
    if !tokens.is_empty() && tokens[0].val != "&&" && tokens[0].val != "||" {
        println!("Query is invalid");
        std::process::exit(1);
    }
    while !tokens.is_empty() && (tokens[0].val == "&&" || tokens[0].val == "||") {
        let operator = tokens.pop_front().expect("Empty operator").val;
        if tokens.is_empty() {
            println!("Query is invalid");
            std::process::exit(1);
        }
        let right = parse_binary_expr(tokens);
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::LogicalExpr,
            left,
            right,
            operator,
        }))
    }
    left
}
