# Configuration-Based Technical Terms Management

## 🎯 Overview

This PR implements a **configuration-driven approach** for managing technical terms used in title filtering, replacing the previous hard-coded approach. This provides users with a **single source of truth** for managing release groups, codecs, quality indicators, and other technical terms that should be filtered from movie titles.

## 🚀 Key Changes

### ✅ **Configuration-First Architecture**
- **Primary**: `config.toml` file (user-editable, version-controlled)
- **Fallback**: Minimal hard-coded essential terms (never changes)
- **No database storage** for technical terms (eliminates user confusion)

### ✅ **Comprehensive Term Categories**
- **Release Groups**: YIFY, YTS, RARBG, 3L, CMCT, etc.
- **Video/Audio Terms**: x264, x265, DTS, AC3, TrueHD, 7.1, 5.1, 2.0
- **Source/Platform Terms**: Netflix, Amazon, iTunes, ATVP
- **File Format Terms**: mkv, mp4, avi, web, dl, rip
- **Special Edition Terms**: Extended, Director's Cut, Unrated
- **Custom Terms**: User-defined terms for specific needs

### ✅ **Enhanced Functionality**
- **Case-insensitive matching** for better filtering accuracy
- **Improved title cleaning** with proper separator handling
- **Essential fallback terms** to ensure parser always works
- **Comprehensive documentation** with examples and troubleshooting

## 🔧 Technical Implementation

### **New Configuration Structure**
```toml
[organization.technical_terms]
# Release group names to filter out
release_groups = ["YIFY", "YTS", "RARBG", "3L", "CMCT"]

# Video/audio codec and quality terms
video_audio_terms = ["x264", "x265", "DTS", "AC3", "TrueHD", "7.1"]

# Source/platform names
source_platform_terms = ["Netflix", "Amazon", "iTunes"]

# File format and container terms
file_format_terms = ["mkv", "mp4", "avi", "web", "dl", "rip"]

# Special edition and version terms
special_edition_terms = ["Extended", "Director's Cut", "Unrated"]

# Additional custom terms
custom_terms = ["YourCustomTerm1", "YourCustomTerm2"]
```

### **Enhanced TechnicalTermsFilter**
- `with_config()` method for configuration-driven initialization
- `add_essential_fallback_terms()` for critical term protection
- Improved case-insensitive matching in `is_technical_term()`
- Better title cleaning with separator handling

### **CLI Integration**
- Added `terms` command structure (placeholder for future implementation)
- Ready for future CLI-based term management features

## 🎉 Benefits

### **For Users**
- ✅ **Single source of truth**: All terms in `config.toml`
- ✅ **User-editable**: No need to recompile for changes
- ✅ **Version controlled**: Changes tracked in git
- ✅ **Immediate effect**: Restart app, changes apply
- ✅ **Portable**: Config file moves with the app
- ✅ **No confusion**: Clear, simple management process

### **For Developers**
- ✅ **Maintainable**: Clear separation of concerns
- ✅ **Extensible**: Easy to add new term categories
- ✅ **Testable**: Comprehensive test coverage
- ✅ **Documented**: Clear implementation and usage docs

## 🐛 Issues Fixed

### **Original 3L Release Group Issue**
- **Problem**: "3L" was appearing in organized movie titles
- **Solution**: Added "3L" to release groups list with proper filtering
- **Result**: "The Batman 3L" → "The Batman" ✅

### **Technical Terms Case Sensitivity**
- **Problem**: Terms weren't being filtered due to case mismatches
- **Solution**: Implemented case-insensitive matching
- **Result**: Better filtering accuracy across all term types ✅

### **Title Cleaning Issues**
- **Problem**: Dots and separators weren't being cleaned properly
- **Solution**: Improved `clean_title()` function with separator handling
- **Result**: "The.Batman.2022" → "The Batman 2022" ✅

## 📚 Documentation

### **New Documentation**
- **`docs/user-guide/technical-terms.md`**: Comprehensive guide for managing technical terms
- **Updated README.md**: Added technical terms management section
- **Updated CHANGELOG.md**: Detailed change documentation

### **Documentation Features**
- Step-by-step configuration guide
- Term category explanations
- Best practices and troubleshooting
- Real-world examples
- Common issues and solutions

## 🧪 Testing

### **Test Coverage**
- ✅ **10 technical terms tests** passing
- ✅ **97 unit tests** passing
- ✅ **Integration tests** passing
- ✅ **Regression tests** passing

### **Key Test Cases**
- `test_filter_3l_release_group`: Verifies 3L filtering works
- `test_is_technical_term`: Verifies case-insensitive matching
- `test_clean_title`: Verifies proper separator handling
- `test_filter_with_audio`: Verifies audio format filtering

## 🔄 Migration

### **Backward Compatibility**
- ✅ **No breaking changes**: Existing functionality preserved
- ✅ **Automatic migration**: Uses default configuration if none exists
- ✅ **Fallback protection**: Essential terms always available

### **User Migration**
- **No action required**: Works with existing configurations
- **Optional enhancement**: Users can customize technical terms in `config.toml`
- **Gradual adoption**: Can start with defaults and customize over time

## 🚀 Future Enhancements

### **Planned CLI Features**
- `terms --list`: List all technical terms
- `terms --add`: Add new technical terms
- `terms --remove`: Remove technical terms
- `terms --categories`: Show terms by category
- `terms --export/--import`: Backup and restore term lists

### **Potential Improvements**
- Database-backed term statistics
- Automatic term discovery from filenames
- Community-maintained term lists
- Term usage analytics

## 📋 Checklist

- [x] **Configuration structure** implemented
- [x] **TechnicalTermsFilter** refactored
- [x] **Essential fallback terms** added
- [x] **Case-insensitive matching** implemented
- [x] **Title cleaning** improved
- [x] **CLI command structure** added
- [x] **Documentation** created and updated
- [x] **Tests** written and passing
- [x] **CHANGELOG** updated
- [x] **README** updated
- [x] **Code formatting** applied
- [x] **Clippy checks** passing
- [x] **All tests** passing

## 🎯 Impact

This change significantly improves the **user experience** and **maintainability** of technical terms management:

1. **Eliminates user confusion** about where to manage terms
2. **Provides clear, simple process** for customization
3. **Maintains backward compatibility** with existing setups
4. **Sets foundation** for future CLI-based term management
5. **Improves filtering accuracy** with case-insensitive matching
6. **Fixes the original 3L issue** and similar edge cases

The configuration-based approach is **much more user-friendly** than the previous hard-coded approach and eliminates the complexity that would have existed with dual storage systems.
