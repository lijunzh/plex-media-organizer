# Current Limitations

## Overview

This document outlines the current limitations of the Plex Media Organizer, focusing on the English movie organization functionality. Understanding these limitations helps set appropriate expectations and guides future development priorities.

## 🎬 **English Movie Focus Limitations**

### **Media Type Scope**
- **Movies Only**: Currently supports English movies exclusively
- **No TV Shows**: Episode detection and season organization not implemented
- **No Music**: Music file organization and metadata not supported
- **No Mixed Content**: Cannot handle directories with multiple media types

### **Language Support**
- **English Movies Only**: Optimized for English-language movie content
- **No International Films**: Limited support for non-English movies
- **No Anime**: Japanese anime and other international content not supported
- **No Multi-Language**: Cannot handle movies with multiple language titles

## 🔧 **Technical Limitations**

### **API Dependencies**
- **TMDB Only**: Single external API dependency for movie metadata
- **No Fallback APIs**: Limited to TMDB for movie information
- **API Rate Limits**: Subject to TMDB's free tier rate limits
- **Internet Required**: Cannot function without internet access

### **File System Limitations**
- **Local Files Only**: Cannot organize cloud-stored or network files
- **Single Directory**: Processes one directory at a time
- **No Recursive Organization**: Cannot organize nested directory structures
- **File Permissions**: Requires appropriate file system permissions

### **Performance Constraints**
- **Sequential Processing**: Files processed one at a time (not fully parallel)
- **Memory Usage**: Large directories may consume significant memory
- **Database Size**: SQLite database grows with operation history
- **Cache Expiration**: TMDB cache expires after 24 hours

## 🛡️ **Safety Limitations**

### **Rollback Limitations**
- **Operation-Based**: Rollback only available for complete operations
- **No Partial Rollback**: Cannot rollback individual file changes
- **Database Dependency**: Rollback requires intact database
- **Time-Based**: Old operations may be cleaned up automatically

### **Error Recovery**
- **Stop on Error**: Processing stops on first critical error
- **No Automatic Retry**: Failed operations require manual intervention
- **Limited Error Context**: Some error messages may be generic
- **No Recovery Mode**: Cannot resume interrupted operations

## 📊 **Accuracy Limitations**

### **Parsing Accuracy**
- **Filename Dependent**: Accuracy depends on filename quality
- **TMDB Coverage**: Limited to movies available in TMDB database
- **Confidence Thresholds**: Low-confidence matches may be skipped
- **No Learning**: System doesn't learn from user corrections

### **Title Matching**
- **Exact Match Preferred**: Fuzzy matching has limitations
- **Year Sensitivity**: Incorrect years may cause matching failures
- **Title Variations**: Some title variations may not be recognized
- **No Context Awareness**: Cannot use surrounding files for context

## 🎯 **User Experience Limitations**

### **CLI Interface**
- **Command Line Only**: No graphical user interface
- **No Interactive Mode**: All operations require command-line input
- **Limited Progress**: Basic progress reporting only
- **No Real-Time Updates**: Cannot see changes as they happen

### **Configuration**
- **Manual Setup**: Requires manual API key configuration
- **Limited Customization**: Few configurable options
- **No Profiles**: Cannot save different configuration profiles
- **No Templates**: Cannot customize naming templates

## 📈 **Scalability Limitations**

### **Large Libraries**
- **Memory Constraints**: Very large directories may cause memory issues
- **Processing Time**: Large libraries take significant time to process
- **Database Performance**: Large operation histories may slow queries
- **No Batching**: Cannot process multiple directories simultaneously

### **Concurrent Usage**
- **Single User**: Designed for single-user operation
- **No Multi-Processing**: Cannot run multiple instances simultaneously
- **File Locking**: May conflict with other file operations
- **No Distributed Processing**: Cannot distribute across multiple machines

## 🔮 **Future Limitations (Planned Features)**

### **Phase 2: TV Series Support**
- **Episode Detection**: Will add TV show episode pattern recognition
- **TVDB Integration**: Will add TVDB API for TV show metadata
- **Season Organization**: Will support season-based directory structures
- **Mixed Content**: Will handle movies and TV shows in same directory

### **Phase 3: Music Support**
- **Music Detection**: Will add music file format recognition
- **MusicBrainz Integration**: Will add MusicBrainz API for music metadata
- **Artist/Album Organization**: Will support music-specific directory structures
- **Track Naming**: Will apply music-specific naming conventions

### **Phase 4: Multi-Language & Multi-API**
- **Language Detection**: Will add automatic language detection
- **Multiple APIs**: Will support multiple external databases
- **Anime Support**: Will add specialized anime content handling
- **International Films**: Will enhance non-English movie support

## 🎯 **Workarounds and Solutions**

### **For Current Limitations**

#### **Large Libraries**
- Process directories in smaller batches
- Use cleanup command to manage database size
- Monitor memory usage during processing

#### **API Rate Limits**
- Use caching to minimize API calls
- Process during off-peak hours
- Consider TMDB paid tier for higher limits

#### **Accuracy Issues**
- Review and manually correct low-confidence matches
- Use test command to preview results before organizing
- Provide feedback for future improvements

#### **File System Issues**
- Ensure proper file permissions
- Use dry-run mode to preview changes
- Keep backups before organizing

### **For Future Features**
- **TV Shows**: Use dedicated TV show organization tools until Phase 2
- **Music**: Use music-specific organization tools until Phase 3
- **International Content**: Use language-specific tools until Phase 4
- **Large Scale**: Consider dedicated media management solutions

## 📝 **Documentation of Limitations**

### **Why Document Limitations?**
- **Set Expectations**: Help users understand what the tool can and cannot do
- **Guide Development**: Prioritize features based on user needs
- **Prevent Misuse**: Avoid users attempting unsupported operations
- **Plan Workarounds**: Help users find alternative solutions

### **Limitation Categories**
1. **Current Limitations**: What the tool cannot do now
2. **Technical Limitations**: Constraints due to technology choices
3. **Design Limitations**: Intentional scope limitations
4. **Future Limitations**: Features planned for future phases

## 🎯 **Conclusion**

The Plex Media Organizer is designed as a focused, reliable tool for English movie organization. While it has specific limitations, these are intentional design choices that prioritize:

- **Safety and Reliability**: Comprehensive error handling and rollback capabilities
- **Accuracy**: High-quality parsing for English movies
- **Simplicity**: Clear, focused functionality
- **Extensibility**: Architecture ready for future expansion

Understanding these limitations helps users make informed decisions about when and how to use the tool, and guides the development team in prioritizing future improvements.
