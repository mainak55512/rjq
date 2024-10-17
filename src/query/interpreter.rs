use super::parser::{ASTNode, LiteralType};
use crate::utils::get_value_from_obj;
use serde_json::Value;

#[derive(Debug)]
pub enum RuntimeType {
    Element(ASTNode),
    Bool(bool),
}

pub(super) fn eval_ast_stmt(
    obj: &Value,
    ast: &ASTNode,
) -> Result<RuntimeType, Box<dyn std::error::Error>> {
    let kind = match ast {
        ASTNode::PrimarySymbol(ast) => ast.kind,
        ASTNode::BinaryExpr(ast) => ast.kind,
        ASTNode::NoneType => LiteralType::NoneType,
    };
    match kind {
        LiteralType::LogicalExpr => eval_logical_expr(obj, ast),
        LiteralType::BinaryExpr => eval_binary_expr(obj, ast),
        LiteralType::NumericLiteral => Ok(RuntimeType::Element(ast.clone())),
        LiteralType::StringLiteral => Ok(RuntimeType::Element(ast.clone())),
        LiteralType::BinaryOperator => Ok(RuntimeType::Bool(false)),
        LiteralType::NoneType => Ok(RuntimeType::Bool(true)),
    }
}

fn eval_logical_expr(
    obj: &Value,
    ast: &ASTNode,
) -> Result<RuntimeType, Box<dyn std::error::Error>> {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let lhs = eval_ast_stmt(obj, &ast.left)?;
        let rhs = eval_ast_stmt(obj, &ast.right)?;
        return Ok(_eval_logical_expr(lhs, rhs, &ast.operator));
    }
    Ok(RuntimeType::Bool(false))
}

fn eval_binary_expr(obj: &Value, ast: &ASTNode) -> Result<RuntimeType, Box<dyn std::error::Error>> {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let lhs = eval_ast_stmt(obj, &ast.left)?;
        let rhs = eval_ast_stmt(obj, &ast.right)?;
        return _eval_binary_expr(obj, lhs, rhs, &ast.operator);
    }
    Ok(RuntimeType::Bool(false))
}

fn _eval_binary_expr(
    obj: &Value,
    lhs: RuntimeType,
    rhs: RuntimeType,
    operator: &str,
) -> Result<RuntimeType, Box<dyn std::error::Error>> {
    let left = if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = &lhs {
        &val.symbol
    } else {
        ""
    };
    let (right, right_node_type) = if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = &rhs {
        (val.symbol.as_str(), &val.kind)
    } else {
        ("", &LiteralType::NoneType)
    };

    match operator {
        "=" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()?
                    == right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(get_value_from_obj(obj, left) == right)),
        },
        ">" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()? > right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(false)),
        },
        "<" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()? < right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(false)),
        },
        ">=" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()?
                    >= right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(false)),
        },
        "<=" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()?
                    <= right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(false)),
        },
        "!=" => match right_node_type {
            LiteralType::NumericLiteral => Ok(RuntimeType::Bool(
                get_value_from_obj(obj, left).to_string().parse::<f64>()?
                    != right.parse::<f64>()?,
            )),
            _ => Ok(RuntimeType::Bool(get_value_from_obj(obj, left) != right)),
        },
        _ => Ok(RuntimeType::Bool(false)),
    }
}

fn _eval_logical_expr(lhs: RuntimeType, rhs: RuntimeType, operator: &str) -> RuntimeType {
    let left = if let RuntimeType::Bool(val) = lhs {
        val
    } else {
        false
    };
    let right = if let RuntimeType::Bool(val) = rhs {
        val
    } else {
        false
    };
    match operator {
        "&&" => RuntimeType::Bool(left && right),
        "||" => RuntimeType::Bool(left || right),
        _ => RuntimeType::Bool(false),
    }
}
