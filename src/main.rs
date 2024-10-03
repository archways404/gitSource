use std::fs::File;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use std::path::Path;
use reqwest::blocking::get;
use std::process::Command;

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

// Function to read user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

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

    println!("gitSource.json file created with version: {}, username: {}, repo: {}, branch: {}", 
             version, username, repo, branch);

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

fn run_git_pull() -> std::io::Result<()> {
    println!("Running 'git pull' to sync with the remote repository...");
    let output = Command::new("git")
        .arg("pull")
        .output()?;

    if output.status.success() {
        println!("Git pull succeeded:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Git pull failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn main() {
    let git_source = match read_git_source_file() {
        Ok(git_source) => {
            println!("Successfully read gitSource.json:");
            println!("{:?}", git_source);
            git_source
        }
        Err(_) => {
            println!("No valid gitSource.json found, asking for user input...");
            let username = read_input("Enter the GitHub username: ");
            let repo = read_input("Enter the repository name: ");
            let branch = read_input("Enter the branch name: ");
            let version = read_input("Enter the version: ");
            if let Err(e) = create_git_source_file(&version, &username, &repo, &branch) {
                eprintln!("Failed to create gitSource.json: {}", e);
                return;
            }
            GitSource {
                version,
                username,
                repo,
                branch,
            }
        }
    };

    let url = create_github_url(&git_source.username, &git_source.repo, &git_source.branch);
    println!("Generated URL: {}", url);

    let remote_version = match fetch_version_from_url(&url) {
        Ok(version) => {
            println!("Version fetched from URL: {}", version);
            version
        }
        Err(e) => {
            eprintln!("Failed to fetch version: {}", e);
            return;
        }
    };

    match compare_versions(&git_source.version, &remote_version) {
        std::cmp::Ordering::Less => {
            println!("The remote version ({}) is newer than the local version ({}).", remote_version, git_source.version);
            if let Err(e) = run_git_pull() {
                eprintln!("Failed to run 'git pull': {}", e);
            }
        }
        std::cmp::Ordering::Greater => {
            println!("The local version ({}) is newer than the remote version ({}).", git_source.version, remote_version);
        }
        std::cmp::Ordering::Equal => {
            println!("The local version and remote version are the same ({}).", git_source.version);
        }
    }
}
