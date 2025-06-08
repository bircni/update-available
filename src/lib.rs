use crate::data::{UpdateAvailable, UpdateInfo};

mod data;
mod logic;

#[cfg(test)]
mod test;

/// A user identifier for GitHub repositories.
pub type User = String;

/// Represents the source from which to check for updates.
pub enum Source {
    /// Check for updates on crates.io.
    CratesIo,
    /// Check for updates on GitHub for a specific user.
    Github(User),
    /// Check for updates on Gitea for a specific user and Gitea URL.
    Gitea(User, String),
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
/// print_check("serde", "1.0.0", Source::CratesIo);
///
/// // Check GitHub
/// print_check("my-repo", "0.1.0", Source::Github("username".to_string()));
///
/// // Check Gitea
/// print_check("my-repo", "0.1.0", Source::Gitea("username".to_string(), "https://gitea.example.com".to_string()));
/// ```
pub fn print_check(name: &str, current_version: &str, source: Source) {
    let result = match source {
        Source::CratesIo => check_crates_io(name, current_version),
        Source::Github(user) => check_github(name, &user, current_version),
        Source::Gitea(user, gitea_url) => {
            let update_available = UpdateAvailable::new(name, current_version);
            update_available.gitea(&user, &gitea_url)
        }
    };
    if let Ok(info) = result {
        info.print();
    }
}

/// Checks for updates on crates.io for the specified package.
///
/// This function queries the crates.io API to check if a newer version
/// of the specified package is available.
///
/// # Arguments
///
/// * `name` - The name of the crate to check on crates.io
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
/// * The crates.io API returns an error
/// * The version strings cannot be parsed
/// * The response format is unexpected
///
/// # Examples
///
/// ```rust
/// use update_available::check_crates_io;
///
/// match check_crates_io("serde", "1.0.0") {
///     Ok(info) => println!("{}", info),
///     Err(e) => eprintln!("Error checking for updates: {}", e),
/// }
/// ```
pub fn check_crates_io(name: &str, current_version: &str) -> anyhow::Result<UpdateInfo> {
    let update_available = UpdateAvailable::new(name, current_version);
    update_available.crates_io()
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
/// ```rust
/// use update_available::check_github;
///
/// match check_github("my-repo", "username", "1.0.0") {
///     Ok(info) => println!("{}", info),
///     Err(e) => eprintln!("Error checking for updates: {}", e),
/// }
/// ```
pub fn check_github(name: &str, user: &str, current_version: &str) -> anyhow::Result<UpdateInfo> {
    let update_available = UpdateAvailable::new(name, current_version);
    update_available.github(user)
}

/// Checks for updates on Gitea for the specified repository.
///
/// This function queries the Gitea API to check if a newer version
/// of the specified repository is available.
///
/// # Arguments
///
/// * `name` - The name of the repository to check
/// * `user` - The Gitea username or organization that owns the repository
/// * `gitea_url` - The base URL of the Gitea instance (e.g., <https://gitea.example.com>)
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
/// * The Gitea API returns an error
/// * The version strings cannot be parsed
/// * The response format is unexpected
/// * The repository does not exist or has no releases
///
/// # Examples
///
/// ```rust
/// use update_available::check_gitea;
///
/// match check_gitea("my-repo", "username", "https://gitea.example.com", "1.0.0") {
///     Ok(info) => println!("{}", info),
///     Err(e) => eprintln!("Error checking for updates: {}", e),
/// }
/// ```
pub fn check_gitea(
    name: &str,
    user: &str,
    gitea_url: &str,
    current_version: &str,
) -> anyhow::Result<UpdateInfo> {
    let update_available = UpdateAvailable::new(name, current_version);
    update_available.gitea(user, gitea_url)
}
