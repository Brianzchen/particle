use std::env;
use std::path::{Path, PathBuf};

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

fn get_config_contents() -> std::io::Result<()> {
    let path = env::current_dir()?;

    match find_config_file(&path) {
        Some(filepath) => println!("config file was found: {:?}", filepath),
        None => println!("No config file was found."),
    };

    Ok(())
}

fn main() {
    get_config_contents().unwrap();
}
