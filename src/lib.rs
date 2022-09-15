mod commands;
mod constants;
mod utils;

use std::process;
use colored::Colorize;

use utils::highlight;

pub async fn run(
    cwd: &str,
    command: &str,
    arg_2: &Option<String>,
    arg_3: &Option<String>,
    arg_4: &Option<String>,
) {
    let (config, root_path) = utils::get_config(cwd).unwrap_or_else(|_| {
        println!("`particle.config.json` not found. You should add one to the root of your project to get started");
        process::exit(1);
    });

    match command {
        "check" => {
            commands::check(&config, &root_path).await;
        },
        "run" => {
            if let Some(script) = arg_2 {
                commands::run(&config, &root_path, script);
            } else {
                println!("To use {} you must also pass a script", highlight(&String::from("run")));
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
                highlight(&String::from("particle help"))
            );
        }
    }
}
