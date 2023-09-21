
use std::path::{Path};
use std::{process};
use clap::ArgMatches;
use colored::Colorize;
use crate::cli::{interactive_encryption_mode, no_subcommand_given};
use crate::file_ops::{build_cryptographic_command, validate_path};
use crate::messages::{print_error, print_info};
use crate::processor::action_loop;

mod cli;
mod docker;
mod utils;
mod messages;
mod processor;
mod file_ops;

#[tokio::main]
async fn main() {
    let matches = cli::cli().get_matches();

    if !utils::is_docker_running() {
        print_error("Docker is not running, the tool will not work without docker, exiting");
        process::exit(1)
    }

    match matches.subcommand() {
        Some((ext, sub_matches)) => {

            if ext == "encrypt" || ext == "decrypt" {
                let (new_path, new_key) = get_path_and_key(sub_matches);

                run_tool(&new_path, &new_key).await;
            }
        }
        _ => {
            println!("No Subcommand Given");
            no_subcommand_given().await;
        }
    }
    docker::remove_container_if_present().await.expect("TODO: panic message");
}

fn get_path_and_key(sub_matches: &ArgMatches) -> (String, String) {
    let path = sub_matches.get_one::<String>("PATH");
    let key = sub_matches.get_one::<String>("KEY");

    let (new_path, new_key) =
        if path.is_none() || key.is_none() {
            interactive_encryption_mode(path.cloned(), key.cloned())
        } else {
            (path.unwrap().to_owned(), key.unwrap().to_owned())
        };
    (new_path, new_key)
}

async fn run_tool(new_path: &String, new_key: &String) {
    let path = Path::new(&new_path);

    if validate_path(path) {
        print_info("The path provided points to a valid file or directory");
    } else {
        print_error("The provided path did not point to a valid file or directory");
        docker::remove_container_if_present().await.expect("TODO: panic message");
        panic!();
    }

    let remove_container_result = docker::remove_container_if_present().await;
    match remove_container_result {
        Ok(_) => print_info("Container removed properly"),
        Err(e) => print_error("Error Removing Container"),
    }
    let cryptographic_command = build_cryptographic_command(&"Encrypt".to_string(), path, &new_key);
    let pull_container_result = docker::pull_docker_image().await;
    match pull_container_result {
        Ok(_) => print_info("Pulled image properly"),
        Err(e) => eprintln!("Error pulling image: {:?}", e),
    }

    let encrypt_decrypt_path = docker::run_docker_container(&cryptographic_command.type_of_command, &cryptographic_command.path.to_str().unwrap(), &cryptographic_command.key).await;

    action_loop().await;

    match encrypt_decrypt_path {
        Ok(_) => print_info("Ran successfully"),
        Err(e) => eprintln!("Error during running: {:?}", e),
    }
}








