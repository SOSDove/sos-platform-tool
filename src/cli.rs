use clap::{arg, Command};

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