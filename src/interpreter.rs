use crate::{
    helper::get_value_from_obj,
    lexer::tokenize,
    parser::{parse_ast, ASTNode, LiteralType},
};
use serde_json::Value;

pub enum RuntimeType {
    Element(ASTNode),
    Bool(bool),
}

fn eval_ast_stmt(obj: &Value, ast: &ASTNode) -> RuntimeType {
    let mut kind = LiteralType::NONE_TYPE;
    match ast {
        ASTNode::PrimarySymbol(ref ast) => {
            kind = ast.kind.clone();
        }
        ASTNode::BinaryExpr(ref ast) => {
            kind = ast.kind.clone();
        }
        ASTNode::NONE_TYPE => {}
    }
    match kind {
        LiteralType::LOGICAL_EXPR => {
            return eval_logical_expr(obj, ast);
        }
        LiteralType::BINARY_EXPR => {
            return eval_binary_expr(obj, ast);
        }
        LiteralType::NUMERIC_LITERAL => {
            return RuntimeType::Element(ast.clone());
        }
        LiteralType::STRING_LITERAL => {
            return RuntimeType::Element(ast.clone());
        }
        _ => RuntimeType::Bool(false),
    }
}

fn _eval_binary_expr(
    obj: &Value,
    lhs: RuntimeType,
    rhs: RuntimeType,
    operator: &str,
) -> RuntimeType {
    let left = if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = &lhs {
        &val.symbol
    } else {
        ""
    };
    let (right, right_node_type) = if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = &rhs {
        (val.symbol.as_str(), &val.kind)
    } else {
        ("", &LiteralType::NONE_TYPE)
    };

    match operator {
        "=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    == right.parse::<f64>().unwrap(),
            ),
            _ => {
                return RuntimeType::Bool(
                    get_value_from_obj(obj, &left).to_string() == right.to_string(),
                );
            }
        },
        ">" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    > right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "<" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    < right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        ">=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    >= right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "<=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    <= right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "!=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, &left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    != right.parse::<f64>().unwrap(),
            ),
            _ => {
                return RuntimeType::Bool(
                    get_value_from_obj(obj, &left).to_string() != right.to_string(),
                )
            }
        },
        _ => RuntimeType::Bool(false),
    }
}

fn _eval_logical_expr(lhs: RuntimeType, rhs: RuntimeType, operator: &str) -> RuntimeType {
    let mut left = false;
    let mut right = false;
    if let RuntimeType::Bool(val) = lhs {
        left = val;
    }
    if let RuntimeType::Bool(val) = rhs {
        right = val;
    }
    match operator {
        "&&" => RuntimeType::Bool(left && right),
        "||" => RuntimeType::Bool(left || right),
        _ => RuntimeType::Bool(false),
    }
}

fn eval_binary_expr(obj: &Value, ast: &ASTNode) -> RuntimeType {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let left = &ast.left;
        let right = &ast.right;
        let lhs = eval_ast_stmt(obj, left);
        let rhs = eval_ast_stmt(obj, right);
        return _eval_binary_expr(obj, lhs, rhs, &ast.operator);
    }
    return RuntimeType::Bool(false);
}

fn eval_logical_expr(obj: &Value, ast: &ASTNode) -> RuntimeType {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let left = &ast.left;
        let right = &ast.right;
        let lhs = eval_ast_stmt(obj, left);
        let rhs = eval_ast_stmt(obj, right);
        return _eval_logical_expr(lhs, rhs, &ast.operator);
    }
    return RuntimeType::Bool(false);
}

fn _evaluate_(obj: &Value, ast: &ASTNode) -> RuntimeType {
    eval_ast_stmt(obj, ast)
}

pub fn eval_query(obj: &Value, query_string: &str) -> bool {
    let mut tokens = tokenize(query_string);
    let ast = parse_ast(&mut tokens);
    if let RuntimeType::Bool(result) = _evaluate_(obj, &ast) {
        return result;
    }
    return false;
}
