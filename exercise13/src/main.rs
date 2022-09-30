use glob::glob;
use log::error;
use std::{env, fs, io::Error, process::Command};

// Common files with all the types and all the types with the Wraph derive macro added
const INPUTS: &str = "./inputs/**/*.json";
const WRAPH_TYPES_PATH: &str = "src/types.rs";

fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "warn");
    env_logger::init();

    let mut contents: Vec<String> = vec![];

    for entry in glob(INPUTS).expect("Failed to read from INPUTS") {
        // MUST be a file with a .json extension
        match entry {
            Ok(path) => {
                let file_path = path
                    .to_str()
                    .expect("Unable to retrieve the file name of the input");
                let file_name = file_path
                    .split("/")
                    .into_iter()
                    .last()
                    .expect("Unable to split the filename correctly");
                let type_name = type_name(file_name);

                // Use the Command tool to generate the types
                match generate_types(file_path, type_name) {
                    Some(p) => {
                        // Add Wraph to each of the types, replacing in the file
                        let mut file = fs::OpenOptions::new().append(true).open(WRAPH_TYPES_PATH)?;
                        let content = add_wraph(p.clone())?;
                        write!(file, "{}", content)?;
                        contents.push(p);
                    }
                    None => (),
                }
            }
            Err(_) => {
                error!("Unable to retrieve the entry");
                continue;
            }
        }
    }

    fs::OpenOptions::new().append(true).open(WRAPH_TYPES_PATH)?;
    // fs::write(WRAPH_TYPES_PATH, contents)?;

    Ok(())
}

fn add_wraph(path: String) -> Result<String, Error> {
    let types = fs::read_to_string(path)?;
    let from = "#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]";
    let to = "#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Wraph)]";
    let contents = types.replace(from, to);
    Ok(contents)
}

fn generate_types(input: &str, type_name: String) -> Option<String> {
    let output = format!("tmp/{}.out", type_name);

    let mut cmd = Command::new("json_typegen");
    cmd.arg(input)
        .args(["-o", &output])
        .args(["-n", &type_name]);

    if let Err(_) = cmd.status() {
        error!("json_typegen failed to execute");
        return None;
    } else {
        Some(output)
    }
}

fn type_name(input: &str) -> String {
    // Name of the type will be the name of the file
    let s: Vec<&str> = input.split(".").collect();
    let chunk = s[0];
    let tname = chunk.to_string()[0..1].to_uppercase() + &chunk[1..];
    tname
}
