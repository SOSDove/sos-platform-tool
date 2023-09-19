# SOS Platform Tool

The SOS Platform Tool is an internal utility designed to provide encryption and decryption functionalities for files and folders. It leverages Docker to run the cryptographic operations using a specific Docker image.

## Features

- **Interactive Mode**: If no path or key is provided, the tool will prompt the user for input.
- **Docker Integration**: Uses the Docker image `quay.sos.eu/edbafjdu/ansible-encrypt-decrypt` to perform encryption and decryption.
- **Container Management**: Before running a new encryption/decryption task, the tool checks for the presence of a container named `encrypt-decrypt-container` and removes it if found.

## Usage

```bash
sos-platform-tool [SUBCOMMAND]
```

### Subcommands:

- **encrypt**: Encrypts a given file or folder.
    - **--path PATH**: The path to the file or folder. (Optional in interactive mode)
    - **--key KEY**: The key to use for encryption. (Optional in interactive mode)

- **decrypt**: Decrypts a given file or folder.
    - **--path PATH**: The path to the file or folder. (Optional in interactive mode)
    - **--key KEY**: The key to use for decryption. (Optional in interactive mode)

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

To build the tool, navigate to the directory containing the code and run:

```bash
cargo build --release
```

## Getting Help

The SOS Platform Tool uses `clap` for command-line argument parsing, which provides a built-in help system. If you ever need assistance with the available commands or their syntax, you can use the following:

### Displaying General Help:

To view a list of available commands and a brief description of each:

```bash
sos-platform-tool --help
```

### Displaying Help for Specific Subcommands:

If you need detailed information about a specific subcommand, you can request help for that subcommand:

```bash
sos-platform-tool encrypt --help
```

```bash
sos-platform-tool decrypt --help
```

The help command will display a description of the subcommand, its arguments, and any available options.

## Contributing

Contributions are welcome! Please ensure that any changes made are compatible with the existing Docker image and cryptographic operations.

---

Feel free to modify or expand upon this README to better fit your project's needs!
