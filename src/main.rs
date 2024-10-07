#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod helper;
mod interpreter;
mod lexer;
mod parser;

use std::collections::VecDeque;

use interpreter::eval_query;
use serde_json::Value;

fn main() {
    let mut result_arr: VecDeque<Value> = VecDeque::new();
    let query_string = String::from(r#"(id.id1 = 'Mainak123' && name = 'Test') || age > 18"#);

    let data = r#"
        [{
            "name": "est",
            "age": 25,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "id": {
                "id1":"Mainak123",
                "id2": "false"
            }
        },{
            "name": "Test",
            "age": 15,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "id": {
                "id1":"Mainak123",
                "id2": "false"
            }
        }]"#;
    let v: VecDeque<Value> = serde_json::from_str(data).unwrap();

    for obj in v.iter() {
        if eval_query(obj.clone(), query_string.clone()) {
            result_arr.push_back(obj.clone());
        }
    }

    println!(
        "{}",
        serde_json::to_string(&result_arr).unwrap().to_string()
    );

    /*
    * Output:
    *
    [{"age":25,"id":{"id1":"Mainak123","id2":"false"},"name":"est","phones":["+44 1234567","+44 2345678"]},{"age":15,"id":{"id1":"Mainak123","id2":"false"},"name":"Test","phones":["+44 1234567","+44 2345678"]}]
    *
    */
}
