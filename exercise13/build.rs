// use std::{fs, io::Error, process::Command};

// fn main() -> Result<(), Error> {
//     let ticker_input = "ticker.json";
//     let types_path = "src/types.rs";
//     let wraph_types_path = "src/wtypes.rs";

//     let mut list_dir = Command::new("json_typegen");

//     list_dir
//         .arg(ticker_input)
//         .args(["-o", types_path])
//         .args(["-n", "Ticker"]);

//     list_dir.status().expect("process failed to execute");

//     let types = fs::read_to_string(types_path)?;
//     let from = "#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]";
//     let to = "#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Wraph)]";
//     let contents = types.replace(from, to);

//     fs::write(wraph_types_path, contents)?;

//     fs::write(wraph_types_path, "blah")?;

//     Ok(())
// }
fn main() {
    ()
}