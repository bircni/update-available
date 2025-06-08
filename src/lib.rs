#[cfg(feature = "async")]
use reqwest::Client;

use crate::data::{CratesResponse, GithubResponse, UpdateInfo};

mod data;
#[cfg(test)]
mod test;

#[derive(Default)]
pub struct UpdateAvailable {
    name: String,
    current_version: String,
}

impl UpdateAvailable {
    pub fn new(name: &str, current_version: &str) -> Self {
        Self {
            name: name.to_string(),
            current_version: current_version.to_string(),
        }
    }
    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn version(&mut self, version: &str) {
        self.current_version = version.to_string();
    }

    #[cfg(feature = "blocking")]
    pub fn check_crates_io(&self) -> anyhow::Result<UpdateInfo> {
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

    #[cfg(feature = "async")]
    pub async fn check_crates_io_async(&self) -> anyhow::Result<UpdateInfo> {
        let url = format!("https://crates.io/api/v1/crates/{}", self.name);
        let client = Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "update-available-lib")
            .send()
            .await?;

        if response.status().is_success() {
            let json: CratesResponse = response.json().await?;
            let info = UpdateInfo::from_crates(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from crates.io: {}", response.status());
            anyhow::bail!("Failed to fetch data from crates.io: {}", response.status());
        }
    }

    #[cfg(feature = "blocking")]
    pub fn check_github(&self, user: &str) -> anyhow::Result<UpdateInfo> {
        let url = format!(
            "https://api.github.com/repos/{user}/{}/releases/latest",
            self.name
        );
        let mut response = ureq::get(url)
            .header("User-Agent", "update-available-lib")
            .call()?;

        if response.status().is_success() {
            let json: GithubResponse = response.body_mut().read_json()?;
            let info = UpdateInfo::from_github(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from GitHub: {}", response.status());
            anyhow::bail!("Failed to fetch data from GitHub: {}", response.status());
        }
    }

    #[cfg(feature = "async")]
    pub async fn check_github_async(&self, user: &str) -> anyhow::Result<UpdateInfo> {
        let url = format!(
            "https://api.github.com/repos/{user}/{}/releases/latest",
            self.name
        );
        // Erwartet eine GitHub-API-URL wie "https://api.github.com/repos/OWNER/REPO/releases/latest"
        let client = Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "update-available-lib")
            .send()
            .await?;

        if response.status().is_success() {
            let json: GithubResponse = response.json().await?;
            let info = UpdateInfo::from_github(json, &self.current_version)?;
            Ok(info)
        } else {
            println!("Failed to fetch data from GitHub: {}", response.status());
            anyhow::bail!("Failed to fetch data from GitHub: {}", response.status());
        }
    }
}
