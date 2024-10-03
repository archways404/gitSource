use reqwest::blocking::get;
use std::process::Command as ShellCommand;
use std::io;

/// Creates a GitHub URL to fetch the gitSource.json file.
pub fn create_github_url(username: &str, repo: &str, branch: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/gitSource.json",
        username, repo, branch
    )
}

/// Fetches the remote version from the GitHub URL.
pub fn fetch_version_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?.text()?;
    let remote_source: super::file_operations::GitSource = serde_json::from_str(&response)?;
    Ok(remote_source.version)
}

/// Runs `git fetch` command to sync the local repository with the remote.
pub fn run_git_fetch() -> std::io::Result<()> {
    println!("Running 'git fetch' to sync with the remote repository...");
    let output = ShellCommand::new("git").arg("fetch").output()?;

    if output.status.success() {
        println!("Git fetch succeeded:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Git fetch failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
