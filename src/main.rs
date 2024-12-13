use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
mod errors;

// returns an iterator to the reader of the lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// read and parse config file with the following syntax:
// - each line must '='
// - each line must contain only one '='
// - key is case insensitive
// - value is case sensitive
// - config must contain 'required_keys'
// - config can only contain 'available_keys' or 'required_keys'
fn read_parse_config_file(
    config_file_path: &str,
    required_keys: HashSet<&str>,
    available_keys: HashSet<&str>,
) -> Result<HashMap<String, String>, errors::FileParseError> {
    let mut config = HashMap::new();
    // read, parse and sanitize the lines in file
    if let Ok(lines) = read_lines(config_file_path) {
        for (line_num, line) in lines.flatten().enumerate().map(|(i, v)| (i + 1, v)) {
            let words: Vec<&str> = line.split("=").collect();
            if words.len() != 2 {
                return Err(errors::FileParseError::new(
                    format!("Check config file syntax on line {}: '{}'", line_num, line)
                        .to_string(),
                ));
            }
            config.insert(words[0].trim().to_lowercase(), words[1].trim().to_string());
        }
    };
    // check for required fields
    for key in required_keys.iter() {
        if !config.contains_key(*key) {
            return Err(errors::FileParseError::new(
                format!("Config file must contain the key '{}'", key).to_string(),
            ));
        }
    }
    // remove unnecessary fields
    config.retain(|key, _| available_keys.contains(key.as_str()) || required_keys.contains(key.as_str()));
    Ok(config)
}

fn read_todo_file() {}

fn main() {
    let config = match read_parse_config_file(
        ".destined",
        HashSet::from(["todo_file"]),
        HashSet::from([]),
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    for (k, v) in config {
        println!("{} => {}", k, v);
    }
}
