# Release Process

This document describes the automated release process for plex-media-organizer.

## Overview

The release process is fully automated through GitHub Actions. When you create a new tag starting with `v*` (e.g., `v0.1.2`), the following happens automatically:

1. **Multi-platform Build**: Builds binaries for Linux, macOS (Intel/Apple Silicon), and Windows
2. **GitHub Release**: Creates a new release with all binaries and documentation
3. **Crates.io Publication**: Publishes the package to crates.io
4. **Homebrew Update**: Updates the Homebrew formula with the new version

## Required Secrets

To enable automated publication, you need to configure the following secrets in your GitHub repository:

### 1. CARGO_REGISTRY_TOKEN

**Purpose**: Authentication token for publishing to crates.io

**How to get it**:
1. Go to [crates.io/settings/tokens](https://crates.io/settings/tokens)
2. Create a new token with "Publish" permissions
3. Copy the token

**How to set it**:
1. Go to your GitHub repository
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `CARGO_REGISTRY_TOKEN`
5. Value: Paste your crates.io token

### 2. GITHUB_TOKEN

**Purpose**: Automatically provided by GitHub Actions for repository access

**Note**: This is automatically available and doesn't need manual configuration.

## Release Workflow

### Trigger
- Push a tag starting with `v` (e.g., `v0.1.2`)

### Jobs

#### 1. Build Jobs
- `build-linux`: Builds Linux x86_64 binary
- `build-macos`: Builds macOS Intel and Apple Silicon binaries
- `build-windows`: Builds Windows x86_64 binary

#### 2. Create Release
- Downloads all build artifacts
- Creates GitHub release with binaries and documentation
- Generates release notes automatically

#### 3. Publish to Crates.io
- Verifies package integrity
- Publishes to crates.io using `CARGO_REGISTRY_TOKEN`
- Updates crates.io with new version

#### 4. Update Homebrew
- Calculates SHA256 checksums for new binaries
- Updates the Homebrew formula in the tap repository
- Commits and pushes changes to the tap

## Manual Release Process

If you need to create a release manually:

```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git add .
git commit -m "feat: prepare for v0.1.2"

# 3. Create and push tag
git tag v0.1.2
git push origin v0.1.2

# 4. Monitor the workflow
# Go to Actions tab in GitHub to watch the process
```

## Verification

After a release, verify that:

1. **GitHub Release**: Check the Releases page for the new version
2. **Crates.io**: Visit https://crates.io/crates/plex-media-organizer
3. **Homebrew**: Test installation with `brew install lijunzh/plex-media-organizer/plex-media-organizer`

## Troubleshooting

### Common Issues

#### Crates.io Publication Fails
- Check that `CARGO_REGISTRY_TOKEN` is set correctly
- Verify the token has publish permissions
- Ensure the version in `Cargo.toml` matches the tag

#### Homebrew Update Fails
- Check that the tap repository exists and is accessible
- Verify the binary URLs are correct
- Ensure SHA256 checksums are calculated properly

#### Build Failures
- Check the build logs for specific errors
- Verify all dependencies are available
- Ensure the code compiles locally before tagging

### Debugging

To debug release issues:

1. Check the Actions tab in GitHub for detailed logs
2. Look for specific error messages in failed jobs
3. Test the build process locally before creating a tag
4. Verify all secrets are configured correctly

## Security Considerations

- Keep your `CARGO_REGISTRY_TOKEN` secure and never commit it to the repository
- Regularly rotate your crates.io token
- Use the minimum required permissions for tokens
- Monitor release logs for any suspicious activity
