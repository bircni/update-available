use core::fmt;

use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct GithubResponse {
    pub(crate) tag_name: String,
    pub(crate) body: Option<String>,
    pub(crate) html_url: String,
}

#[derive(Deserialize)]
pub(crate) struct CratesResponse {
    #[serde(rename = "crate")]
    pub(crate) info: CrateInfo,
}

#[derive(Deserialize)]
pub(crate) struct CrateInfo {
    pub(crate) max_version: Version,
    pub(crate) name: String,
}

pub struct UpdateInfo {
    pub(crate) is_update_available: bool,
    pub(crate) latest_version: Version,
    pub(crate) changelog: Option<String>,
    pub(crate) url: String,
}

impl UpdateInfo {
    pub(crate) fn new(
        latest_version: Version,
        current_version: Version,
        changelog: Option<String>,
        url: String,
    ) -> Self {
        let is_update_available = latest_version > current_version;
        Self {
            is_update_available,
            latest_version,
            changelog,
            url,
        }
    }

    pub(crate) fn from_crates(
        crates_response: CratesResponse,
        current_version: &str,
    ) -> anyhow::Result<Self> {
        let latest_version = crates_response.info.max_version;
        let current_version = Version::parse(current_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse current version: {}", e))?;
        let url = format!("https://crates.io/crates/{}", crates_response.info.name);
        Ok(Self::new(latest_version, current_version, None, url))
    }

    pub(crate) fn from_github(
        github_response: GithubResponse,
        current_version: &str,
    ) -> anyhow::Result<Self> {
        let latest_version = github_response
            .tag_name
            .strip_prefix("v")
            .unwrap_or(&github_response.tag_name);
        let latest_version = Version::parse(latest_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse latest version: {}", e))?;
        let current_version = Version::parse(current_version)
            .map_err(|e| anyhow::anyhow!("Failed to parse current version: {}", e))?;
        Ok(Self::new(
            latest_version,
            current_version,
            github_response.body,
            github_response.html_url,
        ))
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
                        writeln!(f, "    {}", line)?;
                    } else if line.starts_with("•") {
                        writeln!(f, "    {}", line.trim_start_matches('•'))?;
                    } else {
                        writeln!(f, "    • {}", line)?;
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
