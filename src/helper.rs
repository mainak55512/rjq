use std::collections::VecDeque;

use serde_json::Value;

fn get_param_keys(field_str: String) -> VecDeque<String> {
    let mut keys = VecDeque::new();
    keys.push_back(field_str.clone());
    if field_str.contains(".") {
        keys = field_str
            .split(".")
            .map(|x| x.to_string())
            .collect::<VecDeque<String>>();
    }
    return keys;
}

pub fn get_last_key(field_str: String) -> String {
    let keys = get_param_keys(field_str);
    return keys[keys.len() - 1].clone();
}
pub fn get_value_from_obj(obj: Value, field_str: String) -> Value {
    let keys = get_param_keys(field_str);
    let mut value = obj;
    for val in keys.iter() {
        value = value[val].clone();
    }
    return value;
}
