// File: main.rs

mod git;
mod folder_scanner;
fn main() {
    let root_folder = std::env::args().nth(1).expect("No root folder provided");
    match git::is_git_installed() {
        true => folder_scanner::scan_folder(root_folder.as_str())        ,
        false => println!("Git is not installed"),
    }
}
