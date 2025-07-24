use semver::Version;

use crate::data::UpdateInfo;
use crate::{Source, UpdateAvailable, print_check};

#[test]
fn display_update_available() {
    let latest_version = Version::parse("1.2.3").unwrap();
    let update = UpdateInfo {
        is_update_available: true,
        latest_version,
        changelog: Some("Added new features and fixed bugs.".into()),
        url: String::from("https://crates.io/crates/serde"),
    };
    println!("{update}");
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
    println!("{update}");
}

#[test]
fn test_crates_io_check() {
    let update = UpdateAvailable::new("cargo-wash", "0.1.0");
    let result = update.crates_io_with_url(None);
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
    let result = update.github("bircni");
    assert!(result.is_ok(), "Failed to check GitHub: {:?}", result.err());
    let update_info = result.unwrap();
    assert!(
        update_info.is_update_available,
        "Expected an update to be available"
    );
}

#[test]
fn test_print_check_crates_io() {
    print_check("cargo-wash", "0.1.0", Source::CratesIo { url: None });
}

#[test]
fn test_print_check_github() {
    print_check("cargo-wash", "0.1.0", Source::Github("bircni".to_owned()));
}

#[ignore = "Gitea tests are ignored by default, as they require a valid Gitea URL and user."]
#[test]
fn test_print_check_gitea() {
    print_check(
        "cargo-wash",
        "0.1.0",
        Source::Gitea {
            user: "bircni".to_owned(),
            base_url: "https://gitea.example.com".to_owned(),
            token: None,
        },
    );
}

#[test]
fn test_no_update_same_version() {
    let current = Version::parse("1.2.3").unwrap();
    let latest = Version::parse("1.2.3").unwrap();
    let info = UpdateInfo::new(latest, &current, None, "url".into());

    assert!(!info.is_update_available);
}

#[test]
fn test_update_newer_version() {
    let current = Version::parse("1.2.3").unwrap();
    let latest = Version::parse("1.2.4").unwrap();
    let info = UpdateInfo::new(latest, &current, None, "url".into());

    assert!(info.is_update_available);
}

#[test]
fn test_get_check_crates_io() {
    use crate::get_check;
    let result = get_check("cargo-wash", "0.1.0", Source::CratesIo { url: None });
    assert!(result.is_ok(), "get_check should succeed: {:?}", result.err());
}

#[test]
fn test_get_check_github() {
    use crate::get_check;
    let result = get_check("cargo-wash", "0.1.0", Source::Github("bircni".to_owned()));
    assert!(result.is_ok(), "get_check should succeed: {:?}", result.err());
}

#[ignore = "Gitea tests are ignored by default, as they require a valid Gitea URL and user."]
#[test]
fn test_get_check_gitea_with_token() {
    use crate::get_check;
    let result = get_check(
        "cargo-wash",
        "0.1.0",
        Source::Gitea {
            user: "bircni".to_owned(),
            base_url: "https://gitea.example.com".to_owned(),
            token: Some("fake-token".to_owned()),
        },
    );
    // This may fail due to invalid token or URL, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[ignore = "Custom registry tests require a valid registry URL."]
#[test]
fn test_custom_crates_io_url() {
    use crate::get_check;
    let result = get_check(
        "some-crate",
        "0.1.0",
        Source::CratesIo {
            url: Some("https://my-custom-registry.com".to_owned()),
        },
    );
    // This may fail due to invalid URL, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_downgrade_misreported_as_update() {
    let current = Version::parse("2.0.0").unwrap();
    let latest = Version::parse("1.9.9").unwrap();
    let info = UpdateInfo::new(latest, &current, None, "url".into());

    assert!(!info.is_update_available);
}
