# Test Data - Media Directory Tree Outputs

This directory contains real-world media directory structures captured using the `tree` command. These files serve as valuable test data for developing and validating the Plex Media Organizer.

## 📁 **Files**

### `movie_directory.txt`
- **Source**: `/Volumes/media/movie`
- **Content**: Movie files with various naming patterns
- **Key Patterns**:
  - Chinese-English bilingual titles (白蛇2：青蛇劫起..Green.Snake.2021)
  - Bracketed Chinese titles ([雏菊(导演剪辑版)].Daisy.2006)
  - Collection movies (逃学威龙.Fight.Back.to.School.1991)
  - Multi-part movies (CD1, CD2, Part 1, Part 2)
  - Japanese anime movies ([名侦探柯南：百万美元的五棱星])
  - English movies (Avengers.Age.of.Ultron.2015)

### `tv_directory.txt`
- **Source**: `/Volumes/media/tv`
- **Content**: TV shows and anime series
- **Key Patterns**:
  - Complex anime patterns ((BD)十二国記 第01話「月の影 影の海　一章」)
  - Season-based organization (season 1/, season 2/)
  - Subtitle files (.fansub.ass)
  - Traditional TV patterns (Breaking Bad S01E01)

### `music_directory.txt`
- **Source**: `/Volumes/media/music`
- **Content**: Music files and albums
- **Key Patterns**:
  - Chinese music (1-01 - 我是一只小小鸟.flac)
  - English music (Pink Floyd - The Wall - 02 - Another Brick in the Wall.flac)
  - Multi-disc albums (1-01, 2-01 patterns)
  - Artist/Album/Track structures

## 🎯 **Purpose**

These tree outputs are used for:

1. **Pattern Analysis**: Understanding real-world naming conventions
2. **Test Case Generation**: Creating comprehensive test suites
3. **Parser Validation**: Testing parsing strategies against actual data
4. **Edge Case Discovery**: Identifying complex naming patterns
5. **Performance Testing**: Testing with realistic directory structures

## 🔍 **How to Use**

### For Development
- **Reference**: Use as a guide for implementing parsing strategies
- **Testing**: Extract specific patterns for unit tests
- **Validation**: Verify that parsers handle real-world cases correctly

### For Testing
- **Unit Tests**: Create test cases based on specific patterns
- **Integration Tests**: Test complete workflows with real directory structures
- **Performance Tests**: Measure parsing speed with actual data

### For Documentation
- **Examples**: Show real-world usage patterns
- **Pattern Guide**: Document supported naming conventions
- **User Guide**: Provide examples of what the system can handle

## 📊 **Data Characteristics**

### **Movie Directory**
- **Size**: ~60KB
- **Lines**: 734
- **Languages**: Chinese, English, Japanese
- **Formats**: mkv, mp4, avi
- **Quality**: 720p, 1080p, 2160p, 4K

### **TV Directory**
- **Size**: ~602KB
- **Lines**: 6,837
- **Languages**: Japanese, English
- **Formats**: mkv, ass (subtitles)
- **Structure**: Season-based organization

### **Music Directory**
- **Size**: ~1.1MB
- **Lines**: 17,899
- **Languages**: Chinese, English
- **Formats**: flac, mp3, m4a, wav
- **Structure**: Artist/Album/Track hierarchy

## 🚀 **Future Updates**

These files should be updated when:
- New naming patterns are discovered
- Directory structures change
- Additional media types are added
- Significant reorganization occurs

## 📝 **Notes**

- **Privacy**: These files contain only directory structures, not actual media content
- **Accuracy**: Captured directly from your media server using `tree` command
- **Completeness**: Represents the full scope of your media collection
- **Maintenance**: Keep synchronized with actual directory changes

---

**Generated**: August 16, 2024  
**Source**: Real media server directory structures  
**Purpose**: Development and testing reference data
