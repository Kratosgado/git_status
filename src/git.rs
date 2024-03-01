// git.rs

use std::process::Command;
use std::io::{self, Write};

pub fn is_git_installed() -> bool {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .expect("Failed to execute git --version");

    output.status.success()
}

pub fn is_git_repo(path: &std::path::Path) -> bool {
    let output = Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output()
        .expect("Failed to execute git rev-parse --is-inside-work-tree");

    output.status.success()
}

pub fn check_git_status(path: &std::path::Path) {
    let output = Command::new("git")
        .arg("status")
        .current_dir(path)
        .output()
        .expect("Failed to execute git status");

    println!("status: {}", String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        println!("Failed to execute git status");
    } else {
        let mut input = String::new();
        println!("Do you want to add, commit and push changes? (y/n)");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim() == "y" {
            add_commit_push(path);
        } 
    }
}

pub fn add_commit_push(path: &std::path::Path){
    let output = Command::new("git")
        .args(&["add", "."])
        .current_dir(path)
        .output()
        .expect("Failed to execute git command");
    if !output.status.success() {
        println!("Failed to add changes to git");
        return;
    }

    let output = Command::new("git")
        .args(&["commit", "-m", "auto commit"])
        .current_dir(path)
        .output()
        .expect("Failed to execute git command");
    if !output.status.success() {
        println!("Failed to commit changes to git");
        return;
    }

    let output = Command::new("git")
        .args(&["push"])
        .current_dir(path)
        .output()
        .expect("Failed to execute git command");
    if !output.status.success() {
        println!("Failed to push changes to git");
        return;
    }

    println!("Changes added, committed and pushed to git");
}