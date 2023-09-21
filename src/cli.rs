use std::io;
use clap::{arg, Command};
use crate::messages::{print_error, print_info, print_prompt};
use crate::{docker, run_tool};

pub fn cli() -> Command {
    Command::new("sos-platform-tool")
        .alias("spt")
        .long_about("Running the tool with no commands automatically enters interactive mode")
        .after_help("Note: You can run this tool without any subcommands to enter interactive mode")
        .version("1.0.0")
        .about("An SOS Internal Platform tool")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("encrypt")
                .about("encrypts a given file or folder")
                .arg(arg!(<PATH> "The path to the file or folder").long("path").required(false))
                .arg(arg!(<KEY> "The key to use for encryption").long("key").required(false))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("decrypt")
                .about("decrypt a given file or folder")
                .arg(arg!(<PATH> "The path to the filer or folder").long("path").required(false))
                .arg(arg!(<KEY> "The key to use for the decryption").long("key").required(false))
                .arg_required_else_help(false),
        )
}

pub fn get_encrypt_decrypt_choice() -> &'static str {
    loop {
        print_prompt("(E)ncrypt or (D)ecrypt?");
        let mut encrypt_decrypt_choice = String::new();
        io::stdin().read_line(&mut encrypt_decrypt_choice).expect("Failed to read line");
        let encrypt_decrypt_choice = encrypt_decrypt_choice.trim().to_lowercase();

        match encrypt_decrypt_choice.as_str() {
            "e" => {
                return "encrypt";
            },
            "d" => {
                return "decrypt";
            },
            _ => {
                print_error("Invalid choice, please try again.");
            }
        }
    }
}
pub async fn no_subcommand_given() {
    let (new_path, new_key) =
        interactive_encryption_mode(None, None);
    run_tool(&new_path, &new_key).await;
}

pub fn interactive_encryption_mode(path: Option<String>, key: Option<String>) -> (String, String) {
    let mut new_path = String::new();
    let mut new_key = String::new();
    if path.is_none() {
        let current_dir = std::env::current_dir().expect("Could not find current dir").to_str().unwrap().to_string();
        let prompt_message = format!("Detected empty path, default is {}, press enter to use default or enter a new path:", current_dir.clone());
        print_prompt(&prompt_message);
        io::stdin().read_line(&mut new_path).expect("Failed to read path");
        new_path = new_path.trim().to_string();

        if new_path.is_empty() {
            new_path = current_dir;
        }
    } else {
        new_path = path.unwrap();
    }

    if key.is_none() {
        print_prompt("Detected empty key, please enter key:");
        io::stdin().read_line(&mut new_key).expect("Failed to read key");
        new_key = new_key.trim().to_string();
    } else {
        new_key = key.unwrap();
    }
    print_info("Entering Interactive Encrypt Mode");

    (new_path.to_string(), new_key.to_string())
}