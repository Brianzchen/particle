mod commands;
mod constants;
mod utils;
use colored::Colorize;
use clap::Parser;

use crate::utils::{get_config,get_workspaces_data};

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

    // parse workspaces here
    // to create a collection of workspaces and their package details
    get_workspaces_data(&config);

    match command {
        "check" => {
            commands::check(&config, &root_path);
        },
        "workspace" => {
            println!("pull the package name");
            if args.arg_2 == None {
                println!("You've called `workspace` without the --package option");
                println!("Try again");
                return;
            }
            println!("{:?}", args.arg_2);
            // Go check if the package name actually exists
        }
        _ => {
            println!("{}, try `{}` for more information",
                format!("Invalid command given").red().bold(),
                format!("particle help").green()
            );
        }
    }
}
