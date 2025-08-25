# Documentation

Welcome to the Plex Media Organizer documentation. This guide helps you find the right documentation for your needs.

## 📚 **Documentation Structure**

### **For Users** (`user/`)
- **[Getting Started](user/getting-started.md)** - Quick start guide for new users
- **[Features Guide](user/features.md)** - Comprehensive usage documentation
- **[Configuration](user/configuration.md)** - All configuration options and settings
- **[Parsing Strategy](user/parsing-strategy.md)** - How filename parsing works
- **[Troubleshooting](user/troubleshooting.md)** - Common issues and solutions
- **[Examples](user/examples/)** - Real-world usage examples

### **For Users** (`user/`)
- **[CLI Commands Reference](user/cli-commands.md)** - Complete CLI command documentation

### **For Developers** (`developer/`)
- **[Git Practices](developer/git-practices.md)** - Git commit best practices
- **[Documentation Standards](developer/documentation-standards.md)** - Documentation guidelines
- **[Architecture](developer/architecture/)** - Technical architecture and design
  - **[Current Architecture](developer/architecture/architecture.md)** - Existing system design
  - **[Module Structure](developer/architecture/module-structure.md)** - Target architecture
  - **[Filename Parsing Strategy](architecture/filename-parsing-strategy.md)** - New TMDB-first parsing approach
  - **[Roadmap](developer/architecture/roadmap.md)** - Development phases and planning
  - **[Decisions](developer/architecture/decisions/)** - Architecture decision records
- **[Planning](developer/planning/)** - Development planning documents
- **[Retrospectives](developer/retrospectives/)** - Project retrospectives

### **Technical Analysis** (`analysis/`)
- **[Current Limitations](analysis/current-limitations.md)** - Known limitations and trade-offs
- **[TMDB Matching](analysis/tmdb-matching.md)** - TMDB integration analysis
- **[Edge Cases](analysis/edge-cases.md)** - Edge case analysis and handling
- **[Current Issues](analysis/current-issues.md)** - Current issues analysis
- **[Status](analysis/status.md)** - Project status and metrics

## 🎯 **Current Status: Complete CLI Workflow**

The Plex Media Organizer now provides a **complete, production-ready CLI application** with comprehensive media organization capabilities:

### **✅ Completed Features**
- **7 Core CLI Commands**: Complete workflow from setup to cleanup
- **Database-Backed Operations**: SQLite database with rollback capabilities
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Multi-language Support**: English, Chinese, Japanese, Arabic, Russian
- **Safety Features**: Preview mode, rollback, comprehensive error handling
- **Performance Optimization**: Caching, parallel processing, network optimization

### **🎬 CLI Commands**
1. **`setup`** - Interactive configuration setup
2. **`config`** - View and modify configuration
3. **`scan`** - Analyze media directories
4. **`test`** - Test parsing functionality
5. **`organize`** - Organize media files
6. **`rollback`** - Revert previous operations
7. **`cleanup`** - Database maintenance

### **📊 Technical Achievements**
- **96+ Unit Tests**: Comprehensive test coverage
- **Database Integration**: SQLite with operation history
- **Modular Architecture**: Clean separation of concerns
- **Performance Optimized**: Fast parsing and organization
- **Production Ready**: Conservative defaults and safety features

## 🚀 **Next Steps**

### **Immediate Priority: TV Show Support**
- Episode detection and season organization
- TVDB API integration
- Extend existing CLI commands for TV shows
- Mixed content handling (movies + TV shows)

### **Future Iterations**
- **Web Interface**: Browser-based management
- **Advanced Features**: Scheduled operations, batch processing
- **Music Support**: Music file organization and metadata
- **Production Optimization**: Performance and monitoring

## 🎯 **Quick Navigation**

### **New Users**
1. Start with **[Getting Started](user/getting-started.md)**
2. Read the **[CLI Commands Reference](user/cli-commands.md)** for detailed usage
3. Check **[Examples](user/examples/)** for real-world scenarios
4. Use **[Troubleshooting](user/troubleshooting.md)** if you encounter issues

### **Existing Users**
1. Review **[Configuration](user/configuration.md)** for advanced settings
2. Check **[Troubleshooting](user/troubleshooting.md)** for common issues
3. Explore **[Examples](user/examples/)** for advanced usage patterns

### **Contributors**
1. Follow **[Git Practices](developer/git-practices.md)** for proper commits
2. Review **[Architecture](developer/architecture/)** for technical understanding
3. Check **[Documentation Standards](developer/documentation-standards.md)** for guidelines
4. Review **[Planning](developer/planning/)** and **[Retrospectives](developer/retrospectives/)** for context

## 📋 **Quick Start Example**

```bash
# 1. Setup (first time only)
plex-media-organizer setup

# 2. Scan directory
plex-media-organizer scan /path/to/movies

# 3. Test parsing
plex-media-organizer test /path/to/movies --use-cache

# 4. Preview organization
plex-media-organizer organize /path/to/movies --preview

# 5. Organize files
plex-media-organizer organize /path/to/movies

# 6. Cleanup old operations
plex-media-organizer cleanup --keep-count 10
```

---

**Need help?** Check the **[Troubleshooting](user/troubleshooting.md)** guide for common issues and solutions.
