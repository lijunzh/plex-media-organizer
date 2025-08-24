# Installation Guide

This guide covers all installation methods for Plex Media Organizer.

## Quick Installation

### Download Pre-built Binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/lijunzh/plex-media-organizer/releases):

#### macOS
- **Intel Macs**: `plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz`
- **Apple Silicon**: `plex-media-organizer-v1.0.0-aarch64-apple-darwin.tar.gz`

```bash
# Download and extract
curl -L -o plex-media-organizer.tar.gz "https://github.com/lijunzh/plex-media-organizer/releases/latest/download/plex-media-organizer-v1.0.0-x86_64-apple-darwin.tar.gz"
tar -xzf plex-media-organizer.tar.gz

# Install to system PATH
sudo mv plex-media-organizer /usr/local/bin/
chmod +x /usr/local/bin/plex-media-organizer

# Verify installation
plex-media-organizer --version
```

#### Linux
- **x86_64**: `plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`

```bash
# Download and extract
curl -L -o plex-media-organizer.tar.gz "https://github.com/lijunzh/plex-media-organizer/releases/latest/download/plex-media-organizer-v1.0.0-x86_64-unknown-linux-gnu.tar.gz"
tar -xzf plex-media-organizer.tar.gz

# Install to system PATH
sudo mv plex-media-organizer /usr/local/bin/
chmod +x /usr/local/bin/plex-media-organizer

# Verify installation
plex-media-organizer --version
```

#### Windows
- **x86_64**: `plex-media-organizer-v1.0.0-x86_64-pc-windows-msvc.zip`

1. Download the zip file from GitHub Releases
2. Extract to a directory (e.g., `C:\plex-media-organizer`)
3. Add the directory to your system PATH
4. Open Command Prompt and verify: `plex-media-organizer.exe --version`

## Build from Source

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation
- **Git**: For cloning the repository

### Building

```bash
# Clone the repository
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

### Development Build

```bash
# Development build with debug information
cargo build

# Run directly without installation
cargo run -- --help
```

## Package Manager Installation

### Homebrew (macOS)

```bash
# Add the tap (when available)
brew tap lijunzh/plex-media-organizer

# Install
brew install plex-media-organizer
```

### Cargo Install

```bash
# Install from crates.io (when published)
cargo install plex-media-organizer

# Install from GitHub
cargo install --git https://github.com/lijunzh/plex-media-organizer.git
```

## Docker Installation

### Using Docker

```bash
# Pull the image
docker pull ghcr.io/lijunzh/plex-media-organizer:latest

# Run with volume mounts
docker run -v /path/to/movies:/movies -v /path/to/output:/output ghcr.io/lijunzh/plex-media-organizer:latest scan /movies
```

### Docker Compose

```yaml
version: '3.8'
services:
  plex-organizer:
    image: ghcr.io/lijunzh/plex-media-organizer:latest
    volumes:
      - /path/to/movies:/movies
      - /path/to/output:/output
      - ./config:/root/.config/plex-media-organizer
    environment:
      - TMDB_API_KEY=your_api_key_here
```

## Nightly Builds

For the latest development features, you can use nightly builds:

1. Go to [GitHub Releases](https://github.com/lijunzh/plex-media-organizer/releases)
2. Look for releases tagged with `nightly-YYYYMMDD-commit`
3. Download the appropriate binary for your platform
4. Install using the same method as stable releases

**Note**: Nightly builds may contain experimental features and should be used for testing only.

## System Requirements

### Minimum Requirements

- **CPU**: 1 GHz dual-core processor
- **RAM**: 512 MB available memory
- **Storage**: 100 MB free space
- **OS**: macOS 10.15+, Linux (glibc 2.17+), Windows 10+

### Recommended Requirements

- **CPU**: 2 GHz quad-core processor
- **RAM**: 2 GB available memory
- **Storage**: 500 MB free space
- **Network**: Internet connection for TMDB API

### Dependencies

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libsqlite3-dev

# CentOS/RHEL/Fedora
sudo yum install sqlite-devel
# or
sudo dnf install sqlite-devel
```

#### macOS
```bash
# Using Homebrew
brew install sqlite3
```

#### Windows
- SQLite is bundled with the Windows release
- No additional dependencies required

## Verification

After installation, verify everything is working:

```bash
# Check version
plex-media-organizer --version

# Check help
plex-media-organizer --help

# Run setup
plex-media-organizer setup
```

## Troubleshooting

### Common Issues

#### Permission Denied
```bash
# Make executable
chmod +x plex-media-organizer

# Or install to system directory
sudo mv plex-media-organizer /usr/local/bin/
```

#### Command Not Found
```bash
# Check if in PATH
which plex-media-organizer

# Add to PATH if needed
export PATH="$PATH:/path/to/plex-media-organizer"
```

#### Library Not Found (Linux)
```bash
# Install required libraries
sudo apt-get install -y libsqlite3-dev

# Or update library cache
sudo ldconfig
```

#### macOS Security
If you get a security warning on macOS:

1. Go to System Preferences > Security & Privacy
2. Click "Allow Anyway" for the plex-media-organizer binary
3. Or run: `sudo xattr -rd com.apple.quarantine /usr/local/bin/plex-media-organizer`

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
- **Documentation**: [User Guide](.)

## Next Steps

After installation:

1. **Run setup**: `plex-media-organizer setup`
2. **Read the guide**: [CLI Commands](cli-commands.md)
3. **Configure**: [Configuration Guide](configuration.md)
4. **Start organizing**: `plex-media-organizer scan /path/to/movies`
