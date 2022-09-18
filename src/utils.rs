pub mod printer;

use glob::glob;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::from_str;
use run_script::{run_script};

use crate::constants;

fn find_parent_folder(starting_directory: &str, file_name: &str) -> Option<PathBuf> {
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

pub fn get_config(cwd: &str) -> Result<(constants::ParticleConfig, String), Box<dyn Error>> {
    match find_parent_folder(cwd, constants::CONFIG_FILE_NAME) {
        Some(filepath) => {
            let root_path = filepath.display().to_string();
            let config_file_index = root_path.find(constants::CONFIG_FILE_NAME).unwrap();
            let root_path = &root_path[..config_file_index];

            let content = fs::read_to_string(filepath)?;
            let config: constants::ParticleConfig = from_str(&content)?;

            Ok((config, root_path.to_string()))
        },
        None => Err("Directory wasn't found".into()),
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

        let pkg_json_length = path.split("/package.json")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .to_owned();

        constants::Workspace {
            workspace_path: String::from(pkg_json_length),
            path: String::new() + path,
            package: pkg_json,
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
    } else {
        println!("{}", error);
    }
}

pub fn run_script_in_optional_scripts(
    root_path: &String,
    path: &String,
    scripts: &Option<constants::Scripts>,
    script: &String,
) {
    if let Some(s) = scripts {
        let script_value = s.get(script);
        if let Some(run) = script_value {
            let script = String::from(format!(
                "(cd {} && {})",
                path,
                run.replace("<particle-root>", root_path),
            ));
            execute_string(&script);
        } else {
            println!("Script {} does not exist!", printer::highlight(script));
        }
    }
}
