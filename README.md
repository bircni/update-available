# Update Available

[![Crates.io](https://img.shields.io/crates/v/update-available.svg)](https://crates.io/crates/update-available)
[![Documentation](https://docs.rs/update-available/badge.svg)](https://docs.rs/update-available)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/bircni/update-available/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/bircni/update-available/actions/workflows/ci.yml)

A Rust library to check for updates of crates on crates.io or GitHub repositories. Get notified when newer versions of your dependencies are available with beautiful, formatted output.

## Features

- ✅ Check for updates on **crates.io**
- ✅ Check for updates on **GitHub** repositories
- ✅ Beautiful, formatted output with icons
- ✅ Easy-to-use API with comprehensive error handling
- ✅ Support for semantic versioning
- ✅ Blocking HTTP requests (with optional async support planned)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
update-available = "0.1.0"
```

or use `cargo add`:

```bash
cargo add update-available
```

## Quick Start

### Check for crates.io updates

```rust
use update_available::{get_check, Source};

match get_check("serde", "1.0.0", Source::CratesIo { url: None }) {
    Ok(Some(info)) => println!("Update available:\n{}", info),
    Ok(None) => println!("No update available"),
    Err(e) => eprintln!("Error checking for updates: {}", e),
}
```

### Check for GitHub repository updates

```rust
use update_available::{get_check, Source};

match get_check("my-repo", "1.0.0", Source::Github("username".to_string())) {
    Ok(Some(info)) => println!("Update available:\n{}", info),
    Ok(None) => println!("No update available"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Check for Gitea repository updates

```rust
use update_available::{get_check, Source};

match get_check("my-repo", "1.0.0", Source::Gitea {
    user: "username".to_string(),
    base_url: "https://gitea.example.com".to_string(),
    token: Some("your-auth-token".to_string()), // Optional authentication token
}) {
    Ok(Some(info)) => println!("Update available:\n{}", info),
    Ok(None) => println!("No update available"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Check for updates from custom registry (e.g., kellnr)

```rust
use update_available::{get_check, Source};

match get_check("my-crate", "1.0.0", Source::CratesIo { 
    url: Some("https://my-kellnr-registry.com".to_string())
}) {
    Ok(Some(info)) => println!("Update available:\n{}", info),
    Ok(None) => println!("No update available"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Convenience function for direct printing

```rust
use update_available::{print_check, Source};

// Check crates.io and print result
print_check("serde", "1.0.0", Source::CratesIo { url: None });

// Check GitHub and print result
print_check("my-repo", "0.1.0", Source::Github("username".to_string()));

// Check Gitea with token and print result
print_check("my-repo", "0.1.0", Source::Gitea {
    user: "username".to_string(),
    base_url: "https://gitea.example.com".to_string(),
    token: Some("your-token".to_string()),
});
```

## Example Output

When an update is available, you'll see beautifully formatted output like this:

```text
🚀  A new version is available!
🔖  Latest version: 1.0.210
📝  Changelog:
    • Fixed critical security vulnerability
    • Improved performance by 15%
    • Added new serialization features
🌐  More info: https://crates.io/crates/example
````

When you're already using the latest version:

```text
✅  You're already using the latest version! (1.0.210)
```

## API Reference

### Functions

- **`get_check(name, current_version, source)`** - Check for updates and return formatted string
- **`print_check(name, current_version, source)`** - Convenience function that prints results directly if update available
- **`print_check_force(name, current_version, source)`** - Convenience function that always prints results

### Types

- **`UpdateInfo`** - Contains update information including version details and changelog
- **`Source`** - Enum for specifying update source:
  - `CratesIo { url: Option<String> }` - Check crates.io or custom registry
  - `Github(String)` - Check GitHub repository
  - `Gitea { user: String, base_url: String, token: Option<String> }` - Check Gitea repository with optional auth

### Properties of `UpdateInfo`

- `is_update_available: bool` - Whether an update is available
- `latest_version: Version` - The latest available version
- `changelog: Option<String>` - Optional changelog information
- `url: String` - URL for more information

## Examples

You can run the included examples:

```bash
# Check crates.io for updates
cargo run --example example

# Run with specific features
cargo run --features blocking --example example
```

## Features

This crate uses feature flags to control functionality:

- **`blocking`** (default) - Enables blocking HTTP requests using `ureq`

```toml
[dependencies]
# Default features (includes blocking)
update-available = "0.1.0"

# Only blocking features
update-available = { version = "0.1.0", features = ["blocking"] }

# No default features
update-available = { version = "0.1.0", default-features = false }
```

## Error Handling

The library uses `anyhow::Error` for comprehensive error handling. Common error scenarios include:

- Network connectivity issues
- Invalid version strings
- API rate limiting
- Repository not found
- Malformed API responses

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Async support with `tokio` and `reqwest`
- [ ] Custom output formatting
- [ ] Support for other sources (e.g. GitLab)
- [x] Support for Gitea repositories
