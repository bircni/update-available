use update_available::{Source, get_check};

fn main() {
    // even if the version is not available, it will print a message
    update_available::print_check_force("serde", "1.0.0", Source::CratesIo { url: None });
    update_available::print_check_force("serde-rs", "1.0.0", Source::Github("serde-rs".to_owned()));
    update_available::print_check_force(
        "my-repo",
        "0.1.0",
        Source::Gitea {
            user: "username".to_owned(),
            base_url: "https://gitea.example.com".to_owned(),
            token: None,
        },
    );
    
    // this only prints if an update is available
    update_available::print_check("serde", "1.0.0", Source::CratesIo { url: None });
    update_available::print_check("serde-rs", "1.0.0", Source::Github("serde-rs".to_owned()));
    update_available::print_check(
        "my-repo",
        "0.1.0",
        Source::Gitea {
            user: "username".to_owned(),
            base_url: "https://gitea.example.com".to_owned(),
            token: Some("your-auth-token".to_owned()),
        },
    );

    // Example using the new get_check function
    println!("\n--- Using get_check function ---");
    match get_check("serde", "1.0.0", Source::CratesIo { url: None }) {
        Ok(Some(info)) => println!("Update available:\n{}", info),
        Ok(None) => println!("No update available for serde"),
        Err(e) => eprintln!("Error checking serde: {}", e),
    }

    // Example with custom crates.io URL (e.g., kellnr registry)
    match get_check("my-crate", "0.1.0", Source::CratesIo { 
        url: Some("https://my-kellnr-registry.com".to_owned()) 
    }) {
        Ok(Some(info)) => println!("Custom registry update:\n{}", info),
        Ok(None) => println!("No update available from custom registry"),
        Err(e) => eprintln!("Error checking custom registry: {}", e),
    }
}
