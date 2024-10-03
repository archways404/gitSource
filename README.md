
# GitSource

GitSource is a Rust CLI tool designed to manage versioning and synchronization of Git repositories. The tool automates fetching and updating Git repositories, comparing version information, and more.

## Features

- **`init`**: Initializes a `gitSource.json` file that contains information about the Git repository, including username, repo name, branch, and version. If the file already exists, it will skip the initialization.
- **`update`**: Increments the version number by updating the patch version (e.g., from `0.0.1` to `0.0.2`).
- **`update-major`**: Increments the major version number (e.g., from `0.7.2` to `1.0.0`).
- **`fetch`**: Compares the local version with the remote version fetched from GitHub, and runs `git fetch` if the remote version is newer.

## Usage

### 1. Initialize gitSource.json

Use the `init` subcommand to create a `gitSource.json` file. If the file already exists, it won't prompt for input.

```bash
gs init
```

### 2. Update Version

Use the `update` subcommand to increment the patch version:

```bash
gs update
```

For example, this will change the version from `0.0.1` to `0.0.2`.

### 3. Update Major Version

Use the `update-major` subcommand to increment the major version:

```bash
gs update-major
```

For example, this will change the version from `0.7.2` to `1.0.0`.

### 4. Fetch Remote Version

Use the `fetch` subcommand to compare the local version with the remote version. If the remote version is newer, it will automatically fetch the updates:

```bash
gs fetch
```

## Installation

To install GitSource, follow these steps:

1. Install rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
2. Verify that rust and cargo is installed by running:
```bash
rustc --version
cargo --version
```

3. Run the installation script:
```bash
cargo install gitSource@0.0.1
```

4. Use the cli:
```bash
gs [argument]
```

If you wish to manually build the program:

1. Clone the repository:
```bash
git clone https://github.com/yourusername/gitsource.git
```

2. Navigate to the project directory:

```bash
cd gitsource
```

3. Build the project:

```bash
cargo build --release
```

4. Install the binary globally:

```bash
sudo mv target/release/gs /usr/local/bin/
```

Now, you can use the `gs` command from anywhere on your system.

## Contributions

Contributions are welcome! Feel free to fork the repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
