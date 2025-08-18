# 📚 Plex Media Organizer Documentation

Complete documentation for users, power users, and developers.

## 🚀 Quick Navigation

### **New Users - Get Started Fast**
- **[🚀 Quick Start Guide](quick-start.md)** - 5-minute setup and first use
- **[📦 Installation](installation.md)** - All installation methods and platforms
- **[💡 First Movie Organization](examples/basic/first-movie-organization.md)** - Step-by-step first project

### **All Users - Complete Guides**
- **[📖 User Guide](user-guide.md)** - Comprehensive usage tutorial and workflows
- **[⚙️ Configuration](configuration.md)** - Complete configuration reference
- **[🔧 Troubleshooting](troubleshooting.md)** - Common issues and solutions

### **Examples and Use Cases**
- **[💡 Examples Overview](examples/)** - Real-world usage examples
- **[🎬 Basic Examples](examples/basic/)** - Simple, common use cases
- **[⚡ Advanced Examples](examples/advanced/)** - Complex scenarios and workflows
- **[🔌 Integrations](examples/integrations/)** - Media server setup (Plex, Jellyfin)

### **Developers and Contributors**
- **[🏗️ Development Guide](development/)** - Code architecture and contribution guide
- **[📋 API Reference](development/api-reference.md)** - Library API documentation

### **Project Information**
- **[📊 Project Status](../project/status.md)** - Current development progress
- **[🏛️ Architecture](../project/architecture.md)** - System design and decisions
- **[🗺️ Roadmap](../project/roadmap.md)** - Development timeline and planning

## 🎯 Documentation by User Type

### **Light Users** - "I want to get started ASAP"
1. **[Quick Start](quick-start.md)** - 5-minute setup
2. **[First Organization](examples/basic/first-movie-organization.md)** - Your first success
3. **[Plex Setup](examples/integrations/plex-server-setup.md)** - Complete Plex integration

### **Power Users** - "I want to understand all capabilities"
1. **[User Guide](user-guide.md)** - Complete feature overview
2. **[Configuration](configuration.md)** - All settings and options
3. **[Advanced Examples](examples/advanced/)** - Complex workflows
4. **[Troubleshooting](troubleshooting.md)** - Fix any issues

### **Developers** - "I want to understand the code and contribute"
1. **[Development Guide](development/)** - Code architecture overview
2. **[API Reference](development/api-reference.md)** - Library documentation
3. **[Architecture](../project/architecture.md)** - System design philosophy
4. **[Contributing](../CONTRIBUTING.md)** - How to contribute

## 📊 Quick Reference

### **Essential Commands**
```bash
# Test setup
plex-media-organizer test "Movie.Name.2023.1080p.mkv"

# Scan directory
plex-media-organizer scan /path/to/movies

# Organize with preview
plex-media-organizer organize /movies --dry-run

# Full organization with backup
plex-media-organizer organize /movies --backup
```

### **Common Issues**
| Issue | Solution |
|-------|----------|
| API key error | Set `TMDB_API_KEY=your_key` |
| No files found | Check extensions: mkv, mp4, avi |
| Low success rate | Enable debug: `--log-level debug` |
| Permission error | Fix permissions: `chmod 755` |

### **Best Practices**
- ✅ Always use `--dry-run` first
- ✅ Create backups with `--backup`
- ✅ Start with small test directories
- ✅ Check [troubleshooting](troubleshooting.md) for issues

## 🔄 Documentation Structure

```
docs/
├── README.md              # This navigation guide
├── quick-start.md         # 5-minute setup guide
├── installation.md        # Installation methods
├── user-guide.md          # Complete user manual
├── configuration.md       # Configuration reference
├── troubleshooting.md     # Issue solutions
├── examples/              # Real-world examples
│   ├── basic/            # Simple use cases
│   ├── advanced/         # Complex workflows
│   └── integrations/     # Media server setup
└── development/          # Developer documentation
    ├── code-architecture.md
    ├── api-reference.md
    └── adding-features.md
```

## 🎯 Success Metrics

After reading the appropriate documentation, you should be able to:

### **After Quick Start**
- ✅ Install and configure the tool
- ✅ Parse your first movie file successfully
- ✅ Understand the basic workflow

### **After User Guide**
- ✅ Organize complete movie collections
- ✅ Handle different file types and naming patterns
- ✅ Configure advanced settings for your needs

### **After Examples**
- ✅ Set up integration with Plex or Jellyfin
- ✅ Handle complex multi-language collections
- ✅ Automate your media organization workflow

### **After Development Guide**
- ✅ Understand the codebase architecture
- ✅ Know where to make specific types of changes
- ✅ Contribute features or fixes confidently

## 🆘 Need Help?

1. **Check this documentation** - Most questions are answered here
2. **Search issues** - [GitHub Issues](https://github.com/lijunzh/plex-media-organizer/issues)
3. **Ask questions** - [GitHub Discussions](https://github.com/lijunzh/plex-media-organizer/discussions)
4. **Report bugs** - [New Issue](https://github.com/lijunzh/plex-media-organizer/issues/new)

## 📈 Documentation Quality

This documentation is designed to be:
- **User-focused**: Organized by what you want to achieve
- **Example-driven**: Real commands and expected outputs
- **Comprehensive**: Covers basic to advanced usage
- **Maintainable**: Updated with each release

---

**🎯 Goal**: Get you from zero to successfully organizing your media collection with confidence!

**⏱️ Time Investment**: 5 minutes to hours, depending on your needs and collection size.

**💡 Pro Tip**: Start with [Quick Start](quick-start.md) even if you're an experienced user - it establishes good practices for more complex scenarios.
