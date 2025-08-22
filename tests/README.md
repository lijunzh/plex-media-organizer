# Tests Directory

This directory contains the comprehensive test suite for the Plex Media Organizer, following Rust best practices.

## 📁 **Test Structure**

### **Unit Tests** (Co-located with source code)
- **`src/filename_parser.rs`** - Unit tests for `FilenameParser` (using `#[cfg(test)]` module)
- **`src/movie_parser.rs`** - Unit tests for `MovieParser` (to be added)
- **`src/scanner.rs`** - Unit tests for `Scanner` (to be added)
- **`src/organizer.rs`** - Unit tests for `Organizer` (to be added)
- **`src/tmdb_client.rs`** - Unit tests for `TmdbClient` (to be added)

### **Integration Tests** (`tests/` directory)
- **`tests/filename_parsing_integration.rs`** - Comprehensive filename parsing tests
- **`tests/real_world_patterns.rs`** - Real-world pattern validation
- **`tests/parsing_regression.rs`** - Previously failing cases to prevent regressions
- **`tests/edge_case_scanner.rs`** - Comprehensive edge case detection
- **`tests/plex_organization.rs`** - Plex directory structure validation
- **`tests/rollback_integration.rs`** - Rollback functionality testing
- **`tests/dynamic_real_world.rs`** - Dynamic tests using actual tree files

### **Test Utilities**
- **`tests/test_utils.rs`** - Shared test utilities and helpers
- **`tests/test_data/`** - Test data files

## 🎯 **Test Philosophy**

### **Unit Tests (Co-located)**
- **Fast execution** for development feedback
- **Specific validation** of individual components
- **Edge case coverage** for robustness
- **Regression prevention** for changes
- **Follow Rust conventions** with `#[cfg(test)]` modules

### **Integration Tests**
- Test component interactions
- Use realistic test data
- Include performance measurements
- Test end-to-end workflows

### **Regression Tests**
- **Previously failing cases** to prevent regressions
- **Specific bug fixes** validation
- **Edge case preservation** for known issues

### **Specialized Tests**
- ✅ Edge case scanning (real directory analysis)
- ✅ Plex organization structure validation
- ✅ Rollback functionality testing
- ✅ Dynamic real-world data testing

## 🚀 **Running Tests**

### **All Tests**
```bash
cargo test
```

### **Unit Tests Only**
```bash
cargo test --lib
```

### **Integration Tests Only**
```bash
cargo test --test
```

### **Specific Test Categories**
```bash
# Integration tests
cargo test --test filename_parsing_integration
cargo test --test real_world_patterns
cargo test --test parsing_regression
cargo test --test edge_case_scanner
cargo test --test plex_organization
cargo test --test rollback_integration
cargo test --test dynamic_real_world

# Edge case scanning (requires actual media directory)
cargo test --test edge_case_scanner -- --nocapture
```

### **With Output**
```bash
cargo test -- --nocapture
```

## 📊 **Test Coverage**

### **Unit Tests**
- ✅ `FilenameParser` - Basic parsing, edge cases, Unicode handling
- ✅ `FilenameParser` - Chinese-English bilingual patterns
- ✅ `FilenameParser` - Technical terms filtering
- ⏸️ `MovieParser` - TMDB integration (to be added)
- ⏸️ `Scanner` - Directory scanning (to be added)
- ⏸️ `Organizer` - File organization (to be added)

### **Integration Tests**
- ✅ Comprehensive filename parsing patterns
- ✅ Real-world English movie patterns
- ✅ Real-world Chinese bilingual patterns
- ✅ Complex series patterns (Lord of the Rings, Star Wars, etc.)
- ✅ Quality and source detection
- ✅ Pattern statistics and analysis

### **Regression Tests**
- ✅ Unicode handling (Les Misérables)
- ✅ Long title series (Pirates of the Caribbean)
- ✅ Extended editions (Lord of the Rings)
- ✅ Chinese bilingual patterns
- ✅ Technical terms filtering
- ✅ Parenthesized content extraction
- ✅ Empty title prevention
- ✅ Dots in titles (A.I., I, Robot)

### **Specialized Tests**
- ✅ Edge case scanning (real directory analysis)
- ✅ Plex organization structure validation
- ✅ Rollback functionality testing
- ✅ Dynamic real-world data testing

## 🔧 **Test Utilities**

### **Shared Test Functions**
- Pattern validation helpers
- Performance measurement utilities
- Common test assertions
- Test data generation

### **Test Data**
- **`tests/test_data/movie_directory.txt`** - Real movie directory tree output
- **`tests/test_data/tv_directory.txt`** - Real TV show directory tree output
- **`tests/test_data/music_directory.txt`** - Real music directory tree output

## 🔄 **Maintenance**

### **When to Update Tests**
- New parsing patterns are added
- Performance requirements change
- Real-world data structure evolves
- Bug fixes require new test cases
- New components are added

### **Adding New Unit Tests**
1. Add `#[cfg(test)]` module to the source file
2. Write focused tests for the specific component
3. Include edge cases and error conditions
4. Ensure fast execution (<1 second per test)

### **Adding New Integration Tests**
1. Create test in appropriate `tests/` subdirectory
2. Use `#[tokio::test]` for async tests
3. Include comprehensive real-world patterns
4. Add performance assertions where appropriate

### **Test Data Updates**
- Update `tests/test_data/` files when directory structures change
- Regenerate tree outputs after significant reorganization
- Validate test data reflects current naming conventions

## 🎯 **Best Practices**

### **Unit Tests**
- Keep tests close to the code they test
- Use descriptive test names
- Test both success and failure cases
- Keep tests fast and focused

### **Integration Tests**
- Test component interactions
- Use realistic test data
- Include performance measurements
- Test end-to-end workflows

### **Regression Tests**
- Document why each test exists
- Include comments about the original bug
- Ensure tests are specific and reliable
- Don't remove tests without careful consideration

### **Specialized Tests**
- **Edge Case Scanner**: Use for real-world validation
- **Plex Organization**: Test directory structure compliance
- **Rollback Integration**: Ensure safety mechanisms work
- **Dynamic Real World**: Use actual data for comprehensive testing

## 🧹 **Cleanup Summary**

The test directory has been cleaned up by:
- ✅ **Removed all debug files** (`debug_*.rs`)
- ✅ **Organized tests by purpose** (integration, regression, performance)
- ✅ **Renamed files for clarity** (e.g., `edge_case_detector.rs` → `edge_case_scanner.rs`)
- ✅ **Maintained comprehensive coverage** while improving organization
- ✅ **Preserved valuable test cases** from debug files in proper test categories

---

**Note**: This refactored structure follows Rust conventions with unit tests co-located with source code and integration tests in the `tests/` directory. This provides better organization, faster feedback during development, and clearer separation of concerns.
