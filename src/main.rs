use std::{time::Instant, process};
use std::env;
use clap::Parser;

use particle::{run, utils};

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

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let command = &args.command[..];
    let arg_2 = &args.arg_2;
    let arg_3 = &args.arg_3;
    let arg_4 = &args.arg_4;

    let cwd = env::current_dir()
        .expect("Cannot read current dir");
    let cwd = cwd.to_str().unwrap();

    let res = run(
        cwd,
        command,
        arg_2,
        arg_3,
        arg_4,
    ).await;

    match res {
        Ok(_) => {},
        Err(e) => {
            utils::printer::error(&format!("{e}")[..]);
            process::exit(1);
        }
    }

    println!("\n✨ Done in {}ms", start.elapsed().as_millis());
}
