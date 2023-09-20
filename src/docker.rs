use std::process::Command;
use crate::{print_info, print_success};

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
        .arg("quay.sos.eu/edbafjdu/sos-platform-tool")
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
    // Ensure the action is either "encrypt" or "decrypt"
    let vault_action = VaultAction::from_str(action).expect("Invalid action");

    let output = match vault_action {
        VaultAction::Encrypt => {
            Command::new("docker")
                .arg("exec")
                .arg("encrypt-decrypt-container") // Replace with your actual container name
                .arg("ansible-vault")
                .arg(action)
                .arg("/files_to_encrypt/".to_owned() + file_path)
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
                .arg("/files_to_encrypt/".to_owned() + file_path)
                .arg("--vault-password-file")
                .arg("/vault_password.sh")
                .output()?
        }
    };
    // Construct the docker command


    // Handle the command output (for demonstration purposes, we'll just print it)
    if output.status.success() {
        let success_message = format!("Successfull {} of {}", action, file_path);
        print_success(&success_message);
    } else {
        eprintln!("Error! Output:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

enum VaultAction {
    Encrypt, Decrypt
}

impl VaultAction {
    fn from_str(action_str: &str) -> Option<Self> {
        match action_str {
            "encrypt" => Some(VaultAction::Encrypt),
            "decrypt" => Some(VaultAction::Decrypt),
            _ => None,
        }
    }
}

// ... [any other Docker-related functions]
