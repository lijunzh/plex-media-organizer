# Config Optimization Plan

## Overview
This document outlines the strategy and implementation plan for optimizing the configuration system in the Plex Media Organizer.

## Analysis of Current Issues

### Problem 1: Performance Issues ✅ COMPLETED
- **Issue**: Multiple `AppConfig::load()` calls across different components
- **Impact**: Significant performance degradation (35+ seconds for 417 files)
- **Root Cause**: Each component loads config independently, causing redundant I/O operations

### Problem 2: Data Structure Issues ✅ COMPLETED
- **Issue**: Overly large and irrelevant data in config files
- **Impact**: Unnecessary memory usage and processing overhead
- **Examples**: 
  - `common_words` list contained 500+ entries including entire periodic table
  - Irrelevant technical terms mixed with essential movie title words

### Problem 3: Duplication Issues 🔄 PARTIALLY COMPLETED
- **Issue**: Same technical terms defined in multiple places
- **Impact**: Inconsistency and maintenance overhead
- **Examples**:
  - Hardcoded terms in Rust code that also exist in config
  - Same terms in different config sections

### Problem 4: Poor Organization 🔄 PENDING
- **Issue**: Config sections not logically organized
- **Impact**: Difficult to understand and maintain
- **Examples**:
  - Unrelated terms mixed together
  - No clear separation of concerns

### Problem 5: Limited Configurability 🔄 PARTIALLY COMPLETED
- **Issue**: Hardcoded filtering logic not configurable
- **Impact**: Users cannot customize behavior without code changes

## Strategy: Single Load + Parameter Passing

### Approach 1: Single Config Load (Selected)
- **Method**: Load `AppConfig` once at CLI level, pass down to components
- **Benefits**: 
  - Eliminates redundant I/O operations
  - Ensures consistency across components
  - Simple and maintainable
- **Implementation**: Pass `&AppConfig` or specific slices to component methods

### Approach 2: Config Caching (Rejected)
- **Method**: Cache `AppConfig` instances in static variables
- **Issues**: 
  - Complex lifetime management
  - Potential inconsistencies
  - Difficult to test and debug

## Implementation Plan

### Phase 1: Config Optimizations ✅ COMPLETED
**Status**: COMPLETED
**Duration**: 1 session

#### Problem 2: Data Structure Issues ✅ COMPLETED
- [x] Audit `config/default.toml` content
- [x] Reduce `common_words` list from 500+ to ~100 essential words
- [x] Remove irrelevant entries (periodic table, etc.)
- [x] Add essential movie title words ("Star", "Wars", "Episode")
- [x] **Result**: Config file size reduced from 16KB to 12KB
- [x] **Performance Impact**: Unexpected 60x improvement (35s → 560ms)

#### Problem 3: Duplication Issues 🔄 PARTIALLY COMPLETED
- [x] Remove "PiRaTeS" from hardcoded `known_groups` in `src/filename_parser.rs`
- [x] Remove "Star" from hardcoded `source_platform_terms`
- [x] Remove other hardcoded technical terms from Rust code
- [x] **Remaining**: Move remaining hardcoded terms to config

#### Problem 5: Basic Configurability 🔄 PARTIALLY COMPLETED
- [x] Improve filtering logic to be more configurable
- [x] Simplify approach for iteration 1
- [x] **Remaining**: Remove remaining hardcoded arrays

### Phase 2: Single Load Implementation ✅ COMPLETED
**Status**: COMPLETED
**Duration**: 1 session

#### Problem 1: Performance Issues ✅ COMPLETED
- [x] Identify all `AppConfig::load()` calls across components
- [x] Refactor `MovieParser` to accept `&AppConfig` in constructors
- [x] Add `_with_config` constructors to `Scanner`
- [x] Add `_with_config` methods to `TmdbClient`
- [x] Update `src/cli.rs` to load config once and pass down
- [x] Maintain backward compatibility with original methods
- [x] **Result**: Single config load per command execution
- [x] **Performance**: Maintained 560ms for 417 files

### Phase 3: Code Quality Improvements ✅ COMPLETED
**Status**: COMPLETED
**Duration**: 1 session

#### Warning Fixes ✅ COMPLETED
- [x] Fix unused variable `known_titles` in `src/filename_parser.rs`
- [x] Remove dead code: unused methods `extract_language()`, `extract_title_and_original()`, `is_japanese_title_token()`, `is_metadata_token()`
- [x] **Result**: Clean builds with no warnings
- [x] **Benefits**: Cleaner codebase, smaller binary size, better maintainability

### Phase 3: Eliminate Duplication 🔄 PENDING
**Status**: PENDING
**Duration**: 1 session

#### Problem 3: Duplication Issues (Remaining Work)
- [ ] Move remaining hardcoded terms from Rust code to config
- [ ] Ensure all terms are defined in one single source of truth
- [ ] Update parser methods to use config values instead of hardcoded arrays
- [ ] Eliminate same terms defined in multiple places

### Phase 4: Basic Reorganization 🔄 PENDING
**Status**: PENDING
**Duration**: 1 session

#### Problem 4: Poor Organization
- [ ] Logically reorganize config sections
- [ ] Create new `[organization.title_preservation]` section
- [ ] Separate concerns: distinguish between filtering and preservation logic
- [ ] Improve readability through better organization and documentation

### Phase 5: Enhanced Configurability 🔄 PENDING
**Status**: PENDING
**Duration**: 1 session

#### Problem 5: Limited Configurability (Remaining Work)
- [ ] Remove remaining hardcoded arrays from Rust code
- [ ] Allow users to add custom technical terms and title words to config
- [ ] Prepare for database-driven learning system

## Success Metrics

### Performance Metrics ✅ ACHIEVED
- **Before**: 35+ seconds for 417 files
- **After**: ~560ms for 417 files
- **Improvement**: 60x faster (98.4% reduction)

### Code Quality Metrics ✅ ACHIEVED
- **Config Optimization**: 60% complete
- **Performance Improvement**: 100% complete
- **Code Cleanup**: 50% complete
- **Test Coverage**: 100% maintained
- **Warning Elimination**: 100% complete

### Maintainability Metrics
- **Single Source of Truth**: 30% complete
- **Config Organization**: 0% complete
- **User Configurability**: 20% complete

## Future Vision: Database-Driven Learning

### Long-term Strategy
- **Replace config files** with database-driven keyword learning
- **Adaptive system** that learns from real-world data
- **Machine learning** for pattern recognition
- **User feedback** integration for continuous improvement

### Benefits
- **Dynamic adaptation** to new patterns
- **Reduced maintenance** overhead
- **Better accuracy** through learning
- **Scalable** to large collections

## Implementation Notes

### Backward Compatibility
- All original constructors and methods maintained
- New `_with_config` methods added alongside existing ones
- No breaking changes to public API

### Testing Strategy
- All existing tests continue to pass
- New tests added for config-aware methods
- Performance tests verify improvements

### Error Handling
- Graceful fallback to default values when config is missing
- Clear error messages for configuration issues
- Robust handling of malformed config files

## Next Steps

1. **Complete Phase 3**: Eliminate remaining duplication
2. **Complete Phase 4**: Reorganize config structure
3. **Complete Phase 5**: Enhance user configurability
4. **Prepare for database integration**: Design schema for keyword learning
5. **Performance monitoring**: Track real-world performance metrics
