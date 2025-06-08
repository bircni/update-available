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

    /// Checks for updates on crates.io for the specified package.
    ///
    /// This method queries the crates.io API to check if a newer version
    /// of the specified package is available.
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
    /// * The crates.io API returns an error
    /// * The version strings cannot be parsed
    /// * The response format is unexpected
    #[cfg(feature = "blocking")]
    pub(crate) fn crates_io(&self) -> anyhow::Result<UpdateInfo> {
        let url = format!("https://crates.io/api/v1/crates/{}", self.name);
        let mut response = ureq::get(&url)
            .header("User-Agent", "update-available-lib")
            .call()?;

        if response.status().is_success() {
            let json: CratesResponse = response.body_mut().read_json()?;
            let info = UpdateInfo::from_crates(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from crates.io: {}", response.status());
            anyhow::bail!("Failed to fetch data from crates.io: {}", response.status());
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

    /// Checks for updates on Gitea for the specified repository.
    ///
    /// This method queries the Gitea API to check if a newer version
    /// of the specified repository is available by looking at the latest release.
    ///
    /// # Arguments
    ///
    /// * `user` - The Gitea username or organization that owns the repository
    /// * `gitea_url` - The base URL of the Gitea instance (e.g., <https://gitea.example.com>)
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
    #[cfg(feature = "blocking")]
    pub(crate) fn gitea(&self, user: &str, gitea_url: &str) -> anyhow::Result<UpdateInfo> {
        let url = format!(
            "{gitea_url}/api/v1/repos/{user}/{}/releases/latest",
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
            println!("Failed to fetch data from Gitea: {}", response.status());
            anyhow::bail!("Failed to fetch data from Gitea: {}", response.status());
        }
    }
}
