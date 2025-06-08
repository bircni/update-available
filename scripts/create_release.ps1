# Enable push with follow tags
git config push.followTags true

# if current branch is not main, exit
$current_branch = git rev-parse --abbrev-ref HEAD
if ($current_branch -ne 'main') {
    Write-Host "You are currently on branch '$current_branch'. Please switch to the 'main' branch to create a release."
    exit 1
}

# Check if git cliff is installed
if (-not (Get-Command git-cliff -ErrorAction SilentlyContinue)) {
    Write-Host "git cliff is not installed. Installing it now..."
    # Install git cliff using Winget
    winget install -e --id orhun.git-cliff -s winget
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to install git cliff. Please install it manually."
        exit 1
    }
    Write-Host "git cliff installed successfully."
}
# Check if cargo-verset is installed
if (-not (Get-Command cargo-verset -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-verset is not installed. Please install it with cargo install cargo-verset"
    exit 1
}

$version = git cliff --bumped-version
$current_version = git describe --tags
# Check if the version is already the same as the current version
if ($version -eq $current_version) {
    Write-Host "Version is already set to $version. No changes made."
    exit 0
}


Write-Host "Calculated version: $version"
Write-Host "Updating version in Cargo.toml..."
# Update the version in Cargo.toml
cargo verset -v $version
Write-Host "Version updated successfully in Cargo.toml."
# Generate the changelog
Write-Host "Generating changelog..."
git cliff --output CHANGELOG.md -t $version
Write-Host "Changelog generated successfully."
# Ask for confirmation before committing
Write-Host "Please review the changes in Cargo.toml and CHANGELOG.md."
$confirmation = Read-Host "Do you want to commit the changes? (y/n)"
if ($confirmation -ne 'y') {
    Write-Host "Changes not committed. Exiting."
    exit 0
}
# Build the project to update Cargo.lock
Write-Host "Building the project to update Cargo.lock..."
cargo build --release --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Please fix the errors and try again."
    exit 1
}
# Commit the changes
git add Cargo.toml CHANGELOG.md
git commit -m "release($version)"
git tag -a $version -m "Release $version" 

Write-Host "Changes committed and tagged with version $version."
Write-Host "Please push the changes to the remote repository."

$push_confirmation = Read-Host "Do you want to push the changes now? (y/n)"
if ($push_confirmation -eq 'y') {
    git push origin main --follow-tags
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to push changes. Please check your remote repository settings."
        exit 1
    }
    Write-Host "Changes pushed successfully."
} else {
    Write-Host "Changes not pushed. You can push them later using 'git push origin main --follow-tags'."
}

Write-Host "Don't forget to release the new version on GitHub and on crates.io." -BackgroundColor Green