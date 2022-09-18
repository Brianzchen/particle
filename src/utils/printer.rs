use colored::Colorize;

pub fn error(message: &str) {
    eprintln!("{}", "==Error==".bold().red());
    eprintln!("{} {}", ">>".red(), message);
}

pub fn highlight(value: &String) -> String {
    format!("`{}`", format!("{}", value).bold().green())
}
