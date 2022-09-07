use glob::glob;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::from_str;
use run_script::{run_script};
use colored::{Colorize};

use crate::constants;

pub fn highlight(value: &String) -> String {
  format!("`{}`", format!("{}", value).bold().green())
}

fn find_parent_folder(starting_directory: &Path, file_name: &str) -> Option<PathBuf> {
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

  match find_parent_folder(&path, constants::CONFIG_FILE_NAME) {
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

pub fn get_workspaces_data(config: &constants::ParticleConfig, root_path: &String) -> Vec<constants::Workspace> {
  let workspaces = &config.workspaces;

  let mut workspace_paths: Vec<String> = vec![];

  for pattern in workspaces {
    let workspaces = glob(format!("{}{}/package.json", root_path, pattern).as_str())
      .expect(format!("Failed to read glob pattern: {}", pattern).as_str());


    for workspace in workspaces.into_iter() {
      match workspace {
        Ok(path) => {
          workspace_paths.push(path.into_os_string().into_string()
            .expect("Could not convert workspace path into string"));
        },
        Err(err) => {
          panic!("{}", err);
        },
      }
    }
  }

  // We don't want path duplications of the same workspace twice if
  // multiple globs catch the same workspace
  workspace_paths.sort_unstable();
  workspace_paths.dedup();

  let workspaces = workspace_paths.iter().map(|path| {
    let pkg_json = fs::read_to_string(path)
      .expect(format!("workspace {} package.json cannot be read", path).as_str());
    let pkg_json: constants::PkgJson = from_str(&pkg_json)
      .expect(format!("JSON not well formed when parsing {} package.json", path).as_str());

    constants::Workspace {
      name: pkg_json.name,
      path: String::new() + path,
      scripts: pkg_json.scripts,
    }
  }).collect();

  workspaces
}

/// Execute a script in a string format
fn execute_string(script: &String) {
  let (_, output, error) = run_script!(script).unwrap();
  if error.len() == 0 {
    if output.len() > 0 {
      print!("{}", output);
    }
    println!("Done ✨");
  } else {
    println!("{}", error);
  }
}

pub fn run_script_in_optional_scripts(scripts: &constants::Scripts, script: &String) {
  if let Some(s) = scripts {
    let script_value = s.get(script);
    if let Some(run) = script_value {
      execute_string(run);
    } else {
      // says none here, fix it!
      println!("Script {} does not exist!", highlight(script));
    }
  }
}
