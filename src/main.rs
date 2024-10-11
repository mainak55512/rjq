#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod helper;
mod interpreter;
mod lexer;
mod parser;

use clap::Parser;
use helper::get_last_key;
use helper::get_value_from_obj;
use interpreter::eval_query;
use serde_json::Value;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    load: Option<PathBuf>,

    #[arg(short, long)]
    query: Option<String>,

    #[arg(short, long)]
    params: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let content = if let Some(load) = cli.load.as_deref() {
        fs::read_to_string(load).expect("Could't read file content")
    } else {
        io::stdin()
            .lock()
            .lines()
            .fold("".to_string(), |acc, line| {
                acc + &line.expect("Couldn't read from stdin") + "\n"
            })
    };
    let query_string = if let Some(query) = cli.query.as_deref() {
        query
    } else {
        ""
    };

    let params: Vec<&str> = if let Some(params_list) = cli.params.as_deref() {
        params_list.split(",").map(|x| x.trim()).collect()
    } else {
        Vec::<&str>::new()
    };

    let v: VecDeque<Value> = serde_json::from_str(&content).expect("Couldn't parse to JSON");

    if query_string.is_empty() && params.is_empty() {
        println!(
            "{}",
            serde_json::to_string_pretty(&v)
                .expect("Can't convert JSON to string")
                .to_string()
        );
    } else if params.is_empty() {
        let mut result_arr: VecDeque<Value> = VecDeque::new();
        for obj in v.iter() {
            if eval_query(obj, query_string) {
                result_arr.push_back(obj.clone());
            }
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&result_arr)
                .expect("Can't convert JSON to string")
                .to_string()
        );
    } else if query_string.is_empty() {
        let mut result_arr: VecDeque<Value> = VecDeque::new();
        for obj in v.iter() {
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
            serde_json::to_string_pretty(&result_arr)
                .expect("Can't convert JSON to string")
                .to_string()
        );
    } else {
        let mut result_arr: VecDeque<Value> = VecDeque::new();
        for obj in v.iter() {
            if eval_query(obj, query_string) {
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
            serde_json::to_string_pretty(&result_arr)
                .expect("Can't convert JSON to string")
                .to_string()
        );
    }
}
