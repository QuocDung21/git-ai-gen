use console::style;

pub fn success(msg: &str) {
    println!("{} {}", style("✅").green(), style(msg).bold());
}

pub fn info(msg: &str) {
    println!("{} {}", style("ℹ️").blue(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", style("⚠️").yellow().bold(), style(msg).yellow());
}

pub fn error(msg: &str) {
    eprintln!("{} {}", style("❌").red().bold(), style(msg).red());
}

pub fn system(msg: &str) {
    println!("\n{}", style(msg).magenta().bold());
}

pub fn heading(msg: &str) {
    println!("\n{}", style(msg).cyan().bold());
}

pub fn text(msg: &str) {
    println!("{}", msg);
}

pub fn green_text(msg: &str) {
    println!("{}", style(msg).green());
}

pub fn note(msg: &str) {
    println!("{}", style(msg).yellow());
}

pub fn path(msg: &str, path: &str) {
    println!("\n{} {}", style(msg).yellow().bold(), style(path).cyan());
}
