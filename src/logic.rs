use crate::{
    UpdateAvailable,
    data::{CratesResponse, GiteaHubResponse, UpdateInfo},
};

impl UpdateAvailable {
    /// Creates a new `UpdateAvailable` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the package/repository to check for updates
    /// * `current_version` - The current version string (e.g., "1.0.0")
    #[must_use]
    pub fn new(name: &str, current_version: &str) -> Self {
        Self {
            name: name.to_owned(),
            current_version: current_version.to_owned(),
        }
    }

    /// Checks for updates on crates.io or a custom registry for the specified package.
    ///
    /// This method queries the specified registry API to check if a newer version
    /// of the specified package is available.
    ///
    /// # Arguments
    ///
    /// * `custom_url` - Optional custom registry URL. If None, uses crates.io
    ///
    /// # Returns
    ///
    /// Returns a `Result<UpdateInfo, anyhow::Error>` containing update information
    /// if successful, or an error if the check fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The network request fails
    /// * The registry API returns an error
    /// * The version strings cannot be parsed
    /// * The response format is unexpected
    #[cfg(feature = "blocking")]
    pub(crate) fn crates_io_with_url(&self, custom_url: Option<&str>) -> anyhow::Result<UpdateInfo> {
        let base_url = custom_url.unwrap_or("https://crates.io");
        let url = format!("{}/api/v1/crates/{}", base_url, self.name);
        let mut response = ureq::get(&url)
            .header("User-Agent", "update-available-lib")
            .call()?;

        if response.status().is_success() {
            let json: CratesResponse = response.body_mut().read_json()?;
            let info = UpdateInfo::from_crates(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from registry: {}", response.status());
            anyhow::bail!("Failed to fetch data from registry: {}", response.status());
        }
    }

    /// Checks for updates on GitHub for the specified repository.
    ///
    /// This method queries the GitHub API to check if a newer version
    /// of the specified repository is available by looking at the latest release.
    ///
    /// # Arguments
    ///
    /// * `user` - The GitHub username or organization that owns the repository
    ///
    /// # Returns
    ///
    /// Returns a `Result<UpdateInfo, anyhow::Error>` containing update information
    /// if successful, or an error if the check fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The network request fails
    /// * The GitHub API returns an error
    /// * The version strings cannot be parsed
    /// * The response format is unexpected
    /// * The repository does not exist or has no releases
    #[cfg(feature = "blocking")]
    pub(crate) fn github(&self, user: &str) -> anyhow::Result<UpdateInfo> {
        let url = format!(
            "https://api.github.com/repos/{user}/{}/releases/latest",
            self.name
        );
        let mut response = ureq::get(url)
            .header("User-Agent", "update-available-lib")
            .call()?;

        if response.status().is_success() {
            let json: GiteaHubResponse = response.body_mut().read_json()?;
            let info = UpdateInfo::from_gitea_or_hub(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from GitHub: {}", response.status());
            anyhow::bail!("Failed to fetch data from GitHub: {}", response.status());
        }
    }

    /// Checks for updates on Gitea for the specified repository with optional token authentication.
    ///
    /// This method queries the Gitea API to check if a newer version
    /// of the specified repository is available by looking at the latest release.
    /// An optional authentication token can be provided for private repositories
    /// or higher rate limits.
    ///
    /// # Arguments
    ///
    /// * `user` - The Gitea username or organization that owns the repository
    /// * `gitea_url` - The base URL of the Gitea instance (e.g., <https://gitea.example.com>)
    /// * `token` - Optional authentication token
    ///
    /// # Returns
    ///
    /// Returns a `Result<UpdateInfo, anyhow::Error>` containing update information
    /// if successful, or an error if the check fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// * The network request fails
    /// * The Gitea API returns an error
    /// * The version strings cannot be parsed
    /// * The response format is unexpected
    /// * The repository does not exist or has no releases
    /// * The Gitea URL is invalid
    /// * Authentication fails if token is invalid
    #[cfg(feature = "blocking")]
    pub(crate) fn gitea_with_token(&self, user: &str, gitea_url: &str, token: Option<&str>) -> anyhow::Result<UpdateInfo> {
        let url = format!(
            "{gitea_url}/api/v1/repos/{user}/{}/releases/latest",
            self.name
        );
        let mut request = ureq::get(&url)
            .header("User-Agent", "update-available-lib");
        
        if let Some(token) = token {
            request = request.header("Authorization", &format!("token {}", token));
        }
        
        let mut response = request.call()?;

        if response.status().is_success() {
            let json: GiteaHubResponse = response.body_mut().read_json()?;
            let info = UpdateInfo::from_gitea_or_hub(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from Gitea: {}", response.status());
            anyhow::bail!("Failed to fetch data from Gitea: {}", response.status());
        }
    }
}
