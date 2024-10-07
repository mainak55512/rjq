use serde_json::{Map, Result, Value};

fn _main() {
    let _data = set_value();
    let json_string = serde_json::to_string(&_data).unwrap();

    get_value(&json_string).unwrap();
}

fn get_value(data: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    println!(
        "Name: {}, Address: {}, First Key: {}",
        v["name"], v["address"], v["keys"][0]
    );

    Ok(())
}

fn set_value() -> Map<std::string::String, Value> {
    let mut json_obj = Map::new();
    json_obj.insert("name".to_string(), Value::String("Mainak".to_string()));
    json_obj.insert(
        "address".to_string(),
        Value::String("Kolkata, West Bengal".to_string()),
    );
    json_obj.insert(
        "keys".to_string(),
        Value::Array(vec![
            Value::String("kdjffh23bfh".to_string()),
            Value::String("uierb344bhd".to_string()),
            Value::String("etrgh4637hg".to_string()),
            Value::String("poy436bfj34".to_string()),
            Value::String("bvhfjk453vh".to_string()),
        ]),
    );

    return json_obj;
}
