use std::collections::HashMap;
use std::io;
use crate::{docker};
use crate::cli::get_encrypt_decrypt_choice;
use crate::file_ops::{list_files_in_directory, parse_str_to_u32};
use crate::messages::{print_error, print_info, print_prompt};

pub async fn action_loop() {
    loop {
        let current_dir = std::env::current_dir().expect("Could not read current dir").to_str().unwrap().to_string();
        let mount_path = current_dir;
        let file_map = list_files_in_directory(&mount_path).expect("Could not list files");

        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            let user_input = user_input.trim().to_lowercase();

            if user_input == "x" {
                print_info("Exiting App");
                break;
            } else if user_input == "all" || user_input == "a" {
                process_all_files(&file_map).await;
                break;
            } else {
                let number_strings: Vec<&str> = user_input.split(',').collect();
                let mut invalid_choice = false;
                let mut selected_files: Vec<u32> = Vec::new();
                for num_str in number_strings {
                    match num_str.trim().parse::<u32>() {
                        Ok(value) => {
                            if file_map.contains_key(&value) {
                                selected_files.push(value);
                            } else {
                                invalid_choice = true;
                                break;
                            }
                        },
                        Err(_) => {
                            invalid_choice = true;
                            break;
                        }
                    }
                }

                if invalid_choice {
                    print_error("Invalid choice, please try again.");
                } else {
                    for file_number in selected_files {
                        print!("Processing file: {} ", file_number);
                        process_single_file(&file_number, &file_map).await;
                    }
                    break;
                }
            }
        }


        print_prompt("Would you like to do more? (X to exit) - Enter to continue");
        let mut more = String::new();
        io::stdin().read_line(&mut more).expect("Failed to read line");
        let more = more.trim();

        if more.to_lowercase() == "x" {
            print_info("Exiting App");
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

async fn process_single_file(user_input: &u32, file_map: &HashMap<u32, String>) {

    let choice = get_encrypt_decrypt_choice();
    let file = &file_map[user_input];
    docker::run_docker_command(choice, file).await.expect("Failed to run command");
}