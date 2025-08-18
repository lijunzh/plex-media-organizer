# Tests Directory

This directory contains the comprehensive test suite for the Plex Media Organizer.

## 📁 **Test Structure**

### **Unit Tests**
- **`real_world_patterns_test.rs`** - Static unit tests with hardcoded test cases
  - Tests specific filename patterns with predefined assertions
  - Focuses on individual pattern validation
  - Uses hardcoded examples from real-world data
  - Fast execution, specific validation

### **Integration Tests**
- **`dynamic_real_world_test.rs`** - Dynamic integration tests using actual tree files
  - Tests against real directory tree outputs from `tests/test_data/`
  - Validates performance and success rates
  - Uses actual media server directory structures
  - Comprehensive real-world validation

### **Test Utilities**
- **`test_utils.rs`** - Shared test utilities and helpers
  - `DynamicTestRunner` for processing tree files
  - Pattern analysis and reporting
  - Performance measurement utilities
  - Common test assertions

## 🎯 **Test Data**

Test data is stored in `tests/test_data/` directory:

- **`movie_directory.txt`** - Real movie directory tree output (✅ actively used)
- **`tv_directory.txt`** - Real TV show directory tree output (⏸️ for future iterations)
- **`music_directory.txt`** - Real music directory tree output (⏸️ for future iterations)

### **Why Test Data is at Root Level:**
- **Standard practice** for large data files (1.7MB+)
- **Easy to reference** from both unit and integration tests
- **Separate from test code** for better organization
- **Can be gitignored** if needed for privacy
- **Better performance** - large files don't impact git operations

## 🚀 **Running Tests**

### **All Tests**
```bash
cargo test
```

### **Unit Tests Only**
```bash
cargo test --lib
cargo test --test real_world_patterns_test
```

### **Integration Tests Only**
```bash
cargo test --test dynamic_real_world_test
```

### **With Output**
```bash
cargo test -- --nocapture
```

## 📊 **Test Coverage**

### **Static Tests (`real_world_patterns_test.rs`)**
- ✅ Chinese-English bilingual patterns
- ✅ Bracketed Chinese title patterns
- ✅ Multi-part movie patterns
- ✅ Quality and source detection
- ✅ Complex modern patterns
- ✅ Edge cases and error handling

### **Dynamic Tests (`dynamic_real_world_test.rs`)**
- ✅ Real-world movie directory validation
- ⏸️ Real-world TV directory validation (not in scope for iteration 1)
- ⏸️ Real-world music directory validation (not in scope for iteration 1)
- ✅ Performance benchmarking
- ✅ Success rate validation
- ✅ Pattern distribution analysis

## 🔧 **Test Utilities**

### **DynamicTestRunner**
- Processes tree output files
- Analyzes parsing success rates
- Generates performance reports
- Identifies pattern distributions

### **Pattern Analysis**
- Chinese-English bilingual detection
- Bracketed pattern detection
- Quality pattern identification
- Source format detection

## 📈 **Performance Benchmarks**

The dynamic tests validate:
- **Success Rate**: >95% on real-world data
- **Performance**: 445+ files/second parsing speed
- **Memory Usage**: Efficient parsing without bloat
- **Scalability**: Handles large directories (5,774+ files)

## 🎯 **Test Philosophy**

### **Static Tests**
- **Fast execution** for development feedback
- **Specific validation** of parsing logic
- **Edge case coverage** for robustness
- **Regression prevention** for changes

### **Dynamic Tests**
- **Real-world validation** against actual data
- **Performance measurement** with realistic workloads
- **Pattern discovery** from actual usage
- **Production readiness** verification

## 🔄 **Maintenance**

### **When to Update Tests**
- New parsing patterns are added
- Performance requirements change
- Real-world data structure evolves
- Bug fixes require new test cases

### **Test Data Updates**
- Update `tests/test_data/` files when directory structures change
- Regenerate tree outputs after significant reorganization
- Validate test data reflects current naming conventions

---

**Note**: The separation between static and dynamic tests, along with the test data organization, provides comprehensive validation while maintaining good performance and organization.
