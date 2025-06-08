use anyhow::Result;
use update_available::UpdateAvailable;

fn main() -> Result<()> {
    let checker = UpdateAvailable::new("serde", "1.0.0");
    let info = checker.check_crates_io()?;
    println!("{}", info);
    Ok(())
}
