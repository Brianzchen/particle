use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use colored::Colorize;

const CONFIG_FILE_NAME: &str = "particle.config.json";

fn find_parent_file(starting_directory: &Path, file_name: &str) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(file_name);

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

fn get_config() -> Result<(ParticleConfig, String), String> {
    let path = env::current_dir()
        .expect("Cannot read current dir");

    match find_parent_file(&path, CONFIG_FILE_NAME) {
        Some(filepath) => {
            let content = fs::read_to_string(filepath)
                .expect("Should have been able to read the file");
            let config: ParticleConfig = from_str(&content)
                .expect("JSON was not well-formatted");
            Ok((config, "".to_string()))
        },
        None => Err("".to_string()),
    }
}

fn main() {
    let (config, _root_path) = get_config().expect("No config file was found.");
    println!("Workspaces are {:?}", config.workspaces);

    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    if query == "install" {
        println!("install deps I guess");
        // Get a list of workspaces,
        // pull particle.lock.json
        // look at the dependencies/peerDependencies of each
        // create a list of what needs to be installed
        // compare with lock file
        // update lock file
        // and query .npmrc file to figure out where to look for packages
        // then install the remaining uninstall packages
        // cleanup remaining packages
        // only write new lockfile now after successful install
    } else if query == "help" {
        println!("give some helpful hints full of commands")
    } else {
        println!("{}, try `{}` for more information",
            format!("Invalid command given").red().bold(),
            format!("particle help").green()
        )
    }
}
