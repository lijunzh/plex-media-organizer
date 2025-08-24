# Developer Documentation

Welcome to the Plex Media Organizer developer documentation. This guide provides everything you need to contribute to the project effectively.

## 🚀 Getting Started

### New Contributors
Start here if you're new to the project:

1. **[Contributing Guide](../../CONTRIBUTING.md)** - How to contribute to the project
2. **[Git Practices](git-practices.md)** - Commit standards and workflow
3. **[Documentation Standards](documentation-standards.md)** - Documentation guidelines

### Development Setup
```bash
# Clone the repository
git clone https://github.com/lijunzh/plex-media-organizer.git
cd plex-media-organizer

# Install dependencies
cargo build

# Run tests
cargo test

# Run with watch mode
cargo watch -x test
```

## 📚 Documentation Structure

### Development Guidelines
- **[Git Practices](git-practices.md)** - Commit standards and workflow
- **[Documentation Standards](documentation-standards.md)** - Documentation guidelines
- **[Documentation Status](DOCUMENTATION_STATUS.md)** - Documentation completion status
- **[Release Process](release-process.md)** - Automated release and publication workflow

### Architecture & Design
- **[Architecture](architecture/)** - System design and technical architecture
  - **[Current Architecture](architecture/architecture.md)** - Existing system design
  - **[Module Structure](architecture/module-structure.md)** - Target architecture
  - **[Roadmap](architecture/roadmap.md)** - Development phases and planning
  - **[Refactoring Strategy](architecture/refactoring-strategy.md)** - Code refactoring approach
  - **[Codebase Analysis](architecture/codebase-analysis.md)** - Technical analysis
  - **[Cross Reference](architecture/refactoring-cross-reference.md)** - Code references

### Planning & Roadmaps
- **[Planning](planning/)** - Development planning documents
  - **[Implementation Roadmap](planning/implementation-roadmap.md)** - Development roadmap

### Project History
- **[Retrospectives](retrospectives/)** - Project retrospectives and lessons learned
  - **[Phase 1.1 Completion](retrospectives/phase-1-1-completion-report.md)** - Phase completion report

## 🏗️ Architecture Overview

### Current System
The Plex Media Organizer is built with a modular architecture:

```
src/
├── cli/           # Command-line interface
├── config/        # Configuration management
├── database/      # Database operations
├── external/      # External API integrations
├── parsers/       # Media parsing logic
├── scanner/       # File scanning
└── types/         # Common types and structures
```

### Key Components
- **CLI Module**: Command-line interface and user interaction
- **Config Module**: Configuration management and validation
- **Database Module**: SQLite database operations and caching
- **External Module**: TMDB API integration
- **Parsers Module**: Intelligent filename parsing
- **Scanner Module**: File system scanning and organization
- **Types Module**: Common data structures and types

## 🧪 Testing & Quality

### Test Structure
- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end workflow testing
- **Performance Tests**: Performance validation
- **Documentation Tests**: Code documentation validation

### Quality Standards
- **Code Coverage**: Comprehensive test coverage
- **Documentation**: All public APIs documented
- **Performance**: Optimized for large media libraries
- **Safety**: Comprehensive error handling and validation

## 📋 Development Workflow

### Git Workflow
1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Document** your changes
6. **Submit** a pull request

### Commit Standards
- Use [Conventional Commits](https://www.conventionalcommits.org/)
- Include descriptive commit messages
- Reference issues when applicable
- Keep commits focused and atomic

### Code Review Process
1. **Automated Checks**: CI/CD pipeline validation
2. **Code Review**: Maintainer review and feedback
3. **Testing**: Comprehensive test validation
4. **Documentation**: Documentation updates
5. **Merge**: Approved changes merged to main

## 🔧 Development Tools

### Required Tools
- **Rust 1.70+**: Programming language
- **Cargo**: Package manager and build tool
- **Git**: Version control
- **SQLite**: Database (bundled)

### Recommended Tools
- **cargo-watch**: Development with auto-reload
- **cargo-tarpaulin**: Code coverage analysis
- **cargo-audit**: Security vulnerability scanning
- **rustfmt**: Code formatting
- **clippy**: Linting and code quality

### Development Commands
```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Code formatting
cargo fmt

# Linting
cargo clippy

# Security audit
cargo audit

# Documentation
cargo doc --open
```

## 📊 Project Metrics

### Current Status
- **95+ Unit Tests**: Comprehensive test coverage
- **25+ Integration Tests**: End-to-end validation
- **6 Documentation Tests**: Code documentation
- **Performance**: 390+ files/second parsing speed
- **Coverage**: 100% pattern coverage for real-world conventions

### Quality Metrics
- **Code Quality**: Clippy warnings addressed
- **Documentation**: All public APIs documented
- **Performance**: Optimized for large libraries
- **Safety**: Comprehensive error handling

## 🎯 Contribution Areas

### High Priority
- **TV Show Support**: Episode detection and organization
- **Performance Optimization**: Large library handling
- **Documentation**: User and developer guides
- **Testing**: Additional test coverage

### Medium Priority
- **Web Interface**: Browser-based management
- **Advanced Features**: Scheduled operations
- **Music Support**: Music file organization
- **Plugin System**: Extensible architecture

### Low Priority
- **Mobile App**: Mobile application
- **Cloud Integration**: Cloud storage support
- **Advanced Analytics**: Usage analytics
- **API Development**: REST API interface

## 🐛 Issue Management

### Issue Types
- **Bug Reports**: Functionality issues
- **Feature Requests**: New functionality
- **Documentation**: Documentation improvements
- **Performance**: Performance issues
- **Security**: Security vulnerabilities

### Issue Guidelines
- **Reproducible**: Include steps to reproduce
- **Specific**: Provide detailed information
- **Tested**: Verify on latest version
- **Documented**: Include relevant logs

## 📞 Getting Help

### Development Questions
- **GitHub Discussions**: Community discussions
- **GitHub Issues**: Bug reports and feature requests
- **Documentation**: This developer guide
- **Code Comments**: Inline code documentation

### Contributing
- **Contributing Guide**: [CONTRIBUTING.md](../../CONTRIBUTING.md)
- **Code of Conduct**: Project behavior standards
- **Development Setup**: This guide
- **Architecture**: [Architecture Documentation](architecture/)

---

**Ready to contribute?** Start with the [Contributing Guide](../../CONTRIBUTING.md) and [Git Practices](git-practices.md).
