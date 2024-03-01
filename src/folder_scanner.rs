// folder_scanner.rs

use std::path::Path;

pub fn scan_folder(root_folder: &str){
    let mut folders_to_process = vec![root_folder.to_string()];

    while let Some(folder) = folders_to_process.pop() {
        let path = Path::new(&folder);
        if path.is_dir() {
            if super::git::is_git_repo(path){
                println!("Scanning repository: {}", path.display());
                super::git::check_git_status(path);
            }
            if let Ok(entries) = path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(name) = entry.file_name().to_str() {
                            let subfolder = format!("{}/{}", folder, name);
                            folders_to_process.push(subfolder);
                        }
                    }
                }
            }
        }
    }
}