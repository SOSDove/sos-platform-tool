# SOS Platform Tool

The SOS Platform Tool is an internal utility developed to provide encryption and decryption functionalities for files and folders. It utilizes Docker to execute cryptographic operations.

For a short "How To - Guide - See the last section"

## Features

- **Interactive Mode**: When the path or key isn't provided as command-line arguments, the tool enters interactive mode, prompting the user for required details.
- **Docker Integration**: Uses the Docker image `quay.sos.eu/edbafjdu/ansible-encrypt-decrypt` to perform encryption and decryption.
- **Container Management**: Before executing any cryptographic tasks, the tool checks and removes the `encrypt-decrypt-container` if it exists.
- **File Selection**: In interactive mode, users can choose to encrypt or decrypt all files in a directory or select individual files.

## Usage

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

The SOS Platform Tool is designed with simplicity and ease of use in mind, allowing for both command-line and interactive modes of operation. Here's a brief overview:

### Interactive Mode:

When the tool is invoked without specifying a path or key, it enters the interactive mode. This mode is particularly efficient for scenarios where users have multiple files to manage. Here's what happens under the hood:

1. **Docker Container Initialization**:
  - The tool starts a Docker container from the specified image (`quay.sos.eu/edbafjdu/ansible-encrypt-decrypt`).
  - This container remains running in the background, waiting for encryption or decryption requests.

2. **File Selection**:
  - Users are presented with a list of files in the default directory, allowing them to choose specific files or opt to process all files.
  - The default directory is the current working directory appended with `/input`. However, if the path is not specified, users have the opportunity to input their desired path.

3. **Encryption/Decryption**:
  - For each file selected, users are prompted to choose between encryption and decryption.
  - The Docker container receives an "exec" request for each of these operations, ensuring efficient processing without the overhead of starting and stopping the container multiple times.

4. **Batch Processing**:
  - If users have multiple files to decrypt, modify, and then encrypt again, the interactive mode is the recommended approach.
  - Users can decrypt all necessary files, make the required changes externally, and then return to the tool to encrypt them back, all while the tool and Docker container remain active.

### Benefits of Interactive Mode:

- **Efficiency**: No need to restart the tool or the Docker container for each file, which is especially beneficial when working with a large number of files.
- **Flexibility**: Users can easily switch between encrypting and decrypting operations, making it perfect for iterative file modifications.
- **User-Friendly**: The interactive prompts guide users through each step, reducing the chances of errors and ensuring a smooth experience.

For users with a large number of files (e.g., 40-50) that need decryption, modification, and subsequent encryption, the interactive mode is the ideal choice. Simply run the tool, decrypt the files, make your changes, and then encrypt them again without closing the tool.

---
