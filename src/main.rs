mod cli;
mod file_operations;
mod version;
mod git;

use cli::build_cli;
use file_operations::{create_git_source_file, read_git_source_file};
use git::{create_github_url, run_git_fetch, fetch_version_from_url};
use version::{increment_patch_version, increment_major_version};
use std::path::Path;
use std::io::{self, Write};

fn main() {
    let matches = build_cli();

    if let Some(_) = matches.subcommand_matches("init") {
        if Path::new("gitSource.json").exists() {
            println!("gitSource.json already exists.");
        } else {
            let username = read_input("Enter the GitHub username: ");
            let repo = read_input("Enter the repository name: ");
            let branch = read_input("Enter the branch name: ");
            let version = read_input("Enter the version: ");
            if let Err(e) = create_git_source_file(&version, &username, &repo, &branch) {
                eprintln!("Failed to create gitSource.json: {}", e);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("update") {
        match read_git_source_file() {
            Ok(mut git_source) => {
                git_source.version = increment_patch_version(&git_source.version);
                if let Err(e) = create_git_source_file(&git_source.version, &git_source.username, &git_source.repo, &git_source.branch) {
                    eprintln!("Failed to update gitSource.json: {}", e);
                } else {
                    println!("Updated version to {}", git_source.version);
                }
            }
            Err(e) => eprintln!("Error reading gitSource.json: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("update-major") {
        match read_git_source_file() {
            Ok(mut git_source) => {
                git_source.version = increment_major_version(&git_source.version);
                if let Err(e) = create_git_source_file(&git_source.version, &git_source.username, &git_source.repo, &git_source.branch) {
                    eprintln!("Failed to update gitSource.json: {}", e);
                } else {
                    println!("Updated major version to {}", git_source.version);
                }
            }
            Err(e) => eprintln!("Error reading gitSource.json: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("fetch") {
        match read_git_source_file() {
            Ok(git_source) => {
                let url = create_github_url(&git_source.username, &git_source.repo, &git_source.branch);
                println!("Generated URL: {}", url);

                let remote_version = match fetch_version_from_url(&url) {
                    Ok(version) => version,
                    Err(e) => {
                        eprintln!("Failed to fetch version: {}", e);
                        return;
                    }
                };

                if remote_version > git_source.version {
                    println!("The remote version ({}) is newer than the local version ({}).", remote_version, git_source.version);
                    if let Err(e) = run_git_fetch() {
                        eprintln!("Failed to run 'git fetch': {}", e);
                    }
                } else {
                    println!("The local version is up to date.");
                }
            }
            Err(e) => eprintln!("Error reading gitSource.json: {}", e),
        }
    } else {
        println!("No subcommand was used.");
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
