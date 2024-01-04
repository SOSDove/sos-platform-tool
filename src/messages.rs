use colored::Colorize;

pub fn print_info(message: &str) {
    println!("{}", format!("[INFO] - {}", message).cyan());
}

pub fn print_error(message: &str) {
    println!("{}", format!("[ERROR] - {}", message).red());
}

pub fn print_success(message: &str) {
    println!("{}", format!("[SUCCESS] - {}", message).green());
}

pub fn print_prompt(message: &str) {
    println!("{}", format!("[PROMPT] - {}", message).blue());
}

pub fn print_selection_option(option: &str) {
    println!("{}", format!("    {}", option).yellow());
}
