# Iteration 2 Plan: Enhanced Title Extraction and Matching

## Overview

Iteration 2 focuses on improving title extraction accuracy and TMDB matching coverage while maintaining the conservative approach to ensure accuracy.

## Current Issues to Address

### 1. Technical Term Filtering
**Problem**: Hard-coded technical terms list is not scalable and misses many patterns.

**Examples of missed terms**:
- `DualAudio`, `iNT`, `TLF` (release group terms)
- `HDTVRip`, `MP3`, `2Audio` (quality/audio terms)
- `GREENOTEA`, `CMCT`, `FRDS` (release group names)

**Impact**: Movies with these terms in filenames get skipped due to poor title extraction.

### 2. Limited Search Strategies
**Problem**: Only uses exact title + year matching with TMDB.

**Examples of missed matches**:
- Year mismatches (2022 vs 2023)
- Partial title matches (Chinese vs English titles)
- Alternative titles not in TMDB

**Impact**: Valid movies are skipped even when they exist in TMDB.

### 3. No Learning from User Feedback
**Problem**: System doesn't improve based on user corrections.

**Impact**: Same patterns continue to fail across different users.

## Proposed Solutions

### 1. Technical Terms Database

#### 1.1 Database Structure
```toml
# technical_terms.toml
[codecs]
x264 = "H.264 video codec"
x265 = "H.265 video codec"
H264 = "H.264 video codec"
H265 = "H.265 video codec"
AVC = "Advanced Video Coding"
HEVC = "High Efficiency Video Coding"

[audio]
DDP = "Dolby Digital Plus"
DTS = "Digital Theater Systems"
AC3 = "Dolby Digital"
AAC = "Advanced Audio Coding"
FLAC = "Free Lossless Audio Codec"
THD = "Dolby TrueHD"
MA = "Master Audio"

[quality]
1080p = "Full HD"
2160p = "4K Ultra HD"
720p = "HD"
4K = "4K Ultra HD"
UHD = "Ultra High Definition"
HDR = "High Dynamic Range"

[release_groups]
GREENOTEA = "Release group"
CMCT = "Release group"
FRDS = "Release group"
WiKi = "Release group"
HDWinG = "Release group"
NYPAD = "Release group"
ZigZag = "Release group"
NTb = "Release group"
HDS = "Release group"
PTerWEB = "Release group"
OurTV = "Release group"
GPTHD = "Release group"
HHWEB = "Release group"
QHstudIo = "Release group"
tdw9430 = "Release group"
EtHD = "Release group"

[audio_channels]
2Audio = "Dual audio"
3Audio = "Triple audio"
4Audio = "Quad audio"
5Audio = "5-channel audio"
6Audio = "6-channel audio"
7Audio = "7-channel audio"
8Audio = "8-channel audio"
9Audio = "9-channel audio"
10Audio = "10-channel audio"

[special_terms]
DualAudio = "Dual audio track"
iNT = "Internal release"
TLF = "Release group"
HDTVRip = "HDTV rip"
MP3 = "MP3 audio"
```

#### 1.2 Implementation
- **Configurable**: Users can add custom terms
- **Categorized**: Terms organized by type for better management
- **Pattern matching**: Support for regex patterns
- **Case insensitive**: Handle variations in capitalization

### 2. Enhanced TMDB Search Strategies

#### 2.1 Multi-Strategy Search
```rust
enum SearchStrategy {
    ExactTitleYear,           // Current: exact title + year
    TitleOnly,                // Title without year constraint
    YearRange,                // Title + year ± 1
    AlternativeTitles,        // Search alternative titles
    PartialMatch,             // Fuzzy title matching
    OriginalLanguage,         // Search in original language
}
```

#### 2.2 Implementation Priority
1. **Title + Year Range**: Allow ±1 year for year mismatches
2. **Alternative Titles**: Use TMDB's alternative titles
3. **Partial Matching**: Fuzzy match for similar titles
4. **Original Language**: Search in movie's original language

#### 2.3 Confidence Scoring
- **Exact match**: 1.0
- **Year range match**: 0.9
- **Alternative title match**: 0.8
- **Partial match**: 0.6-0.8 (based on similarity)
- **Original language match**: 0.7

### 3. User Feedback System

#### 3.1 Feedback Collection
```rust
struct UserCorrection {
    original_filename: String,
    extracted_title: String,
    corrected_title: String,
    tmdb_id: Option<i32>,
    user_notes: String,
    timestamp: DateTime<Utc>,
}
```

#### 3.2 Learning System
- **Pattern recognition**: Learn from user corrections
- **Term extraction**: Identify new technical terms
- **Title variations**: Learn common title patterns
- **Community sharing**: Share patterns across users

#### 3.3 Implementation
- **Correction file**: JSON file with user corrections
- **Pattern learning**: Automatically extract new patterns
- **Validation**: Validate learned patterns before use
- **Export/Import**: Share patterns between users

### 4. Improved Title Extraction

#### 4.1 Machine Learning Approach
- **Training data**: Use user corrections as training data
- **Pattern recognition**: Learn to identify technical terms
- **Title boundary detection**: Better identify where titles start/end
- **Language detection**: Improved CJK character handling

#### 4.2 Rule-Based Improvements
- **Bracket detection**: Better handling of titles in brackets
- **Punctuation handling**: Improved handling of dots, dashes, etc.
- **Number handling**: Better year and part number detection
- **Release group detection**: Identify and filter release group names

## Implementation Plan

### Phase 1: Technical Terms Database (Week 1-2)
1. **Design database structure**
2. **Create initial term database**
3. **Implement database loading**
4. **Update title extraction to use database**
5. **Add user configuration for custom terms**

### Phase 2: Enhanced TMDB Search (Week 3-4)
1. **Implement year range search**
2. **Add alternative title search**
3. **Implement partial matching**
4. **Update confidence scoring**
5. **Test with real-world data**

### Phase 3: User Feedback System (Week 5-6)
1. **Design feedback data structure**
2. **Implement feedback collection**
3. **Create pattern learning system**
4. **Add export/import functionality**
5. **Test learning accuracy**

### Phase 4: Machine Learning Integration (Week 7-8)
1. **Collect training data**
2. **Implement ML model**
3. **Integrate with existing system**
4. **Validate improvements**
5. **Performance optimization**

## Success Metrics

### Accuracy
- **False positive rate**: < 1% (incorrect organizations)
- **False negative rate**: < 10% (missed valid matches)
- **Overall accuracy**: > 95%

### Coverage
- **Success rate**: > 80% (up from current ~60%)
- **Reduced skipped files**: < 20% of total files
- **Improved matching**: 50% reduction in "No TMDB match found"

### User Experience
- **User satisfaction**: Measured through feedback
- **Reduced manual intervention**: < 10% of files need manual review
- **Faster processing**: < 30% increase in processing time

## Risk Mitigation

### 1. Accuracy Risks
- **Conservative defaults**: Keep high confidence thresholds by default
- **User validation**: Require user confirmation for low-confidence matches
- **Rollback capability**: Ensure users can easily undo changes

### 2. Performance Risks
- **Caching**: Cache TMDB responses to reduce API calls
- **Batch processing**: Process files in batches to manage memory
- **Progress indicators**: Show progress for long operations

### 3. Data Quality Risks
- **Validation**: Validate all learned patterns before use
- **User review**: Allow users to review and approve learned patterns
- **Backup**: Maintain backup of original patterns

## Future Considerations

### 1. Community Features
- **Shared patterns**: Community-driven pattern database
- **User ratings**: Rate and validate community patterns
- **Moderation**: Moderate community contributions

### 2. Advanced Features
- **TV show support**: Extend to TV show organization
- **Music support**: Add music file organization
- **Multi-language**: Support for more languages

### 3. Integration
- **Plex API**: Direct integration with Plex server
- **Other databases**: Support for other movie databases
- **Automation**: Scheduled organization tasks

## Conclusion

Iteration 2 will significantly improve the accuracy and coverage of the Plex Media Organizer while maintaining the conservative approach that ensures data integrity. The focus on user feedback and machine learning will create a system that continuously improves over time.

The phased approach allows for incremental improvements and validation at each step, ensuring that each enhancement provides real value to users while maintaining the high accuracy standards of the current system.
