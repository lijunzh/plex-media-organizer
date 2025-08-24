# Plex Media Organizer - Refactoring Strategy

## Executive Summary

This document outlines a comprehensive refactoring strategy for the Plex Media Organizer codebase to address code quality issues, reduce duplication, eliminate dead code, and improve maintainability through better structure and organization.

## Current State Analysis

### File Size Issues
```
📁 src/ (Total: ~15,000 lines)
├── 🚨 filename_parser.rs (1,800 lines) - CRITICAL: Too large, multiple responsibilities
├── 🚨 scanner.rs (955 lines) - LARGE: File discovery + processing orchestration
├── 🚨 config.rs (805 lines) - LARGE: All configuration in one file
├── 🚨 organizer.rs (700 lines) - LARGE: File operations + metadata extraction
├── 🚨 movie_parser.rs (485 lines) - MEDIUM: Thin wrapper around filename_parser
├── 🚨 metadata_extractor.rs (363 lines) - MEDIUM: Mixed responsibilities
├── ✅ types.rs (272 lines) - GOOD: Well-structured data types
├── ✅ lib.rs (234 lines) - GOOD: Clean library exports
├── ✅ main.rs (14 lines) - GOOD: Simple entry point
└── 📁 Modular directories (GOOD structure)
    ├── 📁 cli/ (1,306 lines → well-organized)
    ├── 📁 external/ (883 lines → focused)
    └── 📁 database/ (1,241 lines → well-organized)
```

### Critical Issues Identified

#### 1. **Monolithic filename_parser.rs (1,800 lines)**
- **Problems**: 
  - Single file handling all parsing logic
  - Mixed pattern detection, title extraction, and technical term filtering
  - Hard to test individual components
  - Difficult to extend with new patterns
- **Impact**: High maintenance burden, low testability

#### 2. **Artificial Separation Between Parsers**
- **Problems**:
  - `movie_parser.rs` is a thin wrapper around `filename_parser.rs`
  - Unnecessary data transformation between `FilenameComponents` and `MovieInfo`
  - Duplicate parsing logic
- **Impact**: Code duplication, confusing architecture

#### 3. **Mixed Responsibilities in Large Files**
- **Problems**:
  - `scanner.rs`: File discovery + processing orchestration
  - `organizer.rs`: File operations + metadata extraction
  - `config.rs`: All configuration types in one file
- **Impact**: Violates single responsibility principle

#### 4. **Dead Code and Unused Imports**
- **Problems**:
  - Unused `year_patterns` field in `FilenameParser`
  - Dead code in various modules
  - Unused imports across files
- **Impact**: Code bloat, maintenance overhead

## Proposed Refactoring Strategy

### Phase 1: Parser Unification and Modularization

#### 1.1 Create Unified Parser Architecture
```
📁 src/parsers/
├── 📄 mod.rs - Public API and re-exports
├── 📄 movie.rs - Main movie parsing orchestration
├── 📁 patterns/ - Pattern detection modules
│   ├── 📄 mod.rs - Pattern detection API
│   ├── 📄 technical.rs - Quality, source, audio, codec patterns
│   ├── 📄 series.rs - Series and collection detection
│   ├── 📄 anime.rs - Anime and CJK title detection
│   └── 📄 language.rs - Language detection and processing
├── 📁 extraction/ - Title extraction logic
│   ├── 📄 mod.rs - Extraction API
│   ├── 📄 title.rs - Main title extraction
│   ├── 📄 technical_terms.rs - Technical term filtering
│   └── 📄 cjk.rs - CJK title processing
└── 📄 types.rs - Parser-specific types
```

**Benefits**:
- Clear separation of concerns
- Testable individual components
- Extensible pattern detection
- Reduced file sizes

#### 1.2 Eliminate Parser Duplication
- Merge `movie_parser.rs` and `filename_parser.rs` into unified `parsers/movie.rs`
- Remove artificial data transformations
- Create single parsing pipeline

### Phase 2: Configuration Modularization

#### 2.1 Split Configuration by Domain
```
📁 src/config/
├── 📄 mod.rs - Main configuration API
├── 📄 app.rs - Main AppConfig and loading logic
├── 📄 api.rs - API configuration (TMDB, etc.)
├── 📄 database.rs - Database configuration
├── 📄 organization.rs - Organization preferences
├── 📄 quality.rs - Quality preferences
├── 📄 language.rs - Language configuration
├── 📄 technical_terms.rs - Technical terms configuration
└── 📄 content_filtering.rs - Content filtering configuration
```

**Benefits**:
- Smaller, focused files
- Better organization by domain
- Easier to maintain and extend

### Phase 3: Scanner and Organizer Separation

#### 3.1 Refactor Scanner
```
📁 src/scanner/
├── 📄 mod.rs - Scanner API
├── 📄 discovery.rs - File discovery logic
├── 📄 processing.rs - Processing orchestration
├── 📄 network.rs - Network drive optimizations
└── 📄 types.rs - Scanner-specific types
```

#### 3.2 Refactor Organizer
```
📁 src/organizer/
├── 📄 mod.rs - Organizer API
├── 📄 operations.rs - File operations
├── 📄 naming.rs - Plex naming conventions
├── 📄 rollback.rs - Rollback functionality
└── 📄 types.rs - Organizer-specific types
```

### Phase 4: CLI Handler Extraction

#### 4.1 Complete CLI Modularization
```
📁 src/cli/
├── 📄 mod.rs - CLI API
├── 📄 commands.rs - Command definitions
├── 📄 output.rs - Output formatting
├── 📁 handlers/ - Command handlers
│   ├── 📄 mod.rs - Handler API
│   ├── 📄 scan.rs - Scan command handler
│   ├── 📄 organize.rs - Organize command handler
│   ├── 📄 setup.rs - Setup command handler
│   ├── 📄 test.rs - Test command handler
│   ├── 📄 rollback.rs - Rollback command handler
│   └── 📄 cleanup.rs - Cleanup command handler
└── 📄 types.rs - CLI-specific types
```

### Phase 5: Dead Code Elimination

#### 5.1 Remove Unused Code
- Remove unused `year_patterns` field
- Clean up unused imports
- Remove dead code paths
- Eliminate duplicate functionality

#### 5.2 Optimize Imports
- Consolidate common imports
- Remove unused dependencies
- Use consistent import patterns

## Implementation Plan

### Phase 1: Parser Refactoring (Priority: HIGH)
**Duration**: 2-3 days
**Steps**:
1. Create new parser module structure
2. Extract pattern detection logic
3. Create unified movie parser
4. Update imports and tests
5. Remove old parser files

### Phase 2: Configuration Refactoring (Priority: HIGH)
**Duration**: 1-2 days
**Steps**:
1. Split configuration files by domain
2. Update configuration loading
3. Maintain backward compatibility
4. Update tests

### Phase 3: Scanner/Organizer Refactoring (Priority: MEDIUM)
**Duration**: 2-3 days
**Steps**:
1. Extract file discovery logic
2. Separate processing orchestration
3. Create network optimizations module
4. Split organizer responsibilities

### Phase 4: CLI Completion (Priority: MEDIUM)
**Duration**: 1-2 days
**Steps**:
1. Extract remaining command handlers
2. Create handler-specific modules
3. Update CLI structure
4. Maintain API compatibility

### Phase 5: Cleanup (Priority: LOW)
**Duration**: 1 day
**Steps**:
1. Remove dead code
2. Clean up imports
3. Update documentation
4. Final testing

## Success Metrics

### Code Quality Improvements
- **File Size Reduction**: Target <500 lines per file
- **Cyclomatic Complexity**: Reduce by 30%
- **Code Duplication**: Eliminate 90% of duplication
- **Test Coverage**: Maintain or improve current coverage

### Maintainability Improvements
- **Single Responsibility**: Each module has one clear purpose
- **Dependency Direction**: Clear dependency flow
- **Extensibility**: Easy to add new patterns/features
- **Documentation**: Comprehensive module documentation

### Performance Impact
- **Zero Performance Regression**: Maintain current performance
- **Memory Usage**: Reduce memory footprint
- **Compilation Time**: Faster incremental compilation

## Risk Mitigation

### Backward Compatibility
- Maintain public API compatibility
- Use feature flags for breaking changes
- Provide migration guides

### Testing Strategy
- Comprehensive unit tests for each module
- Integration tests for parsing pipeline
- Performance regression tests
- Backward compatibility tests

### Rollback Plan
- Each phase is independently reversible
- Git branches for each phase
- Tagged releases for stable points

## Conclusion

This refactoring strategy addresses the critical issues in the current codebase while maintaining functionality and improving maintainability. The phased approach ensures minimal disruption while achieving significant improvements in code quality and structure.

The proposed changes will result in:
- **Better maintainability** through smaller, focused modules
- **Improved testability** through clear separation of concerns
- **Enhanced extensibility** through modular architecture
- **Reduced technical debt** through elimination of duplication and dead code

**Next Steps**: Review this strategy and approve for implementation, then begin with Phase 1 (Parser Refactoring).
