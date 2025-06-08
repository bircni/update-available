#![expect(clippy::redundant_pub_crate, reason = "This is a library module")]
use core::fmt;

use semver::Version;
use serde::Deserialize;

/// Internal structure for managing update checks.
#[derive(Default)]
pub(crate) struct UpdateAvailable {
    pub(crate) name: String,
    pub(crate) current_version: String,
}

/// Response structure for GitHub/Gitea API calls.
#[derive(Deserialize)]
pub(crate) struct GiteaHubResponse {
    pub(crate) tag_name: String,
    pub(crate) body: Option<String>,
    pub(crate) html_url: String,
}

/// Response structure for crates.io API calls.
#[derive(Deserialize)]
pub(crate) struct CratesResponse {
    #[serde(rename = "crate")]
    pub(crate) info: CrateInfo,
}

/// Crate information from crates.io.
#[derive(Deserialize)]
pub(crate) struct CrateInfo {
    pub(crate) max_version: Version,
    pub(crate) name: String,
}

/// Contains information about available updates for a package.
///
/// This structure provides all the necessary information about whether
/// an update is available, including version details, changelog, and
/// where to find more information.
pub struct UpdateInfo {
    /// Whether a newer version is available than the current one.
    pub is_update_available: bool,
    /// The latest available version.
    pub latest_version: Version,
    /// Optional changelog or release notes for the latest version.
    pub changelog: Option<String>,
    /// URL where more information can be found (crates.io, GitHub, etc.).
    pub url: String,
}

impl UpdateInfo {
    /// Creates a new `UpdateInfo` instance.
    ///
    /// Compares the latest version with the current version to determine
    /// if an update is available.
    ///
    /// # Arguments
    ///
    /// * `latest_version` - The latest available version
    /// * `current_version` - The currently installed version
    /// * `changelog` - Optional changelog or release notes
    /// * `url` - URL for more information about the package
    pub(crate) fn new(
        latest_version: Version,
        current_version: &Version,
        changelog: Option<String>,
        url: String,
    ) -> Self {
        let is_update_available = (
            latest_version.major,
            latest_version.minor,
            latest_version.patch,
        ) > (
            current_version.major,
            current_version.minor,
            current_version.patch,
        );

        Self {
            is_update_available,
            latest_version,
            changelog,
            url,
        }
    }

    /// Creates an `UpdateInfo` from a crates.io API response.
    ///
    /// # Arguments
    ///
    /// * `crates_response` - The response from the crates.io API
    /// * `current_version` - The current version string to compare against
    ///
    /// # Errors
    ///
    /// Returns an error if the current version string cannot be parsed.
    pub(crate) fn from_crates(
        crates_response: CratesResponse,
        current_version: &str,
    ) -> anyhow::Result<Self> {
        let latest_version = crates_response.info.max_version;
        let current_version = Version::parse(current_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse current version: {}", e))?;
        let url = format!("https://crates.io/crates/{}", crates_response.info.name);
        Ok(Self::new(latest_version, &current_version, None, url))
    }

    /// Creates an `UpdateInfo` from a GitHub or Gitea API response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from the GitHub or Gitea API
    /// * `current_version` - The current version string to compare against
    ///
    /// # Errors
    ///
    /// Returns an error if the version strings cannot be parsed.
    pub(crate) fn from_gitea_or_hub(
        response: GiteaHubResponse,
        current_version: &str,
    ) -> anyhow::Result<Self> {
        let latest_version = response
            .tag_name
            .strip_prefix("v")
            .unwrap_or(&response.tag_name);
        let latest_version = Version::parse(latest_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse latest version: {}", e))?;
        let current_version = Version::parse(current_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse current version: {}", e))?;
        Ok(Self::new(
            latest_version,
            &current_version,
            response.body,
            response.html_url,
        ))
    }

    /// Prints the update information if an update is available.
    ///
    /// This is a convenience method that only prints output when
    /// `is_update_available` is true.
    pub fn print(&self) {
        if self.is_update_available {
            println!("{self}");
        }
    }
}

impl fmt::Display for UpdateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_update_available {
            writeln!(f, "🚀  A new version is available!")?;
            writeln!(f, "🔖  Latest version: {}", self.latest_version)?;
            if let Some(changelog) = &self.changelog {
                writeln!(f, "📝  Changelog:")?;
                for line in changelog.lines().take(4) {
                    if line.trim().is_empty() || line.starts_with("## ") {
                        continue;
                    }
                    if line.starts_with('-') || line.starts_with('*') {
                        writeln!(f, "    {line}")?;
                    } else if line.starts_with("•") {
                        writeln!(f, "    {}", line.trim_start_matches('•'))?;
                    } else {
                        writeln!(f, "    • {line}")?;
                    }
                }
                if changelog
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.starts_with("## "))
                    .count()
                    > 4
                {
                    writeln!(f, "    • (and more...)")?;
                }
            }
            writeln!(f, "🌐  More info: {}", self.url)?;
        }
        Ok(())
    }
}
