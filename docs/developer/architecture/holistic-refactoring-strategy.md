# Holistic Refactoring Strategy

## Overview

This document outlines the comprehensive refactoring strategy to transform the Plex Media Organizer codebase from a collection of large, monolithic files into a well-organized, modular architecture that's easier to understand, maintain, and extend.

## Current Architecture Analysis

### Module Structure & Sizes
```
📁 src/ (9,049 total lines)
├── 🎯 main.rs (13 lines) - Entry point
├── 📄 lib.rs (232 lines) - Library exports
├── 📄 types.rs (271 lines) - Shared data structures
├── 📄 config.rs (804 lines) - Configuration management
├── 📄 cli.rs (1,306 lines) - Command-line interface
├── 📄 scanner.rs (954 lines) - File discovery and scanning
├── 🎯 filename_parser.rs (1,799 lines) - Filename parsing logic
├── 📄 movie_parser.rs (484 lines) - Movie parsing orchestration
├── 📄 tmdb_client.rs (883 lines) - TMDB API integration
├── 📄 organizer.rs (699 lines) - File organization logic
├── 📄 metadata_extractor.rs (363 lines) - Media metadata extraction
└── 📁 database/ (1,241 lines) - Database layer
    ├── 📄 mod.rs (79 lines)
    ├── 📄 schema.rs (152 lines)
    ├── 📄 connection.rs (303 lines)
    ├── 📄 cache.rs (341 lines)
    └── 📄 movies.rs (367 lines)
```

### Current Dependencies
```
cli.rs → config, movie_parser, scanner, tmdb_client
scanner.rs → config, movie_parser, types
movie_parser.rs → config, filename_parser, tmdb_client, types
organizer.rs → types, metadata_extractor
tmdb_client.rs → types
metadata_extractor.rs → types
database/*.rs → internal dependencies only
```

## Problems with Current Architecture

### 1. Large Monolithic Files
- **filename_parser.rs** (1,799 lines) - Too large, multiple responsibilities
- **cli.rs** (1,306 lines) - All command handlers in one file
- **tmdb_client.rs** (883 lines) - API client + search logic mixed

### 2. Artificial Separation
- **movie_parser.rs** is a thin wrapper around **filename_parser.rs**
- Unnecessary data transformation between `FilenameComponents` and `MovieInfo`

### 3. Mixed Responsibilities
- **scanner.rs** handles both file discovery and processing orchestration
- **organizer.rs** mixes file operations with metadata extraction
- **tmdb_client.rs** combines API client with search algorithms

### 4. Inconsistent Module Organization
- Some modules are well-organized (`database/`)
- Others are monolithic (`cli.rs`, `filename_parser.rs`)

## Proposed Architecture

### Target Structure
```
📁 src/
├── 🎯 main.rs (13 lines) - Entry point
├── 📄 lib.rs (232 lines) - Library exports
├── 📄 types.rs (271 lines) - Shared data structures
├── 📄 config.rs (804 lines) - Configuration management
├── 📁 cli/ (1,306 lines → modular)
│   ├── 📄 mod.rs
│   ├── 📄 commands.rs
│   ├── 📄 handlers/
│   │   ├── 📄 mod.rs
│   │   ├── 📄 scan.rs
│   │   ├── 📄 setup.rs
│   │   ├── 📄 config.rs
│   │   ├── 📄 test.rs
│   │   ├── 📄 organize.rs
│   │   ├── 📄 rollback.rs
│   │   └── 📄 cleanup.rs
│   └── 📄 output.rs
├── 📁 parsers/ (2,283 lines → unified)
│   ├── 📄 mod.rs
│   ├── 📄 movie.rs (merged movie_parser + filename_parser)
│   ├── 📄 patterns/
│   │   ├── 📄 mod.rs
│   │   ├── 📄 series.rs
│   │   ├── 📄 anime.rs
│   │   └── 📄 technical.rs
│   └── 📄 types.rs
├── 📁 external/ (883 lines → focused)
│   ├── 📄 mod.rs
│   ├── 📄 tmdb/
│   │   ├── 📄 mod.rs
│   │   ├── 📄 client.rs
│   │   └── 📄 search.rs
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

## Module Responsibilities

### CLI Module (`src/cli/`)
**Purpose**: User interface and command handling
**Responsibilities**:
- Command parsing and argument handling
- User interaction and progress reporting
- Error presentation and user feedback
- Command routing and execution

**Benefits**:
- Modular command handlers
- Easier testing of individual commands
- Better maintainability and readability

### Parsers Module (`src/parsers/`)
**Purpose**: Unified parsing logic for all media types
**Responsibilities**:
- Filename parsing and component extraction
- Pattern detection (series, anime, technical terms)
- Data transformation and validation
- Parsing strategy management

**Benefits**:
- Single parsing responsibility
- No artificial separation between parsers
- Better performance with reduced data transformations
- Easier to extend for new media types

### External Module (`src/external/`)
**Purpose**: External API integrations
**Responsibilities**:
- TMDB API client and authentication
- Search algorithms and result processing
- Rate limiting and caching
- Error handling for external services

**Benefits**:
- Focused API handling
- Easier to add new external services
- Better separation of external dependencies
- Improved testability with mock APIs

### Core Module (`src/core/`)
**Purpose**: Core business logic orchestration
**Responsibilities**:
- File discovery and directory scanning
- Processing coordination and workflow management
- File organization and renaming
- Error handling and recovery

**Benefits**:
- Clear separation of concerns
- Better testability
- Improved error handling
- Easier to modify business logic

### Media Module (`src/media/`)
**Purpose**: Media file handling
**Responsibilities**:
- Metadata extraction from media files
- File analysis and content detection
- Media type identification
- File format support

**Benefits**:
- Focused media handling
- Easier to extend for new media types
- Better separation from parsing logic
- Improved performance with specialized handling

### Database Module (`src/database/`)
**Purpose**: Data persistence (keep as-is)
**Responsibilities**:
- Data storage and retrieval
- Caching and performance optimization
- Schema management and migrations
- Connection pooling and management

**Benefits**:
- Already well-organized
- Minimal changes needed
- Clear separation of data concerns

## Implementation Strategy

### Phase 1: Core Architecture Restructuring

#### Phase 1A: CLI Refactoring (Week 1)
**Goals**:
- Extract command handlers to separate modules
- Create output utilities for progress reporting
- Update imports and module declarations

**Files to Create**:
- `src/cli/mod.rs`
- `src/cli/commands.rs`
- `src/cli/handlers/mod.rs`
- `src/cli/handlers/scan.rs`
- `src/cli/handlers/setup.rs`
- `src/cli/handlers/config.rs`
- `src/cli/handlers/test.rs`
- `src/cli/handlers/organize.rs`
- `src/cli/handlers/rollback.rs`
- `src/cli/handlers/cleanup.rs`
- `src/cli/output.rs`

**Files to Remove**:
- `src/cli.rs`

#### Phase 1B: Parser Unification (Week 2)
**Goals**:
- Merge `movie_parser.rs` and `filename_parser.rs`
- Extract pattern detection to separate modules
- Update all parser-related imports

**Files to Create**:
- `src/parsers/mod.rs`
- `src/parsers/movie.rs`
- `src/parsers/patterns/mod.rs`
- `src/parsers/patterns/series.rs`
- `src/parsers/patterns/anime.rs`
- `src/parsers/patterns/technical.rs`
- `src/parsers/types.rs`

**Files to Remove**:
- `src/movie_parser.rs`
- `src/filename_parser.rs`

#### Phase 1C: External API Restructuring (Week 3)
**Goals**:
- Split `tmdb_client.rs` into focused modules
- Create external API abstraction layer
- Update TMDB-related imports

**Files to Create**:
- `src/external/mod.rs`
- `src/external/tmdb/mod.rs`
- `src/external/tmdb/client.rs`
- `src/external/tmdb/search.rs`
- `src/external/types.rs`

**Files to Remove**:
- `src/tmdb_client.rs`

#### Phase 1D: Core Logic Separation (Week 4)
**Goals**:
- Refactor `scanner.rs` and `organizer.rs`
- Create processing orchestration layer
- Update core module imports

**Files to Create**:
- `src/core/mod.rs`
- `src/core/scanner.rs`
- `src/core/processor.rs`
- `src/core/organizer.rs`

**Files to Modify**:
- `src/scanner.rs` → `src/core/scanner.rs`
- `src/organizer.rs` → `src/core/organizer.rs`

### Phase 2: Testing & Validation (Week 5)
**Goals**:
- Update all tests to work with new structure
- Validate functionality remains intact
- Performance testing and optimization

**Activities**:
- Update test imports and module references
- Run comprehensive test suite
- Performance benchmarking
- Integration testing

## Migration Strategy

### Gradual Migration Approach
1. **Create new modules** alongside existing ones
2. **Move functionality** piece by piece
3. **Update imports** gradually
4. **Remove old modules** once migration is complete

### Backward Compatibility
- **Maintain public API**: All existing public interfaces remain unchanged
- **Gradual deprecation**: Mark old modules as deprecated during transition
- **Documentation updates**: Update all documentation to reflect new structure

### Testing Strategy
- **Unit tests**: Test each new module independently
- **Integration tests**: Ensure modules work together correctly
- **Regression tests**: Verify existing functionality is preserved
- **Performance tests**: Ensure no performance degradation

## Expected Outcomes

### Code Quality Improvements
- **Reduced complexity**: Smaller, focused modules
- **Better organization**: Clear separation of concerns
- **Improved readability**: Easier to understand and navigate
- **Enhanced maintainability**: Easier to modify and extend

### Development Efficiency
- **Faster development**: Easier to find and modify specific functionality
- **Better collaboration**: Multiple developers can work on different modules
- **Reduced merge conflicts**: Smaller files mean fewer conflicts
- **Improved debugging**: Clearer error boundaries and debugging paths

### Future Maintainability
- **Easier extension**: Clear extension points for new features
- **Better testing**: More focused and comprehensive testing
- **Simpler onboarding**: New developers can understand the codebase faster
- **Reduced technical debt**: Cleaner architecture reduces future maintenance burden

## Success Metrics

### Quantitative Metrics
- **File size reduction**: Average file size < 500 lines
- **Module count increase**: From 12 to ~25 focused modules
- **Test coverage**: Maintain or improve current test coverage
- **Performance**: No degradation in processing speed

### Qualitative Metrics
- **Developer satisfaction**: Easier to work with the codebase
- **Bug reduction**: Fewer bugs due to clearer code organization
- **Feature delivery**: Faster delivery of new features
- **Code review efficiency**: Easier and faster code reviews

## Risk Mitigation

### Technical Risks
- **Breaking changes**: Mitigated by maintaining public API
- **Performance degradation**: Mitigated by comprehensive testing
- **Test failures**: Mitigated by gradual migration and thorough testing

### Process Risks
- **Scope creep**: Mitigated by clear phase boundaries
- **Timeline delays**: Mitigated by incremental approach
- **Team coordination**: Mitigated by clear documentation and communication

## Conclusion

This holistic refactoring strategy will transform the Plex Media Organizer codebase from a collection of large, monolithic files into a well-organized, modular architecture that's easier to understand, maintain, and extend. The phased approach ensures minimal disruption while achieving significant improvements in code quality and developer productivity.

The new architecture will provide a solid foundation for future development, making it easier to add new features, maintain existing functionality, and onboard new team members.
