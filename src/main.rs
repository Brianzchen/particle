use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use colored::Colorize;
use clap::Parser;

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

#[derive(Serialize, Deserialize, Debug)]
struct ParticleDependencyLock {

}

fn get_config() -> Result<(ParticleConfig, String), String> {
    let path = env::current_dir()
        .expect("Cannot read current dir");

    match find_parent_file(&path, CONFIG_FILE_NAME) {
        Some(filepath) => {
            let root_path = filepath.display().to_string();
            let config_file_index = root_path.find(CONFIG_FILE_NAME).unwrap();
            let root_path = &root_path[..config_file_index];

            let content = fs::read_to_string(filepath)
                .expect("Should have been able to read the file");
            let config: ParticleConfig = from_str(&content)
                .expect("JSON was not well-formatted");

            Ok((config, root_path.to_string()))
        },
        None => Err("".to_string()),
    }
}

fn check(config: &ParticleConfig, root_path: &String) {
    println!("install deps I guess");
    // Pull workspace data
    println!("Workspaces are {:?}", config.workspaces);

    // Pull lock file data
    let mut lock_file = root_path.clone();
    lock_file.push_str("/particle.lock.json");
    let lock_file = fs::read_to_string(lock_file);
    let lock_file = match lock_file {
        Ok(content) => {
            let config: ParticleDependencyLock = from_str(&content)
                .expect("lock file is malformed");
            config
        },
        Err(_) => {
            println!("No lock file found");
            ParticleDependencyLock {}
        }
    };
    println!("the lock contents are {:?}", lock_file);
    // look at the dependencies/peerDependencies of each
    // create a list of what needs to be installed
    // compare with lock file
    // update lock file
    // and query .npmrc file to figure out where to look for packages
    // then install the remaining uninstall packages
    // what if we lazily installed dependencies
    // Then particle would also need to be a runtime but we want to be un opinionated
    // cleanup remaining packages
    // only write new lockfile now after successful install
}

/// An unopinionated monorepo package manager for JS based applications.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Check https://github.com/Brianzchen/particle#usage to learn more about available commands
   command: String,

   /// When running the `workspace` command, the package you'd like to target
   arg_2: Option<String>,
}

fn main() {
    let (config, root_path) = get_config()
        .expect("`particle.config.json` not found. You should add one to the root of your project to get started");

    let args = Args::parse();
    let command = &args.command[..];

    match command {
        "check" => {
            check(&config, &root_path);
        },
        "workspace" => {
            println!("pull the package name");
            if args.arg_2 == None {
                println!("You've called `workspace` without the --package option");
            }
            println!("{:?}", args.arg_2);
        }
        _ => {
            println!("{}, try `{}` for more information",
                format!("Invalid command given").red().bold(),
                format!("particle help").green()
            );
        }
    }
}
