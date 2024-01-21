use std::env;
use std::fs;

pub fn move_into_directory(directory_path: &str) {
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Check if it's a directory
                if entry.path().is_dir() {
                    // Move into the subdirectory
                    let subdirectory_path = entry.path();
                    // Use subdirectory_path for further operations or navigation
                    println!("Moved into: {}", subdirectory_path.display());
                }
            }
        }
    } else {
        eprintln!("Error reading directory");
    }
}

pub fn move_out_of_directory(parent_directory: &str) {
    if let Err(err) = env::set_current_dir(parent_directory) {
        eprintln!("Error changing directory: {}", err);
    } else {
        println!("Moved out of: {}", parent_directory);
    }
}
