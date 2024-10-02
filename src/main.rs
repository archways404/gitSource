use std::fs::File;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use std::path::Path;
use reqwest::blocking::get;
use serde_json::Value;

// Struct representing the data in the gitSource.json file
#[derive(Serialize, Deserialize, Debug)]
struct GitSource {
    version: String,
}

// Function to read user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush stdout to display prompt immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string() // Remove any surrounding whitespace
}

fn create_github_url(username: &str, repo: &str, branch: &str) -> String {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/gitSource.json",
        username, repo, branch
    );
    url
}

// Function to create and write the gitSource.json file
fn create_git_source_file(version: &str) -> std::io::Result<()> {
    let git_source = GitSource {
        version: version.to_string(),
    };

    // Convert the struct to a JSON string
    let json_data = serde_json::to_string_pretty(&git_source).unwrap();

    // Define the file path (in this case, the root directory)
    let path = Path::new("gitSource.json");

    // Create the file
    let mut file = File::create(path)?;

    // Write the JSON data to the file
    file.write_all(json_data.as_bytes())?;

    println!("gitSource.json file created with version: {}", version);
    
    Ok(())
}

// Function to read the gitSource.json file and deserialize it into a GitSource struct
fn read_git_source_file() -> Result<GitSource, Box<dyn std::error::Error>> {
    // Define the path to the gitSource.json file
    let path = Path::new("gitSource.json");

    // Open the file
    let mut file = File::open(path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON content into the GitSource struct
    let git_source: GitSource = serde_json::from_str(&contents)?;

    Ok(git_source)
}

// Function to fetch the version from the given URL
fn fetch_version_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Send a GET request to the URL
    let response = get(url)?.text()?;

    // Parse the JSON response
    let git_source: GitSource = serde_json::from_str(&response)?;

    // Return the version
    Ok(git_source.version)
}

fn main() {
    // let username = "archways404";
    // let repo = "HDAVAIL";
    // let branch = "master";
    // let version = "1.0.2";
    
    // let url = create_github_url(username, repo, branch);
    // println!("Generated URL: {}", url);

    // Get user input
    let username = read_input("Enter the GitHub username: ");
    let repo = read_input("Enter the repository name: ");
    let branch = read_input("Enter the branch name: ");
    let version = read_input("Enter the version: ");

    // Create the gitSource.json file in the root
    if let Err(e) = create_git_source_file(&version) {
        eprintln!("Failed to create gitSource.json: {}", e);
    }

    // Generate the URL
    let url = create_github_url(&username, &repo, &branch);

    // Output the result
    println!("Generated URL: {}", url);

    // Attempt to read and print the gitSource.json file
    match read_git_source_file() {
        Ok(git_source) => {
            println!("Successfully read gitSource.json:");
            println!("{:?}", git_source);
        }
        Err(e) => {
            eprintln!("Failed to read gitSource.json: {}", e);
        }
    }

    // Fetch the version from the URL and print it
    match fetch_version_from_url(&url) {
        Ok(version) => {
            println!("Version fetched from URL: {}", version);
        }
        Err(e) => {
            eprintln!("Failed to fetch version: {}", e);
        }
    }
}
