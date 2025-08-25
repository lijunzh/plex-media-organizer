# Plex Media Organizer - Architecture Design

## Overview

The Plex Media Organizer is a focused, production-ready CLI tool for organizing English movies using TMDB integration. The system follows a **TMDB-first strategy** with intelligent fallback parsing, designed to handle the complex naming conventions found in real-world media collections.

### **Current Focus: English Movies**
- **Primary Target**: `/Volume/media/movie/English` directory structure
- **TMDB Integration**: Enhanced metadata using The Movie Database API
- **Plex Compliance**: Strict adherence to official Plex naming conventions
- **Safety First**: Dry-run mode, complete rollback functionality, comprehensive error handling

### **Future Extensibility**
The architecture is designed to support future expansion to:
- **TV Series**: Episode detection, season organization, TVDB integration
- **Music**: Artist/album/track organization, MusicBrainz integration
- **Multi-Language**: Non-English content (anime, international films)
- **Multi-API**: Multiple external database integration

## Core Design Philosophy

### 1. **TMDB-First Strategy**
- Use TMDB as the authoritative source for movie metadata
- Intelligent fallback to filename parsing when API fails
- Fuzzy matching for handling naming variations and typos
- Comprehensive caching to minimize API calls

### 2. **Safety and Reliability**
- Dry-run mode for previewing changes
- Complete rollback functionality for all operations
- Comprehensive error handling and recovery
- Conservative defaults to prevent data loss

### 3. **Performance and Scalability**
- Async processing for large media libraries
- Smart caching to minimize external API calls
- Efficient database operations with SQLite
- Single binary design for direct file system access

### 4. **User Control**
- User-provided API keys for external services
- Configurable organization preferences
- Clear progress reporting and operation summaries
- Transparent operation logging

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Interface Layer                      │
├─────────────────────────────────────────────────────────────────┤
│                    Command Line Interface                        │
│                    Configuration Management                      │
│                    Progress Reporting                            │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Orchestration Layer                         │
├─────────────────────────────────────────────────────────────────┤
│                    Media Processing Pipeline                     │
│                    Workflow Coordinator                         │
│                    Error Handling & Recovery                    │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Core Engine Layer                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │   Scanner   │  │   Parser    │  │  Organizer  │            │
│  │             │  │             │  │             │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Data Access Layer                           │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │   Local     │  │   External  │  │    Cache    │            │
│  │  Database   │  │     APIs    │  │             │            │
│  │             │  │             │  │             │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. **User Interface Layer**

#### Command Line Interface
- **Subcommands**: `setup`, `config`, `scan`, `test`, `organize`, `rollback`, `cleanup`
- **Interactive Mode**: Guided setup and configuration
- **Batch Mode**: Process entire directories non-interactively
- **Progress Display**: Real-time progress bars and status updates
- **Safety Features**: Dry-run mode, rollback capability, backup management

#### Configuration Management
- **User-Provided API Keys**: Users provide their own TMDB API credentials
- **Organization Preferences**: User-defined naming patterns and preferences
- **Safety Settings**: Configurable risk tolerance and validation levels
- **Performance Tuning**: Concurrency and caching configuration

#### Progress Reporting
- **Real-time Updates**: Live progress during operations
- **Summary Reports**: Comprehensive results after completion
- **Error Logging**: Detailed error information for debugging
- **Success Metrics**: Statistics on parsing accuracy and improvements

### 2. **Orchestration Layer**

#### Media Processing Pipeline
- **Workflow Stages**: Scan → Parse → Organize → Report
- **Parallel Processing**: Handle multiple files concurrently
- **Dependency Management**: Ensure proper order of operations
- **Resource Management**: Control memory and CPU usage

#### Workflow Coordinator
- **State Management**: Track progress through pipeline stages
- **Error Recovery**: Handle failures and continue processing
- **Rollback Support**: Undo changes if critical errors occur
- **Resume Capability**: Continue interrupted operations

#### Error Handling & Recovery
- **Graceful Degradation**: Continue processing when components fail
- **Retry Logic**: Automatic retry for transient failures
- **User Notification**: Clear error messages and suggested solutions
- **Logging**: Comprehensive error logging for debugging

### 3. **Core Engine Layer**

#### Scanner Component
- **Directory Walking**: Recursively scan media directories
- **File Filtering**: Identify movie files by extension and content
- **Metadata Extraction**: Extract basic file information
- **Change Detection**: Identify new, modified, or deleted files

#### Parser Component (Movie-Focused)
- **TMDB Integration**: Primary source for movie metadata
- **Filename Parsing**: Fallback parsing for when API fails
- **Fuzzy Matching**: Find similar titles using string similarity
- **Context Analysis**: Use surrounding files and directories
- **Confidence Scoring**: Rate parsing confidence for decision making

#### Organizer Component
- **File Renaming**: Apply Plex naming conventions
- **Directory Structure**: Create organized folder hierarchies
- **Conflict Resolution**: Handle naming conflicts and duplicates
- **Metadata Preservation**: Maintain file metadata during operations

### 4. **Data Access Layer**

#### Local Database (SQLite)
- **Media Information**: Store parsed movie metadata
- **Operation History**: Track all organization operations
- **Cache Data**: Store TMDB API responses and parsing results
- **Rollback Data**: Store information needed for rollback operations
- **Unified Storage**: All data stored in single SQLite database

#### External APIs (TMDB)
- **Movie Database**: Primary source for movie metadata
- **User-Provided Keys**: Users provide their own API credentials
- **Rate Limiting**: Respect API quotas and limits
- **Caching**: Store responses to minimize API calls

#### Cache System
- **Unified SQLite Storage**: All cache data stored in single database
- **API Response Cache**: Store TMDB data with TTL expiration
- **Parsing Result Cache**: Cache successful parse results
- **Metadata Cache**: Store extracted file metadata

## Data Models

### Movie Information
```rust
pub struct MovieInfo {
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<u32>,
    pub tmdb_id: Option<u32>,
    pub confidence_score: f32,
    pub parsing_strategy: ParsingStrategy,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub quality: Option<String>,
    pub source: Option<String>,
}
```

### Parsing Result
```rust
pub struct ParsingResult {
    pub movie_info: MovieInfo,
    pub tmdb_data: Option<TmdbMovieData>,
    pub parsing_errors: Vec<String>,
    pub created_at: DateTime<Utc>,
}
```

### Organization Operation
```rust
pub struct OrganizationOperation {
    pub id: String,
    pub operation_type: OperationType,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub file_size: u64,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
}
```

## Plex Naming Convention Compliance

The system follows Plex's official naming conventions for optimal compatibility:

### **Movies**
- **Format**: `Movie Name (Year) {Edition} {Quality} {Source}.ext`
- **Examples**: 
  - `The Matrix (1999) 1080p BluRay.x264.mkv`
  - `Avengers Endgame (2019) Extended 4K HDR.mkv`
  - `Inception (2010) 720p WEB-DL.mkv`

### **Directory Structure**
- **Format**: `Movie Name (Year)/Movie Name (Year) {Quality} {Source}.ext`
- **Example**: `The Matrix (1999)/The Matrix (1999) 1080p BluRay.x264.mkv`

## Processing Workflow

### 1. **Scanning Phase**
```
Directory Input → File Discovery → Movie Classification → Metadata Extraction
```

### 2. **Parsing Phase**
```
Filename Analysis → TMDB API Lookup → Fuzzy Matching → Fallback Parsing → Result Generation
```

### 3. **Organization Phase**
```
Plex Naming Rules → File Renaming → Directory Creation → Conflict Resolution → Operation Logging
```

### 4. **Rollback Phase**
```
Operation History → File Restoration → Directory Cleanup → Database Cleanup
```

## Configuration System

### API Configuration
```toml
[apis]
# Users provide their own TMDB API key
tmdb_api_key = "your_tmdb_key_here"

[apis.rate_limits]
# Default limits based on free tier
tmdb_requests_per_day = 1000

[apis.setup]
# Instructions for users to obtain API keys
tmdb_setup_url = "https://www.themoviedb.org/settings/api"
```

### Organization Rules
```toml
[organization]
movies_template = "{title} ({year}) {quality}"
prefer_1080p = true
prefer_4k = false
minimum_quality = "720p"

[safety]
dry_run_default = true
require_confirmation = true
backup_before_organize = true
```

### Performance Configuration
```toml
[performance]
max_concurrent_files = 10
cache_ttl_hours = 24
database_connection_pool_size = 5
```

## Performance Considerations

### Caching Strategy
- **Unified SQLite Storage**: All cache data stored in single database
- **API Response Cache**: TTL-based expiration (24 hours for movies)
- **Parsing Result Cache**: Persistent storage with confidence-based expiration
- **Metadata Cache**: File hash-based with modification time checking

### Parallel Processing
- **File Scanning**: Concurrent directory traversal
- **API Requests**: Parallel TMDB API calls with rate limiting
- **File Operations**: Concurrent file renaming and moving
- **Database Operations**: Connection pooling and batch operations

### Memory Management
- **Streaming Processing**: Process files in batches to control memory usage
- **Lazy Loading**: Load metadata only when needed
- **Connection Pooling**: Reuse database and HTTP connections

## Security & Privacy

### Key Security Principles
- **User-Provided Keys**: Users provide their own TMDB API credentials
- **Local Processing**: All media analysis happens locally
- **Minimal External Data**: Only send necessary information to APIs
- **Secure Storage**: Encrypted storage of user-provided API credentials

### Data Privacy
- **Local Processing**: All media analysis happens locally
- **Minimal External Data**: Only send necessary information to APIs
- **User Control**: User decides what data to share or cache

## Error Handling Strategy

### Error Categories
1. **Transient Errors**: Network timeouts, temporary API failures
2. **Recoverable Errors**: Missing metadata, low confidence parsing
3. **Critical Errors**: File corruption, permission issues
4. **User Errors**: Invalid configuration, missing API keys

### Recovery Mechanisms
- **Automatic Retry**: Retry transient errors with exponential backoff
- **Fallback Strategies**: Use alternative parsing methods
- **User Notification**: Clear error messages with suggested solutions
- **Logging**: Comprehensive error logging for debugging

## Future Extensibility

### **Phase 2: TV Series Support**
- **Episode Detection**: Parse season/episode information from filenames
- **TVDB Integration**: Use TVDB API for TV show metadata
- **Season Organization**: Create season-based directory structures
- **Mixed Content**: Handle directories with both movies and TV shows

### **Phase 3: Music Support**
- **Music Detection**: Identify music files and extract metadata
- **MusicBrainz Integration**: Use MusicBrainz API for music metadata
- **Artist/Album Organization**: Create artist/album directory structures
- **Track Naming**: Apply music-specific naming conventions

### **Phase 4: Multi-Language & Multi-API**
- **Language Detection**: Automatically detect content language
- **Multiple APIs**: Support for multiple external databases
- **Anime Support**: Specialized handling for anime content
- **International Films**: Enhanced support for non-English content

### **Architecture Extensions**
- **Plugin System**: Extensible architecture for custom parsers
- **API Abstraction**: Unified interface for multiple external APIs
- **Content Classification**: Advanced media type detection
- **Learning System**: Pattern recognition and confidence building

## Testing Strategy

### Unit Testing
- **Component Isolation**: Test each component independently
- **Mock Dependencies**: Use mocks for external APIs and databases
- **Edge Cases**: Test error conditions and boundary cases

### Integration Testing
- **End-to-End Workflows**: Test complete processing pipelines
- **API Integration**: Test with real TMDB API
- **Database Operations**: Test database operations and migrations

### Performance Testing
- **Load Testing**: Test with large media libraries
- **Memory Profiling**: Monitor memory usage and leaks
- **Performance Benchmarks**: Measure processing speed improvements

## Deployment & Distribution

### Single Binary Design
- **Design Choice**: Single executable rather than client-server architecture
- **Rationale**: 
  - Simpler deployment and distribution
  - Direct file system access for better performance
  - No network complexity or IPC overhead
  - Easier installation and updates

### Binary Distribution
- **Cross-Platform**: Support for Windows, macOS, and Linux
- **Static Linking**: Minimize runtime dependencies
- **Self-Contained**: Include all necessary components
- **Single File**: One executable handles all operations

### Configuration Management
- **Default Configuration**: Sensible defaults for common use cases
- **Environment Variables**: Support for containerized deployments
- **Configuration Validation**: Validate configuration at startup
- **User API Keys**: Users provide their own external service credentials

## Conclusion

This architecture provides a solid foundation for a focused, reliable English movie organization system. By combining TMDB's authoritative data with intelligent fallback parsing, the system can handle the complex naming conventions found in real-world media collections while maintaining high accuracy and safety.

### **Key Architectural Benefits**
- **TMDB-First Strategy**: Leverages authoritative movie database
- **Safety-First Design**: Comprehensive error handling and rollback capabilities
- **Future-Ready**: Extensible architecture for TV, music, and multi-language support
- **User Control**: User-provided APIs and configurable preferences

The modular design allows for incremental implementation, starting with English movies and gradually adding support for other media types. The focus on safety and reliability ensures that users can trust the system with their media collections.
