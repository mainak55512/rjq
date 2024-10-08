#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod helper;
mod interpreter;
mod lexer;
mod parser;

use clap::Parser;
use interpreter::eval_query;
use serde_json::Value;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    load: Option<PathBuf>,

    #[arg(short, long)]
    query: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut result_arr: VecDeque<Value> = VecDeque::new();

    let mut content = String::new();
    let mut query_string = String::new();
    if let Some(load) = cli.load.as_deref() {
        content = fs::read_to_string(load).unwrap();
    }
    if let Some(query) = cli.query.as_deref() {
        query_string = String::from(query);
    }

    let v: VecDeque<Value> = serde_json::from_str(&content).unwrap();

    for obj in v.iter() {
        if eval_query(obj, &query_string) {
            result_arr.push_back(obj.clone());
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&result_arr)
            .unwrap()
            .to_string()
    );
}
