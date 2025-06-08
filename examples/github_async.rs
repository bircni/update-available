use anyhow::Result;
use update_available::UpdateAvailable;

#[tokio::main]
async fn main() -> Result<()> {
    let checker = UpdateAvailable::new("serde", "1.0.0");
    let info = checker.check_github("serde-rs")?;
    println!("{}", info);
    Ok(())
}
