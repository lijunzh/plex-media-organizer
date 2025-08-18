# 📦 Installation Guide

Complete guide for installing Plex Media Organizer on any system.

## 🚀 Quick Install (Recommended)

### From Source (Most Up-to-Date)
```bash
# Prerequisites: Rust 1.70+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Binary location: ./target/release/plex-media-organizer
```

### From Cargo (Future)
```bash
# Coming soon
cargo install plex-media-organizer
```

## 📱 Platform-Specific Instructions

### 🍎 macOS

#### Option 1: Homebrew (Future)
```bash
# Coming soon
brew tap lijunzh/plex-media-organizer
brew install plex-media-organizer
```

#### Option 2: From Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Xcode Command Line Tools (if needed)
xcode-select --install

# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Optional: Install to PATH
sudo cp target/release/plex-media-organizer /usr/local/bin/
```

#### Option 3: Binary Download (Future)
```bash
# Coming soon - download pre-built binary
curl -L https://github.com/lijunzh/plex-media-organizer/releases/latest/download/plex-media-organizer-macos.tar.gz | tar xz
```

### 🐧 Linux

#### Option 1: From Source (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt update
sudo apt install curl build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Optional: Install to PATH
sudo cp target/release/plex-media-organizer /usr/local/bin/
```

#### Option 2: From Source (RHEL/CentOS/Fedora)
```bash
# Install dependencies
sudo dnf install curl gcc make  # Fedora
# OR
sudo yum install curl gcc make  # RHEL/CentOS

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release
```

#### Option 3: Package Managers (Future)
```bash
# Coming soon
sudo apt install plex-media-organizer  # Debian/Ubuntu
sudo dnf install plex-media-organizer  # Fedora
sudo pacman -S plex-media-organizer    # Arch
```

#### Option 4: AppImage (Future)
```bash
# Coming soon - portable binary
wget https://github.com/lijunzh/plex-media-organizer/releases/latest/download/plex-media-organizer.AppImage
chmod +x plex-media-organizer.AppImage
./plex-media-organizer.AppImage
```

### 🪟 Windows

#### Option 1: From Source
```powershell
# Install Rust
# Download and run: https://forge.rust-lang.org/infra/channel-layout.html#artifacts

# Install Git (if not already installed)
# Download from: https://git-scm.com/download/win

# Clone and build
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer
cargo build --release

# Binary location: .\target\release\plex-media-organizer.exe
```

#### Option 2: Binary Download (Future)
```powershell
# Coming soon - download pre-built binary
# Will be available at: https://github.com/lijunzh/plex-media-organizer/releases
```

#### Option 3: Package Managers (Future)
```powershell
# Coming soon
winget install plex-media-organizer
# OR
choco install plex-media-organizer
# OR
scoop install plex-media-organizer
```

## 🐳 Docker

### Quick Docker Run
```bash
# Build from source
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Build Docker image
docker build -t plex-media-organizer .

# Run with your movies directory mounted
docker run -it --rm \
  -v /path/to/your/movies:/movies \
  -e TMDB_API_KEY=your_key_here \
  plex-media-organizer scan /movies
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'
services:
  plex-organizer:
    build: .
    volumes:
      - /path/to/your/movies:/movies
      - ./config:/config
    environment:
      - TMDB_API_KEY=your_key_here
    command: scan /movies
```

### Pre-built Docker Image (Future)
```bash
# Coming soon
docker pull lijunzh/plex-media-organizer:latest
docker run -it --rm \
  -v /path/to/movies:/movies \
  -e TMDB_API_KEY=your_key \
  lijunzh/plex-media-organizer:latest scan /movies
```

## ☁️ Cloud/Remote Installation

### GitHub Codespaces
```bash
# Open in Codespaces
# Click "Code" → "Codespaces" → "Create codespace"

# Install in codespace
cargo build --release
```

### Gitpod
```bash
# Open in Gitpod
# Go to: https://gitpod.io/#https://github.com/lijunzh/plex-media-organizer

# Build automatically starts with .gitpod.yml
```

### Replit
```bash
# Fork the repository on Replit
# Build and run in browser
```

## 🛠️ Development Installation

For contributors and developers:

```bash
# Clone with all branches
git clone --recurse-submodules https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Install development dependencies
rustup component add rustfmt clippy
cargo install cargo-audit cargo-outdated

# Build in development mode
cargo build

# Run tests
cargo test

# Check code quality
cargo fmt --check
cargo clippy
```

## ✅ Verify Installation

After installation, verify everything works:

```bash
# Check version
plex-media-organizer --version

# Test with sample file
TMDB_API_KEY=your_key plex-media-organizer test "Movie.Name.2023.1080p.mkv"

# Expected output: ✅ Successfully parsed movie
```

## ⚙️ System Requirements

### Minimum Requirements
- **RAM**: 100MB (for small libraries)
- **Storage**: 50MB for binary + cache
- **CPU**: Any modern processor
- **OS**: macOS 10.15+, Linux (glibc 2.17+), Windows 10+

### Recommended Requirements
- **RAM**: 500MB+ (for large libraries with 10,000+ files)
- **Storage**: 500MB+ for cache and metadata
- **CPU**: Multi-core for parallel processing
- **Network**: Internet connection for TMDB API

### Performance Expectations
- **Small libraries** (< 1,000 files): Near-instant processing
- **Medium libraries** (1,000-10,000 files): 1-30 seconds
- **Large libraries** (10,000+ files): 1-5 minutes

## 🔧 Post-Installation Setup

After installation, you'll need to:

1. **Get TMDB API Key**: [Get free key](https://www.themoviedb.org/settings/api)
2. **Configure**: See [configuration guide](configuration.md)
3. **Test**: Run initial scan on small directory
4. **Learn**: Read [user guide](user-guide.md) for full features

## 🆘 Installation Troubleshooting

### Common Issues

#### Rust Not Found
```bash
# Make sure Rust is in PATH
source ~/.cargo/env
# OR add to your shell profile
echo 'source ~/.cargo/env' >> ~/.bashrc
```

#### Build Failures
```bash
# Update Rust
rustup update

# Clear cache
cargo clean
cargo build --release
```

#### Permission Errors (Linux/macOS)
```bash
# Don't use sudo with cargo
# If needed, fix ownership:
sudo chown -R $USER:$USER ~/.cargo
```

#### Windows Build Issues
- Install Visual Studio Build Tools
- Or install Visual Studio Community with C++ tools
- Make sure Windows SDK is installed

### Getting Help

- **Quick fixes**: [Troubleshooting guide](troubleshooting.md)
- **Build issues**: [GitHub issues](https://github.com/lijunzh/plex-media-organizer/issues)
- **General help**: [Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)

---

**Next Steps:** Once installed, see the [Quick Start Guide](quick-start.md) to begin organizing your media!
