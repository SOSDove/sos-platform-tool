use std::process::Command;
use crate::print_info;

pub fn is_docker_running() -> bool {
    print_info("Checking if Docker is running");
    let output = Command::new("docker")
        .arg("info")
        .output()
        .expect("Failed to execute command");

    output.status.success()
}