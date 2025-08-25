# Documentation

Welcome to the Plex Media Organizer documentation. This guide helps you find the right documentation for your needs.

## 📚 **Documentation Structure**

### **For Users** (`user/`)
- **[Getting Started](user/getting-started.md)** - Quick start guide for new users
- **[Features Guide](user/features.md)** - Comprehensive usage documentation
- **[Configuration](user/configuration.md)** - All configuration options and settings
- **[Troubleshooting](user/troubleshooting.md)** - Common issues and solutions
- **[Examples](user/examples/)** - Real-world usage examples
- **[CLI Commands Reference](user/cli-commands.md)** - Complete CLI command documentation

### **For Developers** (`developer/`)
- **[Architecture](developer/architecture/)** - Technical architecture and design
  - **[Current Architecture](developer/architecture/architecture.md)** - System design and implementation
  - **[Development Roadmap](developer/architecture/roadmap.md)** - Future development plans
- **[Git Practices](developer/git-practices.md)** - Git commit best practices
- **[Documentation Standards](developer/documentation-standards.md)** - Documentation guidelines

### **Technical Analysis** (`analysis/`)
- **[Current Status](analysis/status.md)** - Project status and metrics
- **[Known Limitations](analysis/current-limitations.md)** - Current limitations and trade-offs

## 🎯 **Current Status: English Movie Organization**

The Plex Media Organizer provides a **production-ready CLI application** focused on **English movie organization** with TMDB integration:

### **✅ Completed Features**
- **7 Core CLI Commands**: Complete workflow from setup to cleanup
- **TMDB-First Strategy**: Enhanced metadata using The Movie Database API
- **English Movie Focus**: Optimized for `/Volume/media/movie/English` directory structure
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
- **Comprehensive Test Suite**: Real-world validation with high success rates
- **Database Integration**: SQLite with operation history and rollback
- **Modular Architecture**: Clean separation of concerns
- **Production Ready**: Conservative defaults and safety features

## 🚀 **Future Roadmap**

### **Phase 2: TV Series Support**
- Episode detection and season organization
- TVDB API integration
- Extend existing CLI commands for TV shows

### **Phase 3: Music Support**
- Music file organization and metadata
- MusicBrainz API integration

### **Phase 4: Multi-Language & Multi-API**
- Non-English content support (anime, international films)
- Multiple external database integration
- Advanced language detection

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

## 📋 **Quick Start Example**

```bash
# 1. Setup (first time only)
plex-media-organizer setup

# 2. Scan English movie directory
plex-media-organizer scan /Volume/media/movie/English

# 3. Test parsing
plex-media-organizer test /Volume/media/movie/English --use-cache

# 4. Preview organization
plex-media-organizer organize /Volume/media/movie/English --preview

# 5. Organize files
plex-media-organizer organize /Volume/media/movie/English

# 6. Cleanup old operations
plex-media-organizer cleanup --keep-count 10
```

---

**Need help?** Check the **[Troubleshooting](user/troubleshooting.md)** guide for common issues and solutions.
