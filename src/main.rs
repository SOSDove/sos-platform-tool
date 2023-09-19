
use std::path::{Path,PathBuf};
use std::{fs, io};
use std::collections::HashMap;

mod cli;
mod docker;

#[tokio::main]
async fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some((ext, sub_matches)) => {

            if ext == "encrypt" || ext == "decrypt" {
                let path = sub_matches.get_one::<String>("PATH");
                let key = sub_matches.get_one::<String>("KEY");

                let (new_path, new_key) = 
                if path.is_none() || key.is_none() {
                    interactive_encryption_mode(path.cloned(), key.cloned())
                } else {
                    (path.unwrap().to_owned(), key.unwrap().to_owned())
                };

                    let path = Path::new(&new_path);

                    if validate_path(path) {
                        println!("The path points to a valid file or directory.");
                     } else {
                        panic!("The provided path did not point to a valid file or directory");
                     }

                println!("Key: {} - Path: {}", new_key, new_path);
                let remove_container_result = docker::remove_container_if_present().await;
                match remove_container_result {
                    Ok(_) => println!("Container removed properly"),
                    Err(e) => eprintln!("Error removing container: {:?}", e),
                }
                let cryptographic_command = build_cryptographic_command(&"Encrypt".to_string(), path, &new_key);
                let pull_container_result = docker::pull_docker_image().await;
                match pull_container_result {
                    Ok(_) => println!("Pulled image properly"),
                    Err(e) => eprintln!("Error pulling image: {:?}", e),
                }

                let encrypt_decrypt_path = docker::run_docker_container(&cryptographic_command.type_of_command, &cryptographic_command.path.to_str().unwrap(), &cryptographic_command.key).await;

                action_loop().await;

                match encrypt_decrypt_path {
                    Ok(_) => println!("Ran successfully"),
                    Err(e) => eprintln!("Error during running: {:?}", e),
                }
            }
        }
        _ => {
            println!("No Subcommand Given");
            no_subcommand_given();
        }// If all subcommands are defined above, anything else is unreachable!()
    }
    docker::remove_container_if_present().await.expect("TODO: panic message");
}

async fn action_loop() {
    loop {
        let current_dir = std::env::current_dir().expect("Could not read current dir").to_str().unwrap().to_string();
        let mount_path = current_dir + "/input";
        let file_map = list_files_in_directory(&mount_path).expect("Could not list files");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input.to_lowercase() == "all" || user_input == "a" {
            process_all_files(&file_map).await;
        } else {
            process_single_file(user_input, &file_map).await;
        }

        println!("Would you like to do more? (Type 'I am done' to exit)");
        let mut more = String::new();
        io::stdin().read_line(&mut more).expect("Failed to read line");
        let more = more.trim();

        if more.to_lowercase() == "i am done" {
            break;
        }
    }
}

async fn process_all_files(file_map: &HashMap<u32, String>) {
    let choice = get_encrypt_decrypt_choice();
    for file in file_map.values() {
        docker::run_docker_command(choice, file).await.expect("Failed to run command");
    }
}

async fn process_single_file(user_input: &str, file_map: &HashMap<u32, String>) {
    let user_input_as_u32: Option<u32> = match parse_str_to_u32(user_input) {
        Ok(value) => Some(value),
        Err(e) => {
            println!("Failed to parse string: {}", e);
            None
        },
    };

    let choice = get_encrypt_decrypt_choice();
    let file = &file_map[&user_input_as_u32.expect("Value not provided")];
    docker::run_docker_command(choice, file).await.expect("Failed to run command");
}

fn get_encrypt_decrypt_choice() -> &'static str {
    println!("(E)ncrypt or (D)ecrypt?");
    let mut encrypt_decrypt_choice = String::new();
    io::stdin().read_line(&mut encrypt_decrypt_choice).expect("Failed to read line");
    let encrypt_decrypt_choice = encrypt_decrypt_choice.trim();

    match encrypt_decrypt_choice {
        "e" => "encrypt",
        "d" => "decrypt",
        _ => panic!("Invalid choice"), // You can handle this more gracefully if needed
    }
}

fn build_cryptographic_command(ext: &String, path: &Path, key: &String) -> CryptographicCommand {
        CryptographicCommand {
        type_of_command: ext.to_string(),
        path: path.to_path_buf(),
        key: key.trim().to_string(),
    }    
}


fn no_subcommand_given() {

}

fn interactive_encryption_mode(path: Option<String>, key: Option<String>) -> (String, String) {
    let mut new_path = String::new();
    let mut new_key = String::new();
    if path.is_none() {
        println!("Detected empty path, default is {}, press enter to use default or enter a new path:", "default-path");
        io::stdin().read_line(&mut new_path).expect("Failed to read path");
        new_path = new_path.trim().to_string();
    } else {
        new_path = path.unwrap();
    }

    if key.is_none() {
        println!("Detected empty key, please enter key:");
        io::stdin().read_line(&mut new_key).expect("Failed to read key");
        new_key = new_key.trim().to_string();
    } else {
        new_key = key.unwrap();
    }
    println!("Entering Interactive Encrypt Mode");

    (new_path.to_string(), new_key.to_string())
}

fn list_files_in_directory(dir_path: &str) -> std::io::Result<HashMap<u32, String>> {
    let entries = fs::read_dir(dir_path)?;

    let mut count = 1;
    let mut file_map = HashMap::new();

    println!("Select Files to Encrypt or Decrypt");
    println!("(A) for all");
    println!("Number for specific file");

    for entry in entries {
        let entry = entry?;
        if entry.path().is_file() {
            let file_name = entry.path().file_name().unwrap().to_string_lossy().to_string();
            println!("({}) - {}", count, &file_name);

            file_map.insert(count, file_name);
            count += 1;
        }
    }

    Ok(file_map)
}

fn validate_path(p: &Path) -> bool {
    p.is_file() || p.is_dir()
}

fn parse_str_to_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}

struct CryptographicCommand  {
    type_of_command: String,
    path: PathBuf,
    key: String
}
