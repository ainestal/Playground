// use std::fs;
// use json_typegen_shared::codegen_from_macro_input;

// use json_typegen::json_typegen;
// json_typegen!("Ticker", "./output.json");
use wraph::wraph;
wraph!("Ticker", "./output.json");

fn main() {
    let _input = "\"Ticker\", \"./output.json\"";
    // let result = codegen_from_macro_input(input).unwrap();

    // println!("{:#?}", result);

    // let output = fs::read_to_string("./output.json").unwrap();
    // println!("{}", output);

    // let t: Ticker = serde_json::from_str(&output).unwrap();
    // println!("{:?}", t);
}

