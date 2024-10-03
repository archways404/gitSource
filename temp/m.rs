use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use reqwest::blocking::get;
use std::process::Command as ShellCommand;

#[derive(Serialize, Deserialize, Debug)]
struct GitSource {
    version: String,
    username: String,
    repo: String,
    branch: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoteGitSource {
    version: String,
}

// Function to create a GitHub URL
fn create_github_url(username: &str, repo: &str, branch: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/gitSource.json",
        username, repo, branch
    )
}

fn create_git_source_file(version: &str, username: &str, repo: &str, branch: &str) -> std::io::Result<()> {
    let git_source = GitSource {
        version: version.to_string(),
        username: username.to_string(),
        repo: repo.to_string(),
        branch: branch.to_string(),
    };

    let json_data = serde_json::to_string_pretty(&git_source).unwrap();
    let path = Path::new("gitSource.json");
    let mut file = File::create(path)?;
    file.write_all(json_data.as_bytes())?;

    println!(
        "gitSource.json file created with version: {}, username: {}, repo: {}, branch: {}",
        version, username, repo, branch
    );

    Ok(())
}

fn read_git_source_file() -> Result<GitSource, Box<dyn std::error::Error>> {
    let path = Path::new("gitSource.json");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let git_source: GitSource = serde_json::from_str(&contents)?;
    Ok(git_source)
}

fn fetch_version_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let remote_source: RemoteGitSource = serde_json::from_str(&response)?;
    Ok(remote_source.version)
}

fn compare_versions(local_version: &str, remote_version: &str) -> std::cmp::Ordering {
    let local_parts: Vec<&str> = local_version.split('.').collect();
    let remote_parts: Vec<&str> = remote_version.split('.').collect();

    for (local_part, remote_part) in local_parts.iter().zip(remote_parts.iter()) {
        let local_number = local_part.parse::<u32>().unwrap_or(0);
        let remote_number = remote_part.parse::<u32>().unwrap_or(0);

        match local_number.cmp(&remote_number) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }

    std::cmp::Ordering::Equal
}

fn run_git_fetch() -> std::io::Result<()> {
    println!("Running 'git fetch' to sync with the remote repository...");
    let output = ShellCommand::new("git").arg("fetch").output()?;

    if output.status.success() {
        println!("Git fetch succeeded:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Git fetch failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn increment_patch_version(version: &str) -> String {
    let mut parts: Vec<u32> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    if parts.len() == 3 {
        parts[2] += 1;
    }
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
}

fn increment_minor_version(version: &str) -> String {
    let mut parts: Vec<u32> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    if parts.len() == 3 {
        parts[1] += 1;
        parts[2] = 0;
    }
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
}

fn increment_major_version(version: &str) -> String {
    let mut parts: Vec<u32> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
    if parts.len() == 3 {
        parts[0] += 1;
        parts[1] = 0;
        parts[2] = 0;
    }
    format!("{}.{}.{}", parts[0], parts[1], parts[2])
}

fn main() {
    let matches = Command::new("gs")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("CLI tool to manage git source versioning")
        .subcommand(Command::new("init").about("Initialize gitSource.json if it doesn't exist"))
        .subcommand(Command::new("update").about("Increment the version by 1"))
        .subcommand(Command::new("update-major").about("Increment the major version"))
        .subcommand(Command::new("fetch").about("Compare and fetch if remote version is newer"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        // Init logic: Check if gitSource.json exists, prompt user if it doesn't
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
        // Update logic: Increment patch version
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
        // Update major logic: Increment major or minor version
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
        // Fetch logic: Compare local and remote version, run `git fetch` if necessary
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

                match compare_versions(&git_source.version, &remote_version) {
                    std::cmp::Ordering::Less => {
                        println!("The remote version ({}) is newer than the local version ({}).", remote_version, git_source.version);
                        if let Err(e) = run_git_fetch() {
                            eprintln!("Failed to run 'git fetch': {}", e);
                        }
                    }
                    _ => println!("The local version is up to date."),
                }
            }
            Err(e) => eprintln!("Error reading gitSource.json: {}", e),
        }
    } else {
        println!("No subcommand was used.");
    }
}

// Helper function to read user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
