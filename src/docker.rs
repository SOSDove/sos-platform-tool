use std::io::Read;
use std::process::{Command, Stdio};
use crate::{print_info};
use crate::messages::{print_error, print_success};

pub async fn pull_docker_image() -> Result<(), Box<dyn std::error::Error>> {
    print_info("Pulling Docker Image");
    let output = Command::new("docker")
        .arg("pull")
        .arg("quay.sos.eu/edbafjdu/sos-platform-tool")
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to pull image: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}

pub async fn run_docker_container(ext: &str, path: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_info("Running Docker");

    let mount_path = format!("{}:/files_to_encrypt/", path);

    let output = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("encrypt-decrypt-container")
        .arg("-v")
        .arg(mount_path)
        .arg("-e")
        .arg("VAULT_PASSWORD=".to_owned() + key)
        .arg("sos-platform-tool-v3")
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to run container: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    print_info("Value Encrypted/Decrypted");
    Ok(())
}

pub async fn remove_container_if_present() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--filter")
        .arg("name=encrypt-decrypt-container")
        .arg("--format")
        .arg("{{.Names}}")
        .output()?;

    let container_name = String::from_utf8_lossy(&output.stdout).clone().trim().to_owned();

    if container_name == "encrypt-decrypt-container".to_string() {
        let remove_output = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg("encrypt-decrypt-container")
            .output()?;

        if !remove_output.status.success() {
            return Err(format!("Failed to remove container: {}", String::from_utf8_lossy(&remove_output.stderr)).into());
        }

        print_info("Container 'encrypt-decrypt-container' removed successfully");
    } else {
        print_info("Container 'encrypt-decrypt-container' does not exist.");
    }

    Ok(())
}

pub async fn run_docker_command(action: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vault_action = VaultAction::from_str(action).expect("Invalid action");

    let output = match vault_action {
        VaultAction::Encrypt => {
            Command::new("docker")
                .arg("exec")
                .arg("encrypt-decrypt-container") // Replace with your actual container name
                .arg("ansible-vault")
                .arg(action)
                .arg("/files_to_encrypt/generate-secrets/input/".to_owned() + file_path)
                .arg("--vault-password-file")
                .arg("/vault_password.sh")
                .arg("--encrypt-vault-id")
                .arg("default")
                .output()?
        }
        VaultAction::Decrypt => {
            Command::new("docker")
                .arg("exec")
                .arg("encrypt-decrypt-container") // Replace with your actual container name
                .arg("ansible-vault")
                .arg(action)
                .arg("/files_to_encrypt/generate-secrets/input/".to_owned() + file_path)
                .arg("--vault-password-file")
                .arg("/vault_password.sh")
                .output()?
        }
        VaultAction::PLAYBOOK => {
            print_info("Running Playbook");
            Command::new("docker")
                .arg("exec")
                .arg("encrypt-decrypt-container") // Replace with your actual container name
                .arg("ansible-playbook")
                .arg("-i")
                .arg(",localhost")
                .arg("-c")
                .arg("local")
                .arg("/files_to_encrypt/generate-secrets/".to_owned() + file_path)
                .arg("--vault-password-file")
                .arg("/vault_password.sh")
                .env("ANSIBLE_DEPRECATION_WARNINGS", "False") // Suppress deprecation warnings
                .output()?
        }
    };

    if output.status.success() {
        let success_message = format!("Successfull {} of {}", action, file_path);
        print_success(&success_message);
    } else {
        let failure_message = format!("Failed {} of {}", action, file_path);
        print_error(&failure_message);
        eprintln!("Error! Output:\n{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("Error! Output:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

enum VaultAction {
    Encrypt,
    Decrypt,
    PLAYBOOK,
}

impl VaultAction {
    fn from_str(action_str: &str) -> Option<Self> {
        match action_str {
            "encrypt" => Some(VaultAction::Encrypt),
            "decrypt" => Some(VaultAction::Decrypt),
            "run-playbook" => Some(VaultAction::PLAYBOOK),
            _ => None,
        }
    }
}
