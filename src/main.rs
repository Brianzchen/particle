use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONFIG_FILE_NAME: &str = "particle.config.json";

fn find_config_file(starting_directory: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(CONFIG_FILE_NAME);

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break None;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParticleConfig {
    workspaces: Option<Vec<String>>,
    scripts: Option<HashMap<String, String>>,
}

fn get_config_contents() {
    let path = env::current_dir()
        .expect("Cannot read current dir");


    match find_config_file(&path) {
        Some(filepath) => {
            println!("config file was found: {:?}", filepath);
            let content = fs::read_to_string(filepath)
                .expect("Should have been able to read the file");
            let config: ParticleConfig = from_str(&content)
                .expect("JSON was not well-formatted");
            println!("deserialized = {:?}", config);
        },
        None => println!("No config file was found."),
    };
}

fn main() {
    get_config_contents();

    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    println!("you called {query}")
}
