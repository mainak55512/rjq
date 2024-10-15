mod query;
mod utils;

use clap::Parser;
use query::Query;
use serde_json::Value;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use utils::get_last_key;
use utils::get_value_from_obj;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    load: Option<PathBuf>,

    #[arg(short, long)]
    query: Option<String>,

    #[arg(short, long)]
    params: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let content = if let Some(load) = cli.load.as_deref() {
        match fs::read_to_string(load) {
            Ok(val) => val,
            Err(_e) => {
                println!("File not found or couldn't read content from file");
                std::process::exit(1);
            }
        }
    } else {
        io::stdin()
            .lock()
            .lines()
            .fold("".to_string(), |acc, line| {
                acc + &line.expect("Couldn't read from stdin") + "\n"
            })
    };
    let content = serde_json::from_str::<Vec<Value>>(&content)?;

    let query_string = cli.query.as_deref().unwrap_or_default();
    let query = Query::new(query_string);

    let params = cli.params.unwrap_or_default();
    let params = params
        .split(",")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    if query_string.is_empty() && params.is_empty() {
        println!(
            "{}",
            serde_json::to_string_pretty(&content).expect("Can't convert JSON to string")
        );
    } else if params.is_empty() {
        let mut result_arr = VecDeque::new();
        for obj in &content {
            if query.eval(obj) {
                result_arr.push_back(obj.clone());
            }
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&result_arr).expect("Can't convert JSON to string")
        );
    } else if query_string.is_empty() {
        let mut result_arr = VecDeque::new();
        for obj in &content {
            let mut entry = serde_json::Map::new();
            for item in &params {
                entry.insert(
                    get_last_key(item).to_string(),
                    get_value_from_obj(obj, item).clone(),
                );
            }
            result_arr.push_back(Value::Object(entry));
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&result_arr).expect("Can't convert JSON to string")
        );
    } else {
        let mut result_arr = VecDeque::new();
        for obj in &content {
            if query.eval(obj) {
                let mut entry = serde_json::Map::new();
                for item in &params {
                    entry.insert(
                        get_last_key(item).to_string(),
                        get_value_from_obj(obj, item).clone(),
                    );
                }
                result_arr.push_back(Value::Object(entry));
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&result_arr).expect("Can't convert JSON to string")
        );
    }

    Ok(())
}
