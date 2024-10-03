use clap::{Command};

/// Builds the CLI structure using Clap.
pub fn build_cli() -> clap::ArgMatches {
    Command::new("gs")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("CLI tool to manage git source versioning")
        .subcommand(Command::new("init").about("Initialize gitSource.json if it doesn't exist"))
        .subcommand(Command::new("update").about("Increment the version by 1"))
        .subcommand(Command::new("update-major").about("Increment the major version"))
        .subcommand(Command::new("fetch").about("Compare and fetch if remote version is newer"))
        .get_matches()
}
