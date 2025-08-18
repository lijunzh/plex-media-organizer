# Contributing to Plex Media Organizer

Thank you for your interest in contributing to Plex Media Organizer! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- A TMDB API key for testing ([Get API Key](https://www.themoviedb.org/settings/api))

### Development Setup
```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/plex-media-organizer.git
cd plex-media-organizer

# Install dependencies
cargo build

# Set up configuration for testing
cargo run -- setup
```

## 🔧 Development Workflow

### 1. **Create a Feature Branch**
```bash
git checkout -b feature/your-feature-name
```

### 2. **Make Your Changes**
- Follow Rust coding standards
- Add comprehensive tests
- Update documentation as needed

### 3. **Run Tests**
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### 4. **Code Quality Checks**
```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Check for security issues
cargo audit

# Quick quality check (run before committing)
cargo fmt && cargo clippy && cargo test
```

### 5. **Commit Your Changes**
```bash
git add .
git commit -m "feat: add new feature description

- Detailed description of changes
- Any breaking changes
- Related issue numbers"
```

### 6. **Push and Create PR**
```bash
git push origin feature/your-feature-name
# Create Pull Request on GitHub
```

## 📝 Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples
```
feat: add fuzzy search for better movie matching
fix: resolve TMDB API timeout issues
docs: update architecture for database integration
test: add comprehensive test suite for organizer
```

## 📚 Documentation Requirements

### **Mandatory Documentation Updates**
For every feature PR, you **must** update:
- [ ] `docs/CURRENT_STATUS.md` - Add new features to "Recent Updates" section
- [ ] `docs/CURRENT_STATUS.md` - Update component status if needed
- [ ] `docs/CURRENT_STATUS.md` - Update "Next Steps" section
- [ ] `docs/CURRENT_STATUS.md` - Add performance metrics if applicable

### **Optional Documentation Updates**
- [ ] `docs/ARCHITECTURE.md` - For design changes
- [ ] `docs/IMPLEMENTATION_ROADMAP.md` - For plan updates
- [ ] `README.md` - For user-facing features
- [ ] `config_example.toml` - For new configuration options

**Remember**: Documentation is part of the feature, not a separate task!

## 🔧 Quality Assurance

### **Pre-PR Checklist**
- [ ] All tests pass (`cargo test`)
- [ ] Code formatting correct (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated (see above)
- [ ] Commit messages follow convention
- [ ] Feature branch is up to date with main

### **CI/CD Requirements**
- [ ] GitHub Actions pass
- [ ] Code coverage maintained
- [ ] Performance benchmarks pass
- [ ] Security checks pass

## 🚫 What NOT to Do

- ❌ **Never push directly to main**
- ❌ **Never create separate PRs for documentation** (unless it's a pure docs PR)
- ❌ **Never merge features without documentation updates**
- ❌ **Never skip the PR review process**

### Examples
```
feat(parser): add support for new filename pattern
fix(tmdb): resolve API rate limiting issue
docs: update README with new usage examples
test: add comprehensive test coverage for movie parser
```

## 🧪 Testing Guidelines

### Test Requirements
- **Unit Tests**: All new functions must have unit tests
- **Integration Tests**: Test complete workflows
- **Test Coverage**: Aim for >90% test coverage
- **Real Data**: Test with actual filename patterns

### Test Naming
```rust
#[test]
fn test_parse_chinese_english_bilingual() {
    // Test implementation
}

#[test]
fn test_parse_chinese_english_bilingual_with_special_characters() {
    // Test edge case
}
```

### Test Data
- Use realistic filename examples
- Include edge cases and error conditions
- Test both success and failure scenarios

## 🏗️ Architecture Guidelines

### Code Organization
- Keep modules focused and single-purpose
- Use clear, descriptive names for functions and variables
- Follow Rust idioms and best practices

### Error Handling
- Use `anyhow::Result<T>` for error propagation
- Provide meaningful error messages
- Include context for debugging

### Performance
- Profile code for bottlenecks
- Use efficient data structures
- Minimize allocations in hot paths

## 📚 Documentation

### Code Documentation
- Document all public functions and types
- Include usage examples
- Explain complex algorithms

### README Updates
- Update README.md for new features
- Include usage examples
- Update roadmap and status

## 🐛 Bug Reports

### Before Reporting
1. Check existing issues
2. Verify the bug with latest code
3. Test with minimal reproduction case

### Bug Report Template
```
**Description**
Clear description of the issue

**Steps to Reproduce**
1. Step 1
2. Step 2
3. Step 3

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., macOS 14.0]
- Rust Version: [e.g., stable (latest)]
- Plex Media Organizer Version: [e.g., 0.1.0]

**Additional Context**
Any other relevant information
```

## 💡 Feature Requests

### Before Requesting
1. Check if the feature is already planned
2. Consider the impact on existing functionality
3. Think about implementation complexity

### Feature Request Template
```
**Feature Description**
Clear description of the requested feature

**Use Case**
Why this feature is needed

**Proposed Implementation**
How you think it could be implemented

**Alternatives Considered**
Other approaches you've considered

**Additional Context**
Any other relevant information
```

## 🔍 Code Review Process

### Review Checklist
- [ ] Code follows Rust conventions
- [ ] Tests are comprehensive and pass
- [ ] Documentation is updated
- [ ] No performance regressions
- [ ] Error handling is appropriate
- [ ] Security considerations addressed

### Review Guidelines
- Be constructive and respectful
- Focus on code quality and correctness
- Suggest improvements when possible
- Ask questions for clarification

## 📊 Performance Considerations

### Benchmarking
- Use `criterion` for performance testing
- Profile with real-world data
- Monitor memory usage and allocations

### Optimization Guidelines
- Profile before optimizing
- Focus on hot paths
- Consider algorithmic improvements
- Measure impact of changes

## 🚨 Security

### Security Guidelines
- Never commit API keys or secrets
- Validate all user input
- Use secure defaults
- Follow Rust security best practices

### Reporting Security Issues
- **DO NOT** create public issues for security problems
- Email security issues to [your-email]
- Include detailed reproduction steps
- Allow time for response before disclosure

## 📞 Getting Help

### Communication Channels
- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Requests**: For code contributions

### Response Time
- We aim to respond to issues within 48 hours
- Code reviews typically completed within 1 week
- Major features may take longer for review

## 🎯 Contribution Areas

### High Priority
- Bug fixes and stability improvements
- Performance optimizations
- Enhanced parsing patterns
- Database integration

### Medium Priority
- Additional media type support
- CLI improvements
- Configuration enhancements
- Documentation updates

### Low Priority
- Nice-to-have features
- UI/UX improvements
- Additional API integrations

## 📋 Checklist for Contributors

Before submitting your contribution:

- [ ] Code follows Rust conventions
- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Commit message follows convention
- [ ] Branch is up to date with main
- [ ] Pull request description is clear

## 🙏 Thank You

Thank you for contributing to Plex Media Organizer! Your contributions help make this tool better for everyone in the media organization community.

---

**Happy coding! 🦀**
