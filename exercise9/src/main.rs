use std::{io::ErrorKind, collections::HashMap};

use walkdir::WalkDir;

struct EndPoint {
    input: String,
    output: String,
}

fn main() {
    all_files();
}

fn all_files() -> HashMap<String, EndPoint> {
    HashMap::new()
}

// fn all_files() -> Result<Vec<EndPoint>, ErrorKind> {
//     let mut end_points: Vec<EndPoint> = Vec::new();

//     for e in WalkDir::new("./data/")
//         .into_iter()
//         .filter_map(|e| e.ok()) {
//             println!("{}", e.path().display());
//             if e.metadata().unwrap().is_file() {
//                 if e.path().display().to_string().ends_with(".json") {
//                     let path: Vec<&str> = e.path().display().to_string().split('/').collect();
//                     let dir_name: String = path[path.len() - 2].to_string();
//                     let file_name: String = path.last().unwrap().to_string();

//                     let mut input = String::new();
//                     let mut output = String::new();
//                     match file_name.as_str() {
//                         "input.json" => input = file_name,
//                         "output.json" => output = file_name,
//                         _ => return Err(ErrorKind::InvalidInput),
//                     }
//                     if file_name.contains("input") {
//                         input = file_name;
//                     }
//                     let output: String = path.last();
//                     end_points.push(EndPoint { name: dir_name, input, output });
//                 }
//             }
//             println!("{:?}", e.metadata().unwrap());
//         };

//     Ok(vec![])
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_all_files_in_data() {
        let files_result = all_files();
        assert!(files_result.is_ok());

        let files = files_result.unwrap();

        assert_eq!(files.len(), 1);

        assert_eq!(files[0].input, "input.json");
        assert_eq!(files[0].output, "output.json");
    }
}
