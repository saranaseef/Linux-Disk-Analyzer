use std::fs;
use std::path::{Path, PathBuf};

pub fn calculate_size(path: &Path) -> u64 {
    if path.is_file() {
        if let Ok(metadata) = fs::metadata(path) {
            return metadata.len();
        }
    } else if path.is_dir() {
        let mut total_size = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    total_size += calculate_size(&entry.path());
                }
            }
        }
        return total_size;
    }

    0
}

pub fn displayFileSize(path: &Path) -> String
{
    let file_size = calculate_size(&path);
    let formatted_file_size = format_file_size(file_size);
    println!("Total size: {}", formatted_file_size);
    return formatted_file_size;


}
pub fn displayFileSizeUnformatted(path: &Path) -> u64
{
    let file_size = calculate_size(&path);
    return file_size;

  
}
pub fn display_file_sizes(path: &Path) -> Vec<(PathBuf, String, u64)> {
    let mut data: Vec<(PathBuf, String, u64)> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_type = if entry_path.is_file() { "File" } else { "Folder" };
                let file_size = calculate_size(&entry_path);
                data.push((entry_path, entry_type.to_string(), file_size));
                // println!("{} {}: {}", entry_type, format_file_name(&entry_path), format_file_size(file_size));
            }
        }
    }
    return data;
}


pub fn display_total_size(path: &Path) -> String {
    let total_size = calculate_size(path);
    let formatted_total_size = format_file_size(total_size);
    println!("Total size: {}", formatted_total_size);
    return formatted_total_size;
}

pub fn display_total_size_unformatted(path: &Path) -> u64 {
    let total_size = calculate_size(path);
    return total_size;
}

fn format_file_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;
    const GB: f64 = 1024.0 * MB;

    if size < KB as u64 {
        format!("{} B", size)
    } else if size < MB as u64 {
        format!("{:.2} KB", size as f64 / KB)
    } else if size < GB as u64 {
        format!("{:.2} MB", size as f64 / MB)
    } else {
        format!("{:.2} GB", size as f64 / GB)
    }
}

fn format_file_name(file_path: &Path) -> String {
    if let Some(file_name) = file_path.file_name() {
        file_name.to_string_lossy().to_string()
    } else {
        file_path.to_string_lossy().to_string()
}
}
