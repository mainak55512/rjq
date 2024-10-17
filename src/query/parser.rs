use std::collections::VecDeque;

use super::lexer::{Token, TokenType};

#[derive(Debug, Clone, Copy)]
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

pub(super) fn parse_ast(
    tokens: &mut VecDeque<Token>,
) -> Result<ASTNode, Box<dyn std::error::Error>> {
    if tokens.is_empty() {
        return Ok(ASTNode::NoneType);
    }
    let mut left = parse_binary_expr(tokens)?;
    if !tokens.is_empty() && tokens[0].val != "&&" && tokens[0].val != "||" {
        return Err("Query is invalid".into());
    }
    while !tokens.is_empty() && (tokens[0].val == "&&" || tokens[0].val == "||") {
        let operator = tokens.pop_front().ok_or("Empty operator")?.val;
        if tokens.is_empty() {
            return Err("Query is invalid".into());
        }
        let right = parse_binary_expr(tokens)?;
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::LogicalExpr,
            left,
            right,
            operator,
        }))
    }
    Ok(left)
}

fn parse_primary_expr(
    token_array: &mut VecDeque<Token>,
) -> Result<ASTNode, Box<dyn std::error::Error>> {
    let tk = &token_array[0].token_type;
    match tk {
        TokenType::Number => Ok(ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::NumericLiteral,
            symbol: token_array.pop_front().ok_or("Not a Number")?.val,
        })),
        TokenType::String => Ok(ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::StringLiteral,
            symbol: token_array.pop_front().ok_or("Invalid String")?.val,
        })),
        TokenType::Binary => Ok(ASTNode::PrimarySymbol(PrimarySymbol {
            kind: LiteralType::BinaryOperator,
            symbol: token_array.pop_front().ok_or("Invalid Operator")?.val,
        })),
        TokenType::Paren => {
            token_array.pop_front();
            let value = parse_ast(token_array)?;
            token_array.pop_front();
            Ok(value)
        }
    }
}

fn parse_binary_expr(
    token_array: &mut VecDeque<Token>,
) -> Result<ASTNode, Box<dyn std::error::Error>> {
    let mut left = parse_primary_expr(token_array)?;
    while !token_array.is_empty()
        && (token_array[0].val == "="
            || token_array[0].val == ">="
            || token_array[0].val == "<="
            || token_array[0].val == ">"
            || token_array[0].val == "<"
            || token_array[0].val == "!=")
    {
        let operator = token_array.pop_front().ok_or("Invalid Operator")?.val;
        let right = parse_primary_expr(token_array)?;
        left = ASTNode::BinaryExpr(Box::new(BinaryExpr {
            kind: LiteralType::BinaryExpr,
            left,
            right,
            operator,
        }))
    }
    Ok(left)
}
