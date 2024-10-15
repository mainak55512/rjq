use std::collections::VecDeque;

use serde_json::Value;

fn param_keys(field_str: &str) -> VecDeque<&str> {
    field_str.split(".").collect()
}

pub fn get_last_key(field_str: &str) -> &str {
    let keys = param_keys(field_str);
    keys[keys.len() - 1]
}

pub fn get_value_from_obj<'a>(obj: &'a Value, field_str: &str) -> &'a Value {
    let keys = param_keys(field_str);
    let mut value = obj;
    for val in keys.iter() {
        value = &value[val];
    }
    value
}
