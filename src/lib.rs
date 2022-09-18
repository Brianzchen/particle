mod commands;
mod constants;
pub mod utils;

use std::{process, error::Error};
use colored::Colorize;

use utils::{printer};

pub async fn run(
    cwd: &str,
    command: &str,
    arg_2: &Option<String>,
    arg_3: &Option<String>,
    arg_4: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    if let Ok(config) = utils::get_config(cwd) {
        let (config, root_path) = config;

        match command {
            "check" => {
                commands::check(&config, &root_path).await;
            },
            "run" => {
                if let Some(script) = arg_2 {
                    commands::run(&config, &root_path, script);
                } else {
                    println!("To use {} you must also pass a script", printer::highlight(&String::from("run")));
                    process::exit(1);
                }
            },
            "workspace" => {
                commands::workspace(
                    &config,
                    &root_path,
                    arg_2,
                    arg_3,
                    arg_4,
                );
            },
            "uncache" => {
                commands::uncache();
            },
            _ => {
                println!("{}, try {} for more information",
                    format!("Invalid command given").red().bold(),
                    printer::highlight(&String::from("particle help"))
                );
            }
        }
    } else {
        return Err("`particle.config.json` not found. You should add one to the root of your project to get started".into());
    }

    Ok(())
}
