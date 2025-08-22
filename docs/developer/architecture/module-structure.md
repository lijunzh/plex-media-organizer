# Module Structure Documentation

## Overview

This document outlines the intended module structure for the Plex Media Organizer after the holistic refactoring is complete. It serves as a reference for understanding the new architecture and module organization.

## Target Module Structure

### Root Level Modules
```
📁 src/
├── 🎯 main.rs (13 lines) - Entry point
├── 📄 lib.rs (232 lines) - Library exports
├── 📄 types.rs (271 lines) - Shared data structures
├── 📄 config.rs (804 lines) - Configuration management
├── 📁 cli/ (1,306 lines → modular)
├── 📁 parsers/ (2,283 lines → unified)
├── 📁 external/ (883 lines → focused)
├── 📁 core/ (1,653 lines → orchestration)
├── 📁 media/ (363 lines → focused)
└── 📁 database/ (1,241 lines → keep as-is)
```

## Detailed Module Breakdown

### CLI Module (`src/cli/`)
**Purpose**: User interface and command handling
**Structure**:
```
📁 cli/
├── 📄 mod.rs              # Main CLI module exports
├── 📄 commands.rs         # Command definitions and main CLI struct
├── 📁 handlers/           # Command handlers
│   ├── 📄 mod.rs
│   ├── 📄 scan.rs         # Scan command handler
│   ├── 📄 setup.rs        # Setup command handler
│   ├── 📄 config.rs       # Config command handler
│   ├── 📄 test.rs         # Test command handler
│   ├── 📄 organize.rs     # Organize command handler
│   ├── 📄 rollback.rs     # Rollback command handler
│   └── 📄 cleanup.rs      # Cleanup command handler
└── 📄 output.rs           # Progress reporting and output formatting
```

**Responsibilities**:
- Command parsing and argument handling
- User interaction and progress reporting
- Error presentation and user feedback
- Command routing and execution

### Parsers Module (`src/parsers/`)
**Purpose**: Unified parsing logic for all media types
**Structure**:
```
📁 parsers/
├── 📄 mod.rs              # Main parsers module exports
├── 📄 movie.rs            # Unified movie parser (merged movie_parser + filename_parser)
├── 📁 patterns/           # Pattern detection modules
│   ├── 📄 mod.rs
│   ├── 📄 series.rs       # Series pattern detection
│   ├── 📄 anime.rs        # Anime pattern detection
│   └── 📄 technical.rs    # Technical terms filtering
└── 📄 types.rs            # Shared parser data structures
```

**Responsibilities**:
- Filename parsing and component extraction
- Pattern detection (series, anime, technical terms)
- Data transformation and validation
- Parsing strategy management

### External Module (`src/external/`)
**Purpose**: External API integrations
**Structure**:
```
📁 external/
├── 📄 mod.rs              # Main external module exports
├── 📁 tmdb/               # TMDB API integration
│   ├── 📄 mod.rs
│   ├── 📄 client.rs       # TMDB API client
│   └── 📄 search.rs       # Search algorithms and result processing
└── 📄 types.rs            # External API data structures
```

**Responsibilities**:
- TMDB API client and authentication
- Search algorithms and result processing
- Rate limiting and caching
- Error handling for external services

### Core Module (`src/core/`)
**Purpose**: Core business logic orchestration
**Structure**:
```
📁 core/
├── 📄 mod.rs              # Main core module exports
├── 📄 scanner.rs          # File discovery and directory scanning
├── 📄 processor.rs        # Processing coordination and workflow management
└── 📄 organizer.rs        # File organization and renaming
```

**Responsibilities**:
- File discovery and directory scanning
- Processing coordination and workflow management
- File organization and renaming
- Error handling and recovery

### Media Module (`src/media/`)
**Purpose**: Media file handling
**Structure**:
```
📁 media/
├── 📄 mod.rs              # Main media module exports
├── 📄 extractor.rs        # Metadata extraction from media files
└── 📄 types.rs            # Media-specific data structures
```

**Responsibilities**:
- Metadata extraction from media files
- File analysis and content detection
- Media type identification
- File format support

### Database Module (`src/database/`)
**Purpose**: Data persistence (keep as-is)
**Structure**:
```
📁 database/
├── 📄 mod.rs              # Database module exports
├── 📄 schema.rs           # Database schema and migrations
├── 📄 connection.rs       # Connection pooling and management
├── 📄 cache.rs            # Caching and performance optimization
└── 📄 movies.rs           # Movie data storage and retrieval
```

**Responsibilities**:
- Data storage and retrieval
- Caching and performance optimization
- Schema management and migrations
- Connection pooling and management

## Module Dependencies

### Dependency Flow
```
cli/ → core/, parsers/, external/, config
core/ → parsers/, external/, media/, database/
parsers/ → external/, types
external/ → types
media/ → types
database/ → types
```

### Import Structure
```rust
// CLI module imports
use crate::core::{Scanner, Processor, Organizer};
use crate::parsers::MovieParser;
use crate::external::tmdb::TmdbClient;
use crate::config::AppConfig;

// Core module imports
use crate::parsers::MovieParser;
use crate::external::tmdb::TmdbClient;
use crate::media::MetadataExtractor;
use crate::database::DatabaseManager;

// Parser module imports
use crate::external::tmdb::TmdbClient;
use crate::types::{MovieInfo, ParsingResult};

// External module imports
use crate::types::{TmdbMovie, TmdbSearchResult};

// Media module imports
use crate::types::{MediaFile, MediaMetadata};
```

## Migration Path

### Phase 1A: CLI Refactoring
**Current**: `src/cli.rs` (1,306 lines)
**Target**: `src/cli/` module with 11 focused files
**Migration**:
1. Create `src/cli/` directory structure
2. Extract command handlers to individual files
3. Create output utilities
4. Update imports and module declarations
5. Remove `src/cli.rs`

### Phase 1B: Parser Unification
**Current**: `src/movie_parser.rs` (484 lines) + `src/filename_parser.rs` (1,799 lines)
**Target**: `src/parsers/` module with unified movie parser
**Migration**:
1. Create `src/parsers/` directory structure
2. Merge movie_parser.rs and filename_parser.rs into movie.rs
3. Extract pattern detection to separate modules
4. Update all parser-related imports
5. Remove old parser files

### Phase 1C: External API Restructuring
**Current**: `src/tmdb_client.rs` (883 lines)
**Target**: `src/external/tmdb/` module with focused components
**Migration**:
1. Create `src/external/` directory structure
2. Split tmdb_client.rs into client.rs and search.rs
3. Create external API abstraction layer
4. Update TMDB-related imports
5. Remove `src/tmdb_client.rs`

### Phase 1D: Core Logic Separation
**Current**: `src/scanner.rs` (954 lines) + `src/organizer.rs` (699 lines)
**Target**: `src/core/` module with separated concerns
**Migration**:
1. Create `src/core/` directory structure
2. Refactor scanner.rs to focus on file discovery
3. Create processor.rs for orchestration
4. Refactor organizer.rs to focus on file operations
5. Update core module imports

## Benefits of New Structure

### Code Organization
- **Clear separation of concerns**: Each module has a single, well-defined responsibility
- **Logical grouping**: Related functionality is grouped together
- **Easier navigation**: Developers can quickly find specific functionality
- **Reduced complexity**: Smaller, focused files are easier to understand

### Maintainability
- **Easier modification**: Changes are isolated to specific modules
- **Better testing**: Each module can be tested independently
- **Reduced coupling**: Clear dependency boundaries
- **Improved debugging**: Clearer error boundaries

### Extensibility
- **New parsers**: Easy to add new parsing strategies in parsers module
- **New external APIs**: Simple to add new external services in external module
- **New media types**: Clear extension points in media module
- **New commands**: Easy to add new CLI commands in cli module

### Performance
- **Reduced overhead**: No unnecessary data transformations
- **Focused caching**: Each module can implement appropriate caching
- **Optimized imports**: Clear dependency boundaries reduce compilation overhead

## Documentation Alignment

### User Documentation (`docs/user/`)
- **No changes needed**: User-facing functionality remains the same
- **CLI commands**: All existing commands continue to work
- **Configuration**: Configuration format remains unchanged
- **Examples**: All examples remain valid

### Developer Documentation (`docs/developer/`)
- **Architecture docs**: Updated to reflect new module structure
- **API documentation**: Updated to show new module organization
- **Development guides**: Updated to reflect new development patterns
- **Testing guides**: Updated to show new testing approaches

### Analysis Documentation (`docs/analysis/`)
- **Current issues**: Updated to reflect new module structure
- **Performance analysis**: Updated to show new performance characteristics
- **Limitations**: Updated to reflect new architecture limitations

## Conclusion

This new module structure provides a solid foundation for the Plex Media Organizer's future development. It addresses the current architectural issues while maintaining backward compatibility and improving developer productivity. The phased migration approach ensures minimal disruption while achieving significant improvements in code organization and maintainability.
