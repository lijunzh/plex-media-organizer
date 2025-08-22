# Plex Media Organizer - Architecture Design

## Overview

The Plex Media Organizer is a database-driven media management system that intelligently parses, identifies, and organizes media files according to Plex naming conventions. Instead of relying on rigid regex patterns, the system uses external APIs, local learning databases, and fuzzy matching to handle the complex and varied naming conventions found in real-world media collections.

### **Architecture Philosophy**
- **Single Binary Design**: CLI tool approach for simplicity and performance
- **User-Provided APIs**: Users manage their own external service credentials
- **Unified Storage**: Single SQLite database for all data and cache
- **Plex Compliance**: Strict adherence to official Plex naming conventions

### **Current Implementation Status (Phase 2.1 Complete)**
- **✅ Movie Parsing**: Complete with TMDB integration and fuzzy matching
- **✅ File Organization**: Plex naming conventions with dry-run and rollback
- **✅ CLI Interface**: Full subcommand support with progress reporting
- **✅ Testing**: Comprehensive test suite with real-world validation (110+ tests, 100% success rate)
- **✅ Database Layer**: SQLite with connection pooling, WAL mode, and unified cache
- **✅ Configuration System**: Flexible configuration with environment variable overrides
- **✅ Safety Features**: Dry-run mode, complete rollback functionality, backup management
- **🔄 Architecture Refactoring**: In progress - transforming to modular structure
- **⏸️ TV Support**: Planned for Iteration 3
- **⏸️ Music Support**: Planned for Iteration 4
- **⏸️ Learning System**: Planned for Phase 2.2

## Core Design Philosophy

### 1. **Intelligence Over Rules**
- Use external databases for authoritative data instead of guessing
- Learn from successful parses to improve future accuracy
- Adapt to different naming conventions automatically

### 2. **Graceful Degradation**
- External APIs as primary source of truth
- Local parsing as fallback when APIs fail
- User manual correction as final fallback

### 3. **Continuous Learning**
- Build knowledge base from successful operations
- Learn user preferences and corrections
- Improve accuracy over time

### 4. **Performance & Scalability**
- Async processing for large media libraries
- Smart caching to minimize external calls
- Efficient database operations
- Unified SQLite storage for optimal performance
- Single binary design for direct file system access

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
│  │   Scanner   │  │  Classifier │  │   Parser    │            │
│  │             │  │             │  │             │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Organizer  │  │   Reporter  │  │   Learner   │            │
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
- **Subcommands**: `scan`, `setup`, `config`, `test`, `organize`, `rollback`, `cleanup`
- **Interactive Mode**: Guided setup and configuration
- **Batch Mode**: Process entire directories non-interactively
- **Progress Display**: Real-time progress bars and status updates
- **Safety Features**: Dry-run mode, rollback capability, cleanup management
- **Single Binary**: All functionality in one executable for simplicity

#### Configuration Management
- **User-Provided API Keys**: Users provide their own external service credentials
- **Preferences**: User-defined organization rules and preferences
- **Profiles**: Different configurations for different media types
- **Templates**: Customizable naming patterns following Plex conventions

#### Progress Reporting
- **Real-time Updates**: Live progress during operations
- **Summary Reports**: Comprehensive results after completion
- **Error Logging**: Detailed error information for debugging
- **Success Metrics**: Statistics on parsing accuracy and improvements

### 2. **Orchestration Layer**

#### Media Processing Pipeline
- **Workflow Stages**: Scan → Classify → Parse → Organize → Report
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
- **File Filtering**: Identify media files by extension and content
- **Metadata Extraction**: Extract basic file information
- **Change Detection**: Identify new, modified, or deleted files

#### Classifier Component
- **Media Type Detection**: Determine if file is movie, TV, music, or subtitle
- **Content Analysis**: Analyze file headers and metadata
- **Context Analysis**: Use directory structure for classification
- **Confidence Scoring**: Rate classification confidence

**Current Scope (Iteration 1)**: Focused on movie detection and parsing only

#### Parser Component
- **Multi-Strategy Parsing**: Try different parsing approaches
- **External Data Lookup**: Query external APIs for authoritative data
- **Fuzzy Matching**: Find similar titles using string similarity
- **Context Integration**: Use surrounding files and directories
- **Language Detection**: Identify content languages automatically

#### Organizer Component
- **File Renaming**: Apply Plex naming conventions
- **Directory Structure**: Create organized folder hierarchies
- **Metadata Embedding**: Write metadata to media files
- **Conflict Resolution**: Handle naming conflicts and duplicates

#### Reporter Component
- **Operation Summary**: Comprehensive results report
- **Statistics**: Parsing accuracy, file counts, improvements
- **Recommendations**: Suggestions for improving organization
- **Export Options**: Save reports in various formats

#### Learner Component
- **Pattern Recognition**: Identify successful parsing patterns
- **User Feedback Integration**: Learn from manual corrections
- **Confidence Building**: Improve accuracy over time
- **A/B Testing**: Evaluate different parsing strategies

## Architecture Refactoring

### **Current Status: Refactoring in Progress**
The codebase is currently undergoing a comprehensive refactoring to improve modularity and maintainability. The refactoring transforms the current monolithic structure into a well-organized, modular architecture.

### **Refactoring Goals**
- **Reduce file sizes**: Break down large files (>1000 lines) into focused modules
- **Improve separation of concerns**: Each module has a single, well-defined responsibility
- **Enhance maintainability**: Easier to understand, modify, and extend
- **Better testing**: Smaller modules are easier to test independently
- **Clearer dependencies**: Well-defined module boundaries and import structure

### **Target Architecture**
The refactoring will result in the following module structure:

```
📁 src/
├── 🎯 main.rs (13 lines) - Entry point
├── 📄 lib.rs (232 lines) - Library exports
├── 📄 types.rs (271 lines) - Shared data structures
├── 📄 config.rs (804 lines) - Configuration management
├── 📁 cli/ (1,306 lines → modular)
│   ├── 📄 mod.rs
│   ├── 📄 commands.rs
│   ├── 📁 handlers/ (7 command handlers)
│   └── 📄 output.rs
├── 📁 parsers/ (2,283 lines → unified)
│   ├── 📄 mod.rs
│   ├── 📄 movie.rs (merged movie_parser + filename_parser)
│   ├── 📁 patterns/ (series, anime, technical)
│   └── 📄 types.rs
├── 📁 external/ (883 lines → focused)
│   ├── 📄 mod.rs
│   ├── 📁 tmdb/ (client, search)
│   └── 📄 types.rs
├── 📁 core/ (1,653 lines → orchestration)
│   ├── 📄 mod.rs
│   ├── 📄 scanner.rs (file discovery only)
│   ├── 📄 processor.rs (processing orchestration)
│   └── 📄 organizer.rs (file organization only)
├── 📁 media/ (363 lines → focused)
│   ├── 📄 mod.rs
│   ├── 📄 extractor.rs
│   └── 📄 types.rs
└── 📁 database/ (1,241 lines → keep as-is)
    ├── 📄 mod.rs
    ├── 📄 schema.rs
    ├── 📄 connection.rs
    ├── 📄 cache.rs
    └── 📄 movies.rs
```

### **Refactoring Phases**
1. **Phase 1A**: CLI Refactoring - Extract command handlers to separate modules
2. **Phase 1B**: Parser Unification - Merge movie_parser and filename_parser
3. **Phase 1C**: External API Restructuring - Split tmdb_client into focused modules
4. **Phase 1D**: Core Logic Separation - Refactor scanner and organizer
5. **Phase 2**: Testing & Validation - Update tests and validate functionality

### **Benefits of Refactoring**
- **Improved maintainability**: Smaller, focused modules
- **Better organization**: Clear separation of concerns
- **Enhanced testability**: Each module can be tested independently
- **Easier extension**: Clear extension points for new features
- **Reduced complexity**: Easier to understand and navigate

For detailed information about the refactoring strategy, see [Holistic Refactoring Strategy](./holistic-refactoring-strategy.md) and [Module Structure Documentation](./module-structure.md).

## Implementation Status

### **✅ Completed Components (Phase 2.1)**

#### **User Interface Layer**
- **✅ CLI Interface**: Complete with all subcommands (`scan`, `setup`, `config`, `test`, `organize`, `rollback`, `cleanup`)
- **✅ Progress Reporting**: Real-time progress bars and verbose output modes
- **✅ Configuration Management**: Interactive setup and configuration validation
- **✅ Safety Features**: Dry-run mode, rollback capability, backup management

#### **Orchestration Layer**
- **✅ Media Processing Pipeline**: Complete workflow from scan to organize
- **✅ Error Handling**: Comprehensive error recovery and user notification
- **✅ Parallel Processing**: Configurable concurrency for large directories
- **✅ Resource Management**: Memory-efficient processing with batch operations

#### **Core Engine Layer**
- **✅ Scanner Component**: File discovery, filtering, and metadata extraction
- **✅ Parser Component**: Multi-strategy parsing with TMDB integration
- **✅ Organizer Component**: Plex naming conventions with conflict resolution
- **✅ Reporter Component**: Comprehensive operation summaries and statistics

#### **Data Access Layer**
- **✅ Local Database**: SQLite with connection pooling and WAL mode
- **✅ External APIs**: TMDB integration with rate limiting and caching
- **✅ Unified Cache**: Single database for all cache data and metadata

### **⏸️ Planned Components (Future Phases)**

#### **Core Engine Layer**
- **⏸️ Classifier Component**: Advanced media type detection (TV, music, subtitles)
- **⏸️ Learner Component**: Pattern recognition and confidence building

#### **Extended Media Support**
- **⏸️ TV Show Support**: Season/episode parsing and organization
- **⏸️ Music Support**: Artist/album/track parsing and organization
- **⏸️ Subtitle Support**: Multi-language subtitle management

### **📊 Current Metrics**
- **Test Coverage**: 110+ tests with 100% success rate
- **Performance**: 185+ files/second processing speed
- **Accuracy**: 100% success rate on real-world media collections
- **Code Quality**: Clean compilation with no warnings
- **Documentation**: Comprehensive user and developer documentation

### 4. **Data Access Layer**

#### Local Database (SQLite)
- **Media Information**: Store parsed media metadata
- **Pattern Database**: Successful parsing patterns and rules
- **User Corrections**: Manual fixes and preferences
- **Learning Data**: Statistics and improvement metrics
- **Unified Storage**: All data including cache stored in single SQLite database
- **Portability**: Single database file for easy backup and migration
- **ACID Compliance**: Reliable data integrity and transaction support

#### External APIs
- **TMDB (The Movie Database)**: Movies and TV shows
- **TVDB (The TV Database)**: Television series
- **MusicBrainz**: Music and audio content
- **AniDB**: Anime and Japanese content
- **User-Provided Keys**: Users provide their own API keys for external services
- **Rate Limiting**: Respect API quotas and limits based on user's API tier
- **Benefits**: 
  - No central API key management required
  - Users control their own rate limits and costs
  - Standard practice in media management tools
  - More reliable and secure approach

#### Enhanced TMDB Integration
The TMDB client implements advanced matching strategies for improved accuracy:

**Fuzzy Search Integration**
- **SkimMatcherV2**: High-quality fuzzy matching algorithm for title similarity
- **Confidence Scoring**: Intelligent scoring system with minimum thresholds
- **Title Variations**: Handles typos, abbreviations, and naming variations

**Multiple Search Strategies**
1. **Exact Search with Year**: Direct match with release year for highest accuracy
2. **Broader Search**: Search without year constraints for flexibility
3. **Cleaned Title Search**: Remove common suffixes/prefixes for better matching
4. **Alternative Variations**: Generate title variations for comprehensive coverage

**Title Processing**
- **Suffix Removal**: `(Director's Cut)`, `(Extended)`, `(Uncut)`, etc.
- **Prefix Removal**: `The`, `A`, `An` articles
- **Variation Generation**: Add/remove "The", handle sequels
- **Number Handling**: Remove sequel numbers for better matching

**Scoring System**
- **Exact Matches**: 200 points for perfect matches
- **Fuzzy Matches**: 1.5x boost multiplier for similar titles
- **Year Proximity**: 100/50/25 points for exact/1yr/3yr matches
- **Popularity Bonus**: TMDB popularity score integration
- **Rating Bonus**: Vote average consideration
- **Confidence Threshold**: Minimum 50.0 points for valid matches

#### Cache System
- **Unified SQLite Storage**: All cache data stored in single database file
- **API Response Cache**: Store external data to minimize calls
- **Parsing Result Cache**: Cache successful parse results
- **Metadata Cache**: Store extracted file metadata
- **Expiration Logic**: Automatically refresh stale data
- **Database Benefits**: Single file for backup, efficient queries, no separate cache management

## Data Models

### Plex Naming Convention Compliance
The system follows Plex's official naming conventions for optimal compatibility:

#### **Movies**
- **Format**: `Movie Name (Year) {Edition} {Quality} {Source}.ext`
- **Examples**: 
  - `The Matrix (1999) 1080p BluRay.x264.mkv`
  - `Avengers Endgame (2019) Extended 4K HDR.mkv`
  - `白蛇2：青蛇劫起 (2021) 1080p WEB-DL.mkv`

##### **CJK (Chinese/Japanese/Korean) Title Strategy**
The system supports flexible title organization for CJK content to address reliability issues with English translations:

**English-First Approach (Default)**:
- Uses English titles for directory organization when available
- Preserves original CJK titles in metadata
- Example: `Hero (2002)/Hero (2002) 1080p BluRay.mkv` with metadata `original_title="英雄"`

**Original-First Approach (Configurable)**:
- Uses original CJK titles for directory organization
- Includes English titles in metadata for reference
- Example: `英雄 (2002)/英雄 (2002) 1080p BluRay.mkv` with metadata `english_title="Hero"`

**Hybrid Approach (Optional)**:
- Combines both titles in filenames
- Example: `英雄 [Hero] (2002)/英雄 [Hero] (2002) 1080p BluRay.mkv`

**Rationale**: English titles for CJK movies are often unreliable due to:
- Marketing adaptations vs literal translations
- Regional variations (US vs UK vs local markets)
- Cultural context loss in translation
- Multiple "correct" English titles for the same film

#### **TV Shows (All Types)**
- **Format**: `Show Name/Season XX/Show Name SXXEXX Episode Title {Quality} {Source}.ext`
- **Examples**:
  - **Western TV**: `Breaking Bad/Season 01/Breaking Bad S01E01 Pilot 720p HDTV.mkv`
  - **Chinese TV**: `琅琊榜/Season 01/琅琊榜 S01E01 琅琊榜 720p.mkv`
  - **Anime**: `Attack on Titan/Season 01/Attack on Titan S01E01 To You, 2,000 Years Ago 1080p.mkv`

#### **Music**
- **Format**: `Artist/Album/Track - Title.ext`
- **Examples**:
  - `Pink Floyd/The Wall/02 - Another Brick in the Wall.flac`
  - `周杰伦/叶惠美/01 - 以父之名.mp3`

### Media File
```rust
pub struct MediaFile {
    pub id: String,
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
    pub media_type: MediaType,
    pub content_hash: String,
    pub last_modified: DateTime<Utc>,
    pub metadata: MediaMetadata,
}
```

### Media Metadata
```rust
pub struct MediaMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<u32>,
    pub language: Vec<String>,
    pub quality: Option<String>,
    pub source: Option<String>,
    pub duration: Option<Duration>,
    pub resolution: Option<Resolution>,
    pub codec: Option<String>,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
}
```

### Parsing Result
```rust
pub struct ParsingResult {
    pub media_file: MediaFile,
    pub parsed_metadata: MediaMetadata,
    pub confidence_score: f32,
    pub parsing_strategy: ParsingStrategy,
    pub external_sources: Vec<ExternalSource>,
    pub user_corrections: Vec<UserCorrection>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Learning Pattern
```rust
pub struct LearningPattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub pattern_data: String,
    pub success_count: u32,
    pub failure_count: u32,
    pub confidence_score: f32,
    pub examples: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}
```

## Processing Workflow

### 1. **Scanning Phase**
```
Directory Input → File Discovery → Media Classification → Metadata Extraction
```

### 2. **Parsing Phase**
```
Filename Analysis → External API Lookup → Fuzzy Matching → Context Analysis → Result Generation
```

### 3. **Organization Phase**
```
Plex Naming Rules → File Renaming → Directory Creation → Metadata Embedding → Conflict Resolution
```

### 4. **Learning Phase**
```
Success Analysis → Pattern Recognition → User Feedback Integration → Confidence Building → Strategy Improvement
```

## Configuration System

### API Configuration
```toml
[apis]
# Users provide their own API keys for external services
tmdb_api_key = "your_tmdb_key_here"
tvdb_api_key = "your_tvdb_key_here"
musicbrainz_user_agent = "PlexMediaOrganizer/1.0"
anidb_username = "your_anidb_username_here"
anidb_password = "your_anidb_password_here"

[apis.rate_limits]
# Default limits based on free tier APIs
tmdb_requests_per_day = 1000
tvdb_requests_per_day = 1000
musicbrainz_requests_per_second = 1
anidb_requests_per_second = 0.5

[apis.setup]
# Instructions for users to obtain API keys
tmdb_setup_url = "https://www.themoviedb.org/settings/api"
tvdb_setup_url = "https://thetvdb.com/api-information"
musicbrainz_setup_url = "https://musicbrainz.org/doc/MusicBrainz_API"
anidb_setup_url = "https://anidb.net/perl-bin/animedb.pl?show=login"
```

### Organization Rules
```toml
[organization]
movies_template = "{title} ({year}) {quality}"
tv_shows_template = "{title}/Season {season:02}/{title} S{season:02}E{episode:02} {episode_title}"
music_template = "{artist}/{album}/{track:02} - {title}"

[organization.quality]
prefer_1080p = true
prefer_4k = false
minimum_quality = "720p"

[organization.cjk_titles]
# CJK (Chinese/Japanese/Korean) title handling strategy
prefer_original_titles = false              # Use original CJK titles for organization
include_english_subtitle = false            # Add English title in brackets: 英雄 [Hero] (2002)
fallback_to_english_on_error = true         # Use English if CJK causes file system issues
preserve_original_in_metadata = true        # Always keep original title in metadata
```

### Learning Configuration
```toml
[learning]
enable_pattern_learning = true
confidence_threshold = 0.7
max_patterns_per_type = 1000
pattern_expiration_days = 365
user_feedback_weight = 0.8
```

## Performance Considerations

### Caching Strategy
- **Unified SQLite Storage**: All cache data stored in single database file
- **API Response Cache**: TTL-based expiration (24 hours for movies, 7 days for TV)
- **Parsing Result Cache**: Persistent storage with confidence-based expiration
- **Metadata Cache**: File hash-based with modification time checking
- **Database Benefits**: 
  - Single file for backup and migration
  - ACID compliance for data integrity
  - Efficient queries and indexing
  - No separate cache management

### Parallel Processing
- **File Scanning**: Concurrent directory traversal
- **API Requests**: Parallel external API calls with rate limiting
- **File Operations**: Concurrent file renaming and moving
- **Database Operations**: Connection pooling and batch operations

### Memory Management
- **Streaming Processing**: Process files in batches to control memory usage
- **Lazy Loading**: Load metadata only when needed
- **Connection Pooling**: Reuse database and HTTP connections

### Database Performance
- **Unified Storage**: Single SQLite database for all operations
- **Efficient Indexing**: Optimized indexes for media queries and pattern matching
- **Connection Management**: Single database connection with proper pooling
- **Transaction Support**: Batch operations for better performance

## Security & Privacy

For detailed security information, API key management, and best practices, see [Security & Privacy](../security.md).

### Key Security Principles
- **User-Provided Keys**: Users provide their own API keys for external services
- **Secure Storage**: Encrypted storage of user-provided API credentials
- **Environment Variables**: Support for external credential management
- **Key Rotation**: Automatic handling of expired or rotated keys
- **Setup Guidance**: Clear instructions for obtaining API keys from each service

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

## Testing Strategy

### Unit Testing
- **Component Isolation**: Test each component independently
- **Mock Dependencies**: Use mocks for external APIs and databases
- **Edge Cases**: Test error conditions and boundary cases

### Integration Testing
- **End-to-End Workflows**: Test complete processing pipelines
- **API Integration**: Test with real external APIs
- **Database Operations**: Test database operations and migrations

### Performance Testing
- **Load Testing**: Test with large media libraries
- **Memory Profiling**: Monitor memory usage and leaks
- **Performance Benchmarks**: Measure processing speed improvements

## Deployment & Distribution

### Architecture Decision: Single Binary (CLI Tool)
- **Design Choice**: Single executable rather than client-server architecture
- **Rationale**: 
  - Simpler deployment and distribution
  - No network complexity or IPC overhead
  - Direct file system access for better performance
  - Easier installation and updates
  - No need to manage server processes

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

### Storage Locations
- **Database**: Single SQLite file for all data and cache
- **Configuration**: Standard application data directories
  - **macOS**: `~/Library/Application Support/plex-media-organizer/`
  - **Linux**: `~/.config/plex-media-organizer/`
  - **Windows**: `%APPDATA%\plex-media-organizer\`
- **Logs**: Same directory as configuration
- **Cache**: Integrated into SQLite database

### Update Mechanism
- **Version Checking**: Check for updates on startup
- **Automatic Updates**: Optional automatic update downloads
- **Rollback Support**: Ability to revert to previous versions

## Architecture Decisions & Rationale

### Why Single Binary Instead of Client-Server?

#### **Client-Server Approach (Rejected)**
- **Complexity**: Requires managing server processes, network communication, IPC
- **Deployment**: More complex installation and configuration
- **Performance**: Network/IPC overhead for file operations
- **Maintenance**: Server process management, crash recovery
- **Distribution**: Multiple components to distribute and update

#### **Single Binary Approach (Selected)**
- **Simplicity**: One executable, direct file system access
- **Performance**: No IPC overhead, direct memory access
- **Deployment**: Single file distribution and installation
- **Reliability**: No server process to manage or crash
- **Portability**: Works on any system without network setup

#### **When Client-Server Makes Sense**
- **Multi-user environments**: Multiple users accessing same media library
- **Remote processing**: Processing media on different machines
- **Web interface**: Browser-based media management
- **Distributed processing**: Processing across multiple machines

### Future Enhancements

### Machine Learning Integration
- **Neural Networks**: Deep learning for title parsing
- **Natural Language Processing**: Better understanding of episode titles
- **Image Recognition**: Extract information from media thumbnails

### Community Features
- **Pattern Sharing**: Share successful parsing patterns
- **Collaborative Learning**: Learn from community corrections
- **Plugin System**: Extensible architecture for custom parsers

### Cloud Integration
- **Remote Databases**: Sync learning data across devices
- **Cloud Storage**: Organize cloud-stored media
- **Collaborative Organization**: Multi-user media organization

## Development Process



### 🚀 **Process Improvements**

#### **Iteration Development Process**
1. **Planning**: Define clear goals and success criteria
2. **Development**: Implement functionality
3. **Code Review**: Systematic review against goals
4. **Gap Analysis**: Identify and document gaps
5. **Gap Resolution**: Address gaps before completion
6. **Validation**: Final validation against user expectations
7. **Documentation**: Update all documentation
8. **Completion**: Mark iteration complete

#### **Code Review Template**
```
## Iteration [X] Code Review

### Files Reviewed
- [ ] src/main.rs
- [ ] src/cli.rs
- [ ] src/types.rs
- [ ] src/config.rs
- [ ] src/movie_parser.rs
- [ ] src/scanner.rs
- [ ] src/tmdb_client.rs
- [ ] tests/
- [ ] docs/

### Goals Validation
- [ ] [Goal 1] - [Status]
- [ ] [Goal 2] - [Status]
- [ ] [Goal 3] - [Status]

### User Expectations
- [ ] [Expectation 1] - [Status]
- [ ] [Expectation 2] - [Status]
- [ ] [Expectation 3] - [Status]

### Gaps Identified
- [ ] [Gap 1] - [Action needed]
- [ ] [Gap 2] - [Action needed]

### Completion Decision
- [ ] All gaps addressed
- [ ] All goals met
- [ ] User expectations satisfied
- [ ] Documentation updated
- [ ] Iteration can be marked complete
```

### 🎯 **Continuous Improvement**

This section should be updated with each new lesson learned to ensure continuous improvement of our development process.

**Remember**: The goal is not to avoid mistakes, but to learn from them and improve our processes.

## Conclusion

This architecture provides a solid foundation for a robust, intelligent media organization system. By combining external authoritative data with local learning capabilities, the system can handle the complex and varied naming conventions found in real-world media collections while continuously improving its accuracy over time.

### **Key Architectural Benefits**
- **Single Binary Design**: Simplifies deployment, distribution, and maintenance
- **User-Provided APIs**: Eliminates central API management and associated costs
- **Unified SQLite Storage**: Provides portability, reliability, and performance
- **Plex Convention Compliance**: Ensures optimal compatibility with media servers

The modular design allows for incremental implementation, starting with basic functionality and gradually adding more sophisticated features. The focus on learning and adaptation ensures that the system becomes more effective as it processes more media files and receives user feedback.
