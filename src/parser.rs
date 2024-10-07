use std::collections::VecDeque;

use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum LiteralType {
    NUMERIC_LITERAL,
    STRING_LITERAL,
    BINARY_OPERATOR,
    BINARY_EXPR,
    LOGICAL_EXPR,
    NONE_TYPE,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    PrimarySymbol(PrimarySymbol),
    BinaryExpr(Box<BinaryExpr>),
    NONE_TYPE,
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
    let tk: &TokenType = &token_array[0].token_type;
    match tk {
        TokenType::NUMBER => {
            return ASTNode::PrimarySymbol(PrimarySymbol {
                kind: LiteralType::NUMERIC_LITERAL,
                symbol: token_array.pop_front().unwrap().val,
            })
        }
        TokenType::STRING => {
            return ASTNode::PrimarySymbol(PrimarySymbol {
                kind: LiteralType::STRING_LITERAL,
                symbol: token_array.pop_front().unwrap().val,
            })
        }
        TokenType::BINARY_OPERATOR => {
            return ASTNode::PrimarySymbol(PrimarySymbol {
                kind: LiteralType::BINARY_OPERATOR,
                symbol: token_array.pop_front().unwrap().val,
            })
        }
        TokenType::PAREN => {
            token_array.pop_front();
            let value = parse_ast(token_array);
            token_array.pop_front();
            return value;
        }
    }
}

fn parse_binary_expr(token_array: &mut VecDeque<Token>) -> ASTNode {
    let mut left = parse_primary_expr(token_array);
    while token_array.len() > 0
        && (token_array[0].val == "="
            || token_array[0].val == ">="
            || token_array[0].val == "<="
            || token_array[0].val == ">"
            || token_array[0].val == "<"
            || token_array[0].val == "!=")
    {
        let operator = token_array.pop_front().unwrap().val;
        let right = parse_primary_expr(token_array);
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::BINARY_EXPR,
            left,
            right,
            operator,
        }))
    }
    return left;
}

pub fn parse_ast(token_array: &mut VecDeque<Token>) -> ASTNode {
    let mut left = parse_binary_expr(token_array);
    while token_array.len() > 0 && (token_array[0].val == "&&" || token_array[0].val == "||") {
        let operator = token_array.pop_front().unwrap().val;
        let right = parse_binary_expr(token_array);
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::LOGICAL_EXPR,
            left,
            right,
            operator,
        }))
    }
    return left;
}
