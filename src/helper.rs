use std::collections::VecDeque;

use serde_json::Value;

fn get_param_keys(field_str: &str) -> VecDeque<&str> {
    let mut keys: VecDeque<&str> = VecDeque::new();
    keys.push_back(&field_str);
    if field_str.contains(".") {
        keys = field_str.split(".").map(|x| x).collect::<VecDeque<&str>>();
    }
    return keys;
}

pub fn get_last_key(field_str: &str) -> &str {
    let keys = get_param_keys(field_str);
    return keys[keys.len() - 1];
}
pub fn get_value_from_obj<'a>(obj: &'a Value, field_str: &'a str) -> &'a Value {
    let keys = get_param_keys(field_str);
    let mut value = obj;
    for val in keys.iter() {
        value = &value[val];
    }
    return value;
}
