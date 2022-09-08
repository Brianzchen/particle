mod commands;
mod constants;
mod utils;
use colored::Colorize;
use clap::Parser;

use crate::utils::{get_config, highlight};

/// An unopinionated monorepo package manager for JS based applications.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Check https://github.com/Brianzchen/particle#usage to learn more about available commands
   command: String,

   /// When running the `workspace` command, the package you'd like to target
   arg_2: Option<String>,

   /// If
   /// workspace: the workspace you'd like to run commands on
   arg_3: Option<String>,

   /// If
   /// workspace: the command against the workspace
   arg_4: Option<String>,
}

fn main() {
    let (config, root_path) = get_config()
        .expect("`particle.config.json` not found. You should add one to the root of your project to get started");

    let args = Args::parse();
    let command = &args.command[..];
    let arg_2 = &args.arg_2;
    let arg_3 = &args.arg_3;
    let arg_4 = &args.arg_4;

    match command {
        "check" => {
            commands::check(&config, &root_path);
        },
        "run" => {
            if let Some(script) = arg_2 {
                commands::run(&config, &root_path, script);
            } else {
                println!("To use {} you must also pass a script", highlight(&String::from("run")));
            }
        },
        "workspace" => {
            commands::workspace(
                &config,
                &root_path,
                &arg_2,
                &arg_3,
                &arg_4,
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

    println!("\nDone ✨");
}
