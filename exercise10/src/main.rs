use std::fs;

use json_typegen::json_typegen;

json_typegen!("Ticker", "./output.json");

fn main() {
    let output = fs::read_to_string("./output.json").unwrap();
    println!("{}", output);

    let t: Ticker = serde_json::from_str(&output).unwrap();
    println!("{:?}", t);
}
