mod query;
mod utils;

use clap::Parser;
use query::Query;
use serde_json::Value;
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
        fs::read_to_string(load).map_err(|err| err.to_string())?
    } else {
        io::stdin()
            .lock()
            .lines()
            .map(|x| x.expect("Couldn't read from stdin"))
            .collect()
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

    // create a filter closure that applies the params.
    let filter: &dyn Fn(&Value) -> Value = if params.is_empty() {
        &|obj| obj.clone()
    } else {
        &|obj| {
            let mut map = serde_json::Map::new();
            for item in &params {
                map.insert(
                    get_last_key(item).to_string(),
                    get_value_from_obj(obj, item).clone(),
                );
            }
            Value::Object(map)
        }
    };

    // apply the query and params to content.
    let mut output = Vec::new();
    for obj in &content {
        if query.eval(obj) {
            output.push(filter(obj));
        }
    }
    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
