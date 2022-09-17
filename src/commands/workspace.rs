use crate::constants::{ParticleConfig, Workspace};
use crate::utils::{get_workspaces_data, run_script_in_optional_scripts, highlight};

fn find_workspace(workspaces: Vec<Workspace>, lookup: &String) -> Result<Workspace, String> {
    let found_workspace = workspaces.into_iter().find(|w| {
        if let Some(name) = &w.package.name {
            return name == lookup;
        }
        false
    });

    match found_workspace {
        Some(found_workspace) => {
            Ok(found_workspace)
            },
        None => {
            Err(lookup.to_owned())
        },
    }
}

pub fn main(
    config: &ParticleConfig,
    root_path: &String,
    arg_2: &Option<String>,
    arg_3: &Option<String>,
    arg_4: &Option<String>,
) {
  let workspaces = get_workspaces_data(&config, &root_path);

  match arg_2 {
    Some(user_workspace) => {
      let found_workspace = find_workspace(workspaces, user_workspace);
      match found_workspace {
        Ok(workspace) => {
          match arg_3 {
            Some(cmd) => {
              match cmd.as_str() {
                "install" => {
                  println!("You ran install");
                },
                "run" => {
                  match arg_4 {
                    Some(script) => {
                      run_script_in_optional_scripts(
                        &workspace.workspace_path,
                        &workspace.package.scripts,
                        script,
                      );
                    },
                    None => {
                      panic!("You cannot call workspace run without a script");
                    },
                  }
                },
                _ => {
                  // TODO try to run relative path command
                  println!("Command {} not possible on workspace", highlight(cmd));
                  println!("If you're trying to run a script try prefix the script with `{}`", highlight(&String::from("run ")));
                },
              }
            },
            None => {

            },
          }
        },
        Err(workspace) => {
          println!("Could not find workspace {}, are you sure it exists?", highlight(&workspace));
        },
      }
    },
    None => {
        println!("You've called {} without the --package option", highlight(&String::from("workspace")));
        println!("Try again");
    },
  }
}
