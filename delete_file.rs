use std::fs;
use std::path::Path;

pub fn delete_file(file_path: &Path) -> Result<(), std::io::Error> {
    fs::remove_file(file_path)
}
pub fn delete_folder(folder_path: &Path) -> Result<(), std::io::Error> {
    fs::remove_dir_all(folder_path)
}