use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::from_str;

use crate::constants;

pub fn find_parent_file(starting_directory: &Path, file_name: &str) -> Option<PathBuf> {
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

pub fn get_config() -> Result<(constants::ParticleConfig, String), ()> {
  let path = env::current_dir()
      .expect("Cannot read current dir");

  match find_parent_file(&path, constants::CONFIG_FILE_NAME) {
      Some(filepath) => {
          let root_path = filepath.display().to_string();
          let config_file_index = root_path.find(constants::CONFIG_FILE_NAME).unwrap();
          let root_path = &root_path[..config_file_index];

          let content = fs::read_to_string(filepath)
              .expect("Should have been able to read the file");
          let config: constants::ParticleConfig = from_str(&content)
              .expect("JSON was not well-formatted");

          Ok((config, root_path.to_string()))
      },
      None => Err(()),
  }
}

pub fn get_workspaces_data(_config: &constants::ParticleConfig) {
  // TODO
}
