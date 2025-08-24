# GitHub Actions Workflows

This directory contains the streamlined CI/CD workflows for the Plex Media Organizer project.

## Workflow Overview

### 🔄 `ci.yml` - Main CI/CD Pipeline
**Triggers:** PRs and pushes to main branch
**Purpose:** Core testing and validation
- ✅ Code formatting and linting
- ✅ Unit and integration tests
- ✅ Build verification
- ✅ CLI functionality tests
- ✅ Documentation build
- ✅ Security audit
- ✅ Performance tests (PRs only)
- ✅ PR status comments

### 🏷️ `release.yml` - Production Releases
**Triggers:** Tagged releases (v*)
**Purpose:** Create production releases
- 🔨 Multi-platform builds (Linux, macOS, Windows)
- 📦 Optimized binaries with LTO and strip
- 🚀 GitHub releases with assets
- 📚 Documentation included

### 🌙 `nightly.yml` - Nightly Builds
**Triggers:** Daily at 2 AM UTC + manual
**Purpose:** Latest development builds
- 🔨 Multi-platform builds
- 📦 Optimized binaries
- 🔄 Single release replacement (keeps history clean)
- 🏷️ Date+commit versioning (e.g., `nightly-20241201-a1b2c3d`)

### 📋 `quality.yml` - Quality Assurance
**Triggers:** PRs + weekly schedule + manual
**Purpose:** Code quality and analysis
- 🔍 PR analysis and recommendations
- 📚 Documentation checks
- 📦 Dependency analysis
- 🧪 Weekly comprehensive testing

## Workflow Benefits

### ✅ **Streamlined Structure**
- **Reduced redundancy**: Eliminated duplicate testing
- **Clear separation**: Each workflow has a specific purpose
- **Efficient resource usage**: Optimized caching and parallel execution

### ✅ **Improved Quality**
- **Comprehensive testing**: Full test suite on every PR
- **Performance validation**: Regular performance checks
- **Security scanning**: Automated security audits
- **Documentation validation**: Ensures docs stay current

### ✅ **Better User Experience**
- **Fast feedback**: Quick CI runs for developers
- **Clean releases**: Organized release history
- **Nightly access**: Latest builds for testing
- **Quality insights**: PR analysis and recommendations

## Usage

### For Developers
1. **PRs**: Automatically trigger CI and quality checks
2. **Main branch**: Full validation on every push
3. **Manual testing**: Use nightly builds for latest features

### For Maintainers
1. **Releases**: Tag with `v*` to trigger production builds
2. **Quality monitoring**: Weekly comprehensive tests
3. **Nightly management**: Single release keeps history clean

### For Users
1. **Stable releases**: Use tagged releases for production
2. **Latest features**: Use nightly builds for testing
3. **Multi-platform**: All platforms supported

## Configuration

### Required Secrets
- `TMDB_API_KEY`: For integration tests
- `GITHUB_TOKEN`: Automatically provided

### Optional Optimizations
- **Caching**: Dependencies cached for faster builds
- **Parallel execution**: Jobs run in parallel where possible
- **Resource optimization**: Minimal toolchain profiles

## Maintenance

### Adding New Tests
1. Add to `ci.yml` for PR validation
2. Add to `quality.yml` for comprehensive testing
3. Update documentation as needed

### Modifying Builds
1. Update `release.yml` for production changes
2. Update `nightly.yml` for development builds
3. Test changes in PRs first

### Workflow Updates
1. Test changes in a branch first
2. Update this README when workflows change
3. Monitor workflow performance and adjust as needed
