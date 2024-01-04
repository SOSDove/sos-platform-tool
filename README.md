# SOS Platform Tool

The SOS Platform Tool is an internal utility developed to provide encryption, decryption, and playbook execution functionalities for files and folders. Leveraging Docker, it efficiently executes cryptographic operations and runs Ansible playbooks, catering to a variety of internal data management needs.

For a short "How To - Guide - See the last section"

## Features

- **Interactive Mode**: When the path or key isn't provided as command-line arguments, the tool enters interactive mode, prompting the user for required details.
- **Docker Integration**: Utilizes the Docker image `quay.sos.eu/edbafjdu/sos-platform-tool` for efficient execution of encryption, decryption, and playbook operations.
- **Container Management**: Before executing any cryptographic tasks or running playbooks, the tool checks and removes the `encrypt-decrypt-container` if it exists.
- **File Selection**: In interactive mode, users can select to encrypt or decrypt all files in a directory or specific individual files.
- **Playbook Execution**: Supports running Ansible playbooks, providing a streamlined process for automating complex IT application deployment and configuration tasks.

## Installation

Follow these steps to install the SOS Platform Tool on your machine:

1. **Download the Tool**:
   - Build the tool on your own by following the build guide at step 4.
   - Obtain the latest version of the SOS Platform Tool from [source/repository URL].

2. **Install Docker**:
  - Ensure Docker is installed on your machine. You can download it from the official Docker website.

3. **Set Up the Environment**:
  - Add the tool to your path variable or eq.

4. **Build the Tool (Optional)**:
  - If you have downloaded the source code, navigate to the tool's directory and run `cargo build --release` to build the tool.

5. **Verify Installation**:
  - Run `sos-platform-tool --help` to verify that the installation was successful and the tool is functioning correctly.

For more detailed instructions and troubleshooting, refer to the documentation provided with the tool or contact your system administrator.


## Usage

Usually, you would run the tool within the config folder, so for healthcare-journey, we might have a healthcare-journey-config-dev1 and I would run the tool in there if I wanted to manage secrets within that folder.

```bash
sos-platform-tool [SUBCOMMAND]
```

### Subcommands:

- **encrypt**: Encrypts a specified file or folder.
  - **--path PATH**: Specifies the path to the file or folder. If omitted, the tool defaults to the current directory and enters interactive mode.
  - **--key KEY**: Provides the encryption key. If omitted, the tool prompts for it in interactive mode.

- **decrypt**: Decrypts a specified file or folder.
  - **--path PATH**: Specifies the path to the file or folder. If omitted, the tool defaults to the current directory and enters interactive mode.
  - **--key KEY**: Provides the decryption key. If omitted, the tool prompts for it in interactive mode.

### Examples:

```bash
sos-platform-tool encrypt --path /path/to/file.txt --key mysecretkey
```

```bash
sos-platform-tool decrypt --path /path/to/encrypted/file.txt --key mysecretkey
```

## Prerequisites

- **Docker**: Ensure Docker is installed and running on your machine.

## Building

To compile the tool, navigate to the directory containing the code and execute:

```bash
cargo build --release
```

## Getting Help

The SOS Platform Tool uses `clap` for command-line argument parsing. If you need assistance regarding the available commands or their syntax:

### Display General Help:

```bash
sos-platform-tool --help
```

### Show Help for Specific Subcommands:

```bash
sos-platform-tool encrypt --help
```

```bash
sos-platform-tool decrypt --help
```

## Contributing

Contributors are welcome! Ensure that your modifications remain compatible with the existing Docker image and cryptographic operations.

---

## How It Works

The SOS Platform Tool is designed with simplicity and ease of use in mind, offering both command-line and interactive modes of operation. Here's a detailed overview:

### Interactive Mode:

When the tool is invoked without specifying a path or key, it enters interactive mode, which is particularly efficient for managing multiple files. Here's the process:

1. **Automatic Path Detection**:
  - If no path is provided, the tool automatically searches for a specific directory structure and suggests it as the top domain for operations.
  - This feature simplifies file selection by guiding the user towards the most relevant directories for their tasks.

2. **Docker Container Initialization**:
  - A Docker container is initiated from the specified image (`quay.sos.eu/edbafjdu/ansible-encrypt-decrypt`).
  - This container stays active in the background, ready to handle encryption, decryption, and playbook execution requests.

3. **File Selection**:
  - Users are shown a list of files in the default or detected directory, allowing them to select specific files or process all files.
  - The tool accommodates user input for paths when the automatic detection does not meet their specific needs.

4. **Encryption/Decryption**:
  - Users are prompted to choose between encrypting and decrypting each selected file.
  - Efficient processing is achieved as the Docker container processes each request without the need to start and stop repeatedly.

5. **Playbook Execution**:
  - The tool also supports the execution of Ansible playbooks.
  - Users can run specific playbooks against the selected files or directories, making it a versatile tool for complex configuration tasks.
  - Currently only generate-secrets are available, but other playbooks can be added at a later date.

6. **Batch Processing**:
  - For multiple files requiring decryption, modification, and re-encryption, the interactive mode streamlines the process.
  - Users can decrypt files, perform necessary modifications, and re-encrypt them using the same active tool and Docker container setup.

### Benefits of Interactive Mode:

- **Efficiency**: Reduces the need to restart the tool or Docker container for each operation, beneficial for handling numerous files.
- **Flexibility**: Easy switching between encryption, decryption, and playbook execution.
- **User-Friendly**: Guided prompts and automatic path detection enhance user experience and minimize errors.

This mode is especially beneficial for users with a large number of files (e.g., 40-50) that require decryption, modification, and subsequent encryption. The interactive mode facilitates these tasks seamlessly.

---
