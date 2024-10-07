// use std::collections::{vec_deque, VecDeque};

use helper::get_value_from_obj;
use interpreter::eval_query;
// use interpreter::_eval_binary_expr;
use serde_json::{Number, Value};
mod helper;
mod interpreter;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
// mod data_load;
mod lexer;
mod parser;

fn main() {
    let query_string = String::from(r#"(name = "Mainak" || name = "Test") && age > 18"#);

    let data = r#"
        {
            "name": "Mainak",
            "age": 19,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "id": {
                "id1":"Mainak123",
                "id2": false
            }
        }"#;
    let v: Value = serde_json::from_str(data).unwrap();

    println!("{}", eval_query(v, query_string));
}
