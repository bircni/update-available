use semver::Version;

use crate::data::UpdateInfo;

use super::*;

#[test]
fn display_update_available() {
    let latest_version = Version::parse("1.2.3").unwrap();
    let update = UpdateInfo {
        is_update_available: true,
        latest_version,
        changelog: Some("Added new features and fixed bugs.".into()),
        url: String::from("https://crates.io/crates/serde"),
    };
    println!("{}", update);
}

#[test]
fn display_no_update() {
    let latest_version = Version::parse("1.2.3").unwrap();
    let update = UpdateInfo {
        is_update_available: false,
        latest_version,
        changelog: None,
        url: String::new(),
    };
    println!("{}", update);
}

#[cfg(feature = "blocking")]
#[test]
fn test_crates_io_check() {
    let update = UpdateAvailable::new("cargo-wash", "0.1.0");
    let result = update.check_crates_io();
    assert!(
        result.is_ok(),
        "Failed to check crates.io: {:?}",
        result.err()
    );
    let update_info = result.unwrap();
    assert!(
        update_info.is_update_available,
        "Expected an update to be available"
    );
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_crates_io_check_async() {
    let update = UpdateAvailable::new("cargo-wash", "0.1.0");
    let result = update.check_crates_io_async().await;
    assert!(
        result.is_ok(),
        "Failed to check crates.io: {:?}",
        result.err()
    );
    let update_info = result.unwrap();
    assert!(
        update_info.is_update_available,
        "Expected an update to be available"
    );
}

#[test]
fn test_github_check() {
    let update = UpdateAvailable::new("cargo-wash", "0.1.0");
    let result = update.check_github("bircni");
    assert!(result.is_ok(), "Failed to check GitHub: {:?}", result.err());
    let update_info = result.unwrap();
    assert!(
        update_info.is_update_available,
        "Expected an update to be available"
    );
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_github_check_async() {
    let update = UpdateAvailable::new("cargo-wash", "0.1.0");
    let result = update.check_github_async("bircni").await;
    assert!(result.is_ok(), "Failed to check GitHub: {:?}", result.err());
    let update_info = result.unwrap();
    assert!(
        update_info.is_update_available,
        "Expected an update to be available"
    );
}
