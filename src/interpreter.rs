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

fn eval_ast_stmt(obj: Value, ast: ASTNode) -> RuntimeType {
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
            // return RuntimeType::Bool(true);
        }
        LiteralType::BINARY_EXPR => {
            return eval_binary_expr(obj, ast);
            // return RuntimeType::Bool(true);
        }
        LiteralType::NUMERIC_LITERAL => {
            return RuntimeType::Element(ast);
        }
        LiteralType::STRING_LITERAL => {
            return RuntimeType::Element(ast);
        }
        _ => RuntimeType::Bool(false),
    }
}

fn _eval_binary_expr(
    obj: Value,
    lhs: RuntimeType,
    rhs: RuntimeType,
    operator: String,
) -> RuntimeType {
    let mut left: String = "".to_string();
    // let mut leftNodeType = LiteralType::NONE_TYPE;
    let mut right: String = "".to_string();
    let mut right_node_type = LiteralType::NONE_TYPE;
    if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = lhs {
        left = val.symbol;
        // leftNodeType = val.kind;
    }
    if let RuntimeType::Element(ASTNode::PrimarySymbol(val)) = rhs {
        right = val.symbol;
        right_node_type = val.kind;
    }

    match operator.as_str() {
        "=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    == right.parse::<f64>().unwrap(),
            ),
            _ => {
                return RuntimeType::Bool(
                    get_value_from_obj(obj, left).to_string() == right.to_string(),
                );
            }
        },
        ">" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    > right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "<" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    < right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        ">=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    >= right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "<=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    <= right.parse::<f64>().unwrap(),
            ),
            _ => return RuntimeType::Bool(false),
        },
        "!=" => match right_node_type {
            LiteralType::NUMERIC_LITERAL => RuntimeType::Bool(
                get_value_from_obj(obj, left)
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
                    != right.parse::<f64>().unwrap(),
            ),
            _ => {
                return RuntimeType::Bool(
                    get_value_from_obj(obj, left).to_string() != right.to_string(),
                )
            }
        },
        _ => RuntimeType::Bool(false),
    }
}

fn _eval_logical_expr(lhs: RuntimeType, rhs: RuntimeType, operator: String) -> RuntimeType {
    let mut left = false;
    let mut right = false;
    if let RuntimeType::Bool(val) = lhs {
        left = val;
    }
    if let RuntimeType::Bool(val) = rhs {
        right = val;
    }
    match operator.as_str() {
        "&&" => RuntimeType::Bool(left && right),
        "||" => RuntimeType::Bool(left || right),
        _ => RuntimeType::Bool(false),
    }
}

fn eval_binary_expr(obj: Value, ast: ASTNode) -> RuntimeType {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let left = ast.left.clone();
        let right = ast.right.clone();
        let lhs = eval_ast_stmt(obj.clone(), left);
        let rhs = eval_ast_stmt(obj.clone(), right);
        return _eval_binary_expr(obj, lhs, rhs, ast.operator.clone());
    }
    return RuntimeType::Bool(false);
}

fn eval_logical_expr(obj: Value, ast: ASTNode) -> RuntimeType {
    if let ASTNode::BinaryExpr(ref ast) = ast {
        let left = ast.left.clone();
        let right = ast.right.clone();
        let lhs = eval_ast_stmt(obj.clone(), left);
        let rhs = eval_ast_stmt(obj.clone(), right);
        return _eval_logical_expr(lhs, rhs, ast.operator.clone());
    }
    return RuntimeType::Bool(false);
}

fn _evaluate_(obj: Value, ast: ASTNode) -> RuntimeType {
    eval_ast_stmt(obj, ast)
}

pub fn eval_query(obj: Value, query_string: String) -> bool {
    let mut tokens = tokenize(query_string);
    let ast = parse_ast(&mut tokens);
    if let RuntimeType::Bool(result) = _evaluate_(obj, ast) {
        return result;
    }
    return false;
}
