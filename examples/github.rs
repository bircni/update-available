use anyhow::Result;
use update_available::UpdateAvailable;

fn main() -> Result<()> {
    let checker = UpdateAvailable::new("cargo-wash", "1.0.0");
    let info = checker.check_github("bircni")?;
    println!("{}", info);
    Ok(())
}
