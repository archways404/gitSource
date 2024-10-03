use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Struct representing the Git source details.
#[derive(Serialize, Deserialize, Debug)]
pub struct GitSource {
    pub version: String,
    pub username: String,
    pub repo: String,
    pub branch: String,
}

/// Creates the gitSource.json file with the provided details.
pub fn create_git_source_file(version: &str, username: &str, repo: &str, branch: &str) -> std::io::Result<()> {
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

/// Reads and deserializes the gitSource.json file.
pub fn read_git_source_file() -> Result<GitSource, Box<dyn std::error::Error>> {
    let path = Path::new("gitSource.json");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let git_source: GitSource = serde_json::from_str(&contents)?;
    Ok(git_source)
}
