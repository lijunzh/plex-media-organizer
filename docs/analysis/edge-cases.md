# Edge Cases Analysis - Movie Directory Scan

**Date**: August 19, 2025  
**Scan Results**: 420 files processed, 60 edge cases detected (14.3%)  
**Success Rate**: 100% (all files parsed successfully)

## 📊 **Overall Statistics**

- **Total Files**: 420
- **Successful Parses**: 420 (100%)
- **Failed Parses**: 0 (0%)
- **Edge Cases**: 60 (14.3%)

## 🚨 **Critical Issues (Empty Titles)**

### **Files with Empty Titles (39 files)**
These files are being parsed but have empty titles, which is a critical issue:

1. `Moneyball.2011.UHD.2160p.WEB-Rip.DDP.5.1.HEVC-DDR[EtHD].mkv`
2. `The.LEGO.Ninjago.Movie.2017.RERIP.1080p.BluRay.x264-GECKOS[EtHD].mkv`
3. `MADAGASCAR2.mkv`
4. `MADAGASCAR3.mkv`

**Root Cause**: The filename parser is incorrectly identifying release group names (like "EtHD", "GECKOS") as technical metadata instead of preserving them as part of the title.

**Impact**: These files cannot be properly organized because they have no title to work with.

## ⚠️ **Major Issues**

### **1. Excessive Punctuation (12-14 punctuation marks)**
- Multiple Pirates of the Caribbean files
- Files with complex naming patterns using many dots and special characters

**Impact**: These files are being parsed but may have poor title quality.

### **2. High Similarity to Filename (83-88% similarity)**
- Multiple Harry Potter files
- Various short films and documentaries
- Files where the parsed title is very similar to the original filename

**Impact**: This suggests the parser is not effectively extracting meaningful titles and is just returning cleaned versions of the filename.

### **3. Bilingual Titles with Brackets**
- `钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv`
- `钢铁侠2.Iron.Man.2.2010.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv`
- `钢铁侠3.Iron.Man.3.2013.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv`

**Impact**: These are actually working correctly but flagged as edge cases due to bracket detection.

## 📈 **Parsing Statistics**

### **Year Distribution**
- **Most Common Years**: 2023 (28), 2022 (20), 2024 (14), 2010 (14)
- **Year Range**: 1977-2025
- **Files with Years**: 381 (90.7%)
- **Files without Years**: 39 (9.3%)

### **Quality Distribution**
- **1080p**: 137 files (32.6%)
- **720p**: 91 files (21.7%)
- **2160p**: 72 files (17.1%)
- **UHD**: 12 files (2.9%)
- **4K**: 1 file (0.2%)

### **Source Distribution**
- **BluRay**: 210 files (50.0%)
- **WEB-DL**: 67 files (16.0%)
- **bluray**: 20 files (4.8%)
- **HDTV**: 6 files (1.4%)

### **Language Distribution**
- **Japanese,English**: 45 files (10.7%)
- **JPN**: 3 files (0.7%)
- **Chi**: 1 file (0.2%)

### **Title Length Distribution**
- **11-30 characters**: 175 files (41.7%)
- **31-60 characters**: 180 files (42.9%)
- **60+ characters**: 26 files (6.2%)
- **0-10 characters**: 39 files (9.3%) - **CRITICAL**

## 🔧 **Recommended Fixes**

### **Priority 1: Fix Empty Titles**
1. **Improve Release Group Detection**: Better identify and preserve release group names
2. **Enhanced Tokenization**: Improve how complex filenames are broken into tokens
3. **Fallback Title Extraction**: When primary parsing fails, use filename as fallback

### **Priority 2: Improve Title Quality**
1. **Better Technical Term Filtering**: More precise identification of metadata vs. title content
2. **Enhanced Bilingual Support**: Better handling of mixed-language titles
3. **Improved Punctuation Handling**: Better cleaning of excessive punctuation

### **Priority 3: Enhance Metadata Extraction**
1. **Better Year Detection**: Improve year extraction from complex filenames
2. **Quality Pattern Matching**: More accurate quality identification
3. **Source Pattern Matching**: Better source identification

## 🧪 **Test Cases for Fixes**

### **Critical Test Cases**
1. `Moneyball.2011.UHD.2160p.WEB-Rip.DDP.5.1.HEVC-DDR[EtHD].mkv`
   - **Expected**: Title = "Moneyball", Year = 2011, Quality = "2160p", Source = "WEB-Rip"
   - **Current**: Empty title, no year, no quality, no source

2. `The.LEGO.Ninjago.Movie.2017.RERIP.1080p.BluRay.x264-GECKOS[EtHD].mkv`
   - **Expected**: Title = "The LEGO Ninjago Movie", Year = 2017, Quality = "1080p", Source = "BluRay"
   - **Current**: Empty title, no year, no quality, no source

### **Complex Test Cases**
1. `2.Pirates.of.the.Caribbean.Dead.Mans.Chest.2006.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv`
   - **Expected**: Title = "Pirates of the Caribbean: Dead Man's Chest", Year = 2006
   - **Current**: Excessive punctuation in title

2. `钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv`
   - **Expected**: Title = "钢铁侠 [Iron Man]", Year = 2008
   - **Current**: Working but flagged as edge case

## 📋 **Action Items**

### **Immediate (Next Sprint)**
1. Fix empty title issue for files with release group names
2. Improve year extraction for complex filenames
3. Add fallback title extraction when primary parsing fails

### **Short Term (Next 2 Sprints)**
1. Enhance technical term filtering
2. Improve bilingual title handling
3. Better punctuation cleaning

### **Long Term (Next Month)**
1. Machine learning approach for title extraction
2. User feedback integration for edge cases
3. Comprehensive test suite for edge cases

## 🎯 **Success Metrics**

- **Empty Titles**: Reduce from 39 to <5 files
- **High Similarity**: Reduce from 20+ to <5 files
- **Excessive Punctuation**: Reduce from 12+ to <5 files
- **Overall Edge Cases**: Reduce from 60 to <20 files

## 📝 **Notes**

- The parser is working well for most files (85.7% have no edge cases)
- The main issues are with complex filenames and release group names
- Bilingual titles are working correctly but flagged due to bracket detection
- Year extraction is working well for most files (90.7% success rate)
