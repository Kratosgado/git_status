// File: main.rs

use colored::Colorize;

mod folder_scanner;
mod git;
fn main() {
    let root_folder = std::env::args()
        .nth(1)
        .expect("No root folder provided".red().to_string().as_str());
    match git::is_git_installed() {
        true => folder_scanner::scan_folder(root_folder.as_str()),
        false => println!("{}", "Git is not installed".red()),
    }
}
