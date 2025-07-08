use update_available::Source;

fn main() {
    // even if the version is not available, it will print a message
    update_available::print_check_force("serde", "1.0.0", Source::CratesIo);
    update_available::print_check_force("serde-rs", "1.0.0", Source::Github("serde-rs".to_owned()));
    update_available::print_check_force(
        "my-repo",
        "0.1.0",
        Source::Gitea(
            "username".to_owned(),
            "https://gitea.example.com".to_owned(),
        ),
    );
    // this only prints if an update is available
    update_available::print_check("serde", "1.0.0", Source::CratesIo);
    update_available::print_check("serde-rs", "1.0.0", Source::Github("serde-rs".to_owned()));
    update_available::print_check(
        "my-repo",
        "0.1.0",
        Source::Gitea(
            "username".to_owned(),
            "https://gitea.example.com".to_owned(),
        ),
    );
}
