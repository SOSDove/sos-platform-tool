use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;
use crate::messages::{print_prompt, print_selection_option};

pub fn validate_path(p: &Path) -> bool {
    p.is_file() || p.is_dir()
}

pub fn parse_str_to_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}

pub struct CryptographicCommand  {
    pub(crate) type_of_command: String,
    pub(crate) path: PathBuf,
    pub(crate) key: String
}

pub fn build_cryptographic_command(ext: &String, path: &Path, key: &String) -> CryptographicCommand {
    CryptographicCommand {
        type_of_command: ext.to_string(),
        path: path.to_path_buf(),
        key: key.trim().to_string(),
    }
}

pub fn list_files_in_directory(dir_path: &str) -> std::io::Result<HashMap<u32, String>> {
    let entries = fs::read_dir(dir_path)?;

    let mut count = 1;
    let mut file_map = HashMap::new();

    print_prompt("Select Files to Encrypt or Decrypt");
    print_prompt("(A) for all");
    print_prompt("Number for specific file, comma separated for multiple files");
    print_prompt("(R) to run playbook");
    print_prompt("(X) for exit");

    for entry in entries {
        let entry = entry?;
        if entry.path().is_file() {
            let file_name = entry.path().file_name().unwrap().to_string_lossy().to_string();
            let selection_option = format!("({}) - {}", count, &file_name);
            print_selection_option(&selection_option);

            file_map.insert(count, file_name);
            count += 1;
        }
    }

    Ok(file_map)
}

pub fn find_file_in_folder(folder: &str, pattern: &str) -> Option<PathBuf> {
    let search_pattern = format!("{}/{}", folder, pattern);
    for entry in glob(&search_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    return Some(path);
                }
            }
            Err(e) => println!("Error reading file: {:?}", e),
        }
    }
    None
}
