use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

mod errors;
mod task;

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
    let lines = read_lines(config_file_path)
        .map_err(|_| errors::FileParseError::new("Could not read config file".to_string()))?;
    // fill config map
    for (line_num, line) in lines.flatten().enumerate().map(|(i, v)| (i + 1, v)) {
        let words: Vec<&str> = line.split("=").collect();
        if words.len() != 2 {
            return Err(errors::FileParseError::new(
                format!("Check config file syntax on line {}: '{}'", line_num, line).to_string(),
            ));
        }
        config.insert(words[0].trim().to_lowercase(), words[1].trim().to_string());
    }
    // check for required fields
    for key in required_keys.iter() {
        if !config.contains_key(*key) {
            return Err(errors::FileParseError::new(
                format!("Config file must contain the key '{}'", key).to_string(),
            ));
        }
    }
    // remove unnecessary fields
    config.retain(|key, _| {
        available_keys.contains(key.as_str()) || required_keys.contains(key.as_str())
    });
    Ok(config)
}

// read and parse todo file with following syntax
// - white spaces are ignored
// - # tells that the following tasks are associated with the goal
// - - tells that the line is a task
fn read_parse_todo_file(todo_file_path: &str) -> Result<Vec<task::Task>, errors::FileParseError> {
    let mut tasks: Vec<task::Task> = vec![];
    let lines = read_lines(todo_file_path)
        .map_err(|_| errors::FileParseError::new("Could not read todo file".to_string()))?;
    let mut current_goal = String::from("");
    for (line_num, line) in lines.flatten().enumerate().map(|(i, v)| (i + 1, v)) {
        let line = line.trim();
        if line.starts_with("-") {
            let mut task = task::Task::from_string(line).map_err(|e| {
                errors::FileParseError::new(format!(
                    "Cannot parse todo on line {}: {}",
                    line_num + 1,
                    e.message
                ))
            })?;
            if !current_goal.is_empty() {
                task.set_goal(current_goal.clone());
            }
            tasks.push(task);
        } else if line.starts_with("#") {
            current_goal = line.trim_start_matches(['#', ' ']).to_string();
        }
    }
    Ok(tasks)
}

fn main() {
    // parse config
    let config = match read_parse_config_file(
        ".destined",
        HashSet::from(["todo_file", "history_file", "editor"]),
        HashSet::from([]),
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    println!("==========");
    println!("found config keys:");
    for (k, v) in &config {
        println!("\t{}    =>    {}", k, v);
    }
    println!("==========");

    // parse todo
    let todos = match read_parse_todo_file(&config["todo_file"]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
}
