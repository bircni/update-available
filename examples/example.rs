#![expect(clippy::unwrap_used, reason = "This is an example code")]
use update_available::Source;

fn main() {
    let cratesio_infos = update_available::check_crates_io("serde", "1.0.0").unwrap();
    // do something with the infos
    println!(
        "Crates.io Update Available: {}",
        cratesio_infos.is_update_available
    );
    let github_infos = update_available::check_github("serde", "serde-rs", "1.0.0").unwrap();
    // do something with the github infos
    println!(
        "GitHub Update Available: {}",
        github_infos.is_update_available
    );
    // Print the update check results
    update_available::print_check("serde", "1.0.0", Source::CratesIo);
}
