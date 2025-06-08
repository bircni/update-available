use crate::{
    UpdateAvailable,
    data::{CratesResponse, GiteaHubResponse, UpdateInfo},
};

impl UpdateAvailable {
    #[must_use]
    pub fn new(name: &str, current_version: &str) -> Self {
        Self {
            name: name.to_owned(),
            current_version: current_version.to_owned(),
        }
    }

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
