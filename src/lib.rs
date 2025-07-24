use crate::data::{UpdateAvailable, UpdateInfo};

mod data;
mod logic;

#[cfg(test)]
mod test;

/// A user identifier for GitHub repositories.
type User = String;

/// Represents the source from which to check for updates.
pub enum Source {
    /// Check for updates on crates.io.
    /// Optionally specify a custom registry URL (e.g., for kellnr or other local mirrors).
    CratesIo { 
        /// Custom registry URL. If None, uses default crates.io URL.
        url: Option<String> 
    },
    /// Check for updates on GitHub for a specific user.
    Github(User),
    /// Check for updates on Gitea for a specific user and Gitea URL.
    /// Optionally specify an authentication token.
    Gitea { 
        /// The Gitea username or organization that owns the repository.
        user: String,
        /// The base URL of the Gitea instance (e.g., https://gitea.example.com).
        base_url: String,
        /// Optional authentication token for private repositories or higher rate limits.
        token: Option<String>,
    },
}

/// Gets update information for a package from the specified source as a formatted string.
///
/// This function checks for updates and returns the result as a formatted string
/// that can be used by the caller. Returns `None` if no update is available.
///
/// # Arguments
///
/// * `name` - The name of the package to check
/// * `current_version` - The current version string (e.g., "1.0.0")
/// * `source` - The source to check for updates
///
/// # Returns
///
/// Returns `Ok(Some(String))` with formatted update information if an update is available,
/// `Ok(None)` if no update is available, or `Err(anyhow::Error)` if the check fails.
///
/// # Examples
///
/// ```rust
/// use update_available::{get_check, Source};
///
/// // Check crates.io
/// match get_check("serde", "1.0.0", Source::CratesIo { url: None }) {
///     Ok(Some(info)) => println!("Update available: {}", info),
///     Ok(None) => println!("No update available"),
///     Err(e) => eprintln!("Error checking for updates: {}", e),
/// }
///
/// // Check GitHub
/// match get_check("my-repo", "0.1.0", Source::Github("username".to_string())) {
///     Ok(Some(info)) => println!("{}", info),
///     Ok(None) => println!("No update available"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
///
/// // Check Gitea with token
/// match get_check("my-repo", "0.1.0", Source::Gitea {
///     user: "username".to_string(),
///     base_url: "https://gitea.example.com".to_string(),
///     token: Some("your-token".to_string()),
/// }) {
///     Ok(Some(info)) => println!("{}", info),
///     Ok(None) => println!("No update available"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_check(name: &str, current_version: &str, source: Source) -> anyhow::Result<Option<String>> {
    let result = match source {
        Source::CratesIo { url } => {
            let update_available = UpdateAvailable::new(name, current_version);
            update_available.crates_io_with_url(url.as_deref())
        },
        Source::Github(user) => check_github(name, &user, current_version),
        Source::Gitea { user, base_url, token } => {
            let update_available = UpdateAvailable::new(name, current_version);
            update_available.gitea_with_token(&user, &base_url, token.as_deref())
        },
    };
    
    match result {
        Ok(info) if info.is_update_available => Ok(Some(format!("{}", info))),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Prints update information for a package from the specified source.
///
/// This is a convenience function that checks for updates and prints the result
/// directly to stdout even if no update is available. To show the user
/// the current status of the package, including whether an update is available.
///
/// # Arguments
///
/// * `name` - The name of the package to check
/// * `current_version` - The current version string (e.g., "1.0.0")
/// * `source` - The source to check for updates
///
/// # Examples
///
/// ```rust
/// use update_available::{print_check_force, Source};
///
/// // Check crates.io
/// print_check_force("serde", "1.0.0", Source::CratesIo { url: None });
///
/// // Check GitHub
/// print_check_force("my-repo", "0.1.0", Source::Github("username".to_string()));
///
/// // Check Gitea with token
/// print_check_force("my-repo", "0.1.0", Source::Gitea {
///     user: "username".to_string(),
///     base_url: "https://gitea.example.com".to_string(),
///     token: Some("your-token".to_string()),
/// });
/// ```
pub fn print_check_force(name: &str, current_version: &str, source: Source) {
    match get_check(name, current_version, source) {
        Ok(Some(info)) => println!("{}", info),
        Ok(None) => println!("No updates available for {name}@{current_version}"),
        Err(e) => eprintln!("Error checking for updates: {e}"),
    }
}

/// Prints update information for a package from the specified source.
///
/// This is a convenience function that checks for updates and prints the result
/// directly to stdout if an update is available.
///
/// # Arguments
///
/// * `name` - The name of the package to check
/// * `current_version` - The current version string (e.g., "1.0.0")
/// * `source` - The source to check for updates
///
/// # Examples
///
/// ```rust
/// use update_available::{print_check, Source};
///
/// // Check crates.io
/// print_check("serde", "1.0.0", Source::CratesIo { url: None });
///
/// // Check GitHub  
/// print_check("my-repo", "0.1.0", Source::Github("username".to_string()));
///
/// // Check Gitea with token
/// print_check("my-repo", "0.1.0", Source::Gitea {
///     user: "username".to_string(),
///     base_url: "https://gitea.example.com".to_string(),
///     token: Some("your-token".to_string()),
/// });
/// ```
pub fn print_check(name: &str, current_version: &str, source: Source) {
    match get_check(name, current_version, source) {
        Ok(Some(info)) => println!("{}", info),
        Ok(None) => {}, // Only print if update is available
        Err(_) => {}, // Silently ignore errors
    }
}

/// Checks for updates on GitHub for the specified repository.
///
/// This function queries the GitHub API to check if a newer version
/// of the specified repository is available.
///
/// # Arguments
///
/// * `name` - The name of the repository to check
/// * `user` - The GitHub username or organization that owns the repository
/// * `current_version` - The current version string (e.g., "1.0.0")
///
/// # Returns
///
/// Returns a `Result<UpdateInfo, anyhow::Error>` containing update information
/// if successful, or an error if the check fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The network request fails
/// * The GitHub API returns an error
/// * The version strings cannot be parsed
/// * The response format is unexpected
/// * The repository does not exist or has no releases
///
/// # Examples
///
/// ```ignore
/// use update_available::check_github;
///
/// match check_github("my-repo", "username", "1.0.0") {
///     Ok(info) => println!("{}", info),
///     Err(e) => eprintln!("Error checking for updates: {}", e),
/// }
/// ```
fn check_github(name: &str, user: &str, current_version: &str) -> anyhow::Result<UpdateInfo> {
    let update_available = UpdateAvailable::new(name, current_version);
    update_available.github(user)
}
