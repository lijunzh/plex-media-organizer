# Filename Parsing Strategy

## Overview

The filename parsing system follows a TMDB-first approach to ensure accurate title extraction and language-aware output formatting.

## Core Strategy

### 1. Technical Terms Filtering
- **Filter out all technical and non-title tokens** including:
  - Years (e.g., 2023, 1999)
  - Quality indicators (e.g., 1080p, 2160p, 4K, UHD)
  - Release groups (e.g., YTS, FRDS, 3L)
  - Audio/video codecs (e.g., x264, x265, DTS, AC3)
  - Source indicators (e.g., BluRay, WEB-DL, HDTV)
  - File extensions and other metadata

### 2. Title Extraction Priority
- **Primary**: Extract English title for better TMDB search
- **Fallback**: Use original language title if no English title found
- **Goal**: Obtain the cleanest possible title for TMDB matching

### 3. TMDB Integration
- **Search**: Use clean title + year combination for TMDB search
- **Confidence Check**: Skip TMDB enhancement if confidence is low (< 0.3)
- **Language Detection**: Use TMDB's `original_language` field to determine output format

### 4. Output Formatting
Based on TMDB original language:

#### English Movies (`original_language: "en"`)
```
Output: "Movie Title (2023) [1080p]"
```

#### Non-English Movies (`original_language: "zh"`, `"ja"`, `"ko"`, etc.)
```
Output: "Original Title (2023) [English Title] [1080p]"
```

### 5. Metadata Reassembly
- **Year**: Always added back to title
- **Quality**: Added optionally (controlled by configuration)
- **Source**: Stored separately (not in title)

## Implementation Flow

```mermaid
graph TD
    A[Raw Filename] --> B[Remove Extension]
    B --> C[Tokenize Filename]
    C --> D[Filter Technical Terms]
    D --> E[Extract Clean Title]
    E --> F[Extract Year]
    F --> G[TMDB Search: Title + Year]
    G --> H{TMDB Match Found?}
    H -->|Yes| I[Get Original Language]
    H -->|No| J[Use Default Language]
    I --> K{Original Language?}
    J --> K
    K -->|English| L[Format: Title (Year)]
    K -->|Non-English| M[Format: Original (Year) [English]]
    L --> N[Add Quality if enabled]
    M --> N
    N --> O[Final Output]
```

## Configuration Options

### Quality Display
```toml
[organization]
# Whether to include quality in the final title
include_quality_in_title = false
```

### TMDB Integration
```toml
[apis]
# TMDB API key for enhanced parsing
tmdb_api_key = "your_api_key_here"

[organization.matching]
# Minimum confidence threshold for TMDB enhancement
min_tmdb_confidence = 0.3
```

## Examples

### Input: `The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv`
1. **Filter**: Remove `2022`, `2160p`, `Remux`, `HEVC`, `DoVi`, `TrueHD`, `7.1`, `3L`
2. **Clean Title**: `The Batman`
3. **Year**: `2022`
4. **TMDB Search**: "The Batman" + 2022
5. **TMDB Result**: English movie (`original_language: "en"`)
6. **Output**: `The Batman (2022) [2160p]`

### Input: `钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv`
1. **Filter**: Remove `2008`, `BluRay`, `2160p`, `x265`, `10bit`, `HDR`, `4Audio`, `mUHD`, `FRDS`
2. **Clean Title**: `Iron Man` (prioritize English)
3. **Year**: `2008`
4. **TMDB Search**: "Iron Man" + 2008
5. **TMDB Result**: English movie (`original_language: "en"`)
6. **Output**: `Iron Man (2008) [2160p]`

### Input: `武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4`
1. **Filter**: Remove `1992`, `2160p`, `WEB-DL`, `H264`, `AAC`, `2Audio`, `OurTV`
2. **Clean Title**: `King of Beggars` (prioritize English)
3. **Year**: `1992`
4. **TMDB Search**: "King of Beggars" + 1992
5. **TMDB Result**: Chinese movie (`original_language: "zh"`)
6. **Output**: `武状元苏乞儿 (1992) [King of Beggars] [2160p]`

## Benefits

1. **Accurate Language Detection**: Uses TMDB's authoritative original language data
2. **Better TMDB Matching**: Clean titles improve search accuracy
3. **Consistent Output**: Predictable formatting based on movie origin
4. **Flexible Configuration**: Quality inclusion controlled by user preference
5. **Plex-Friendly**: Maintains metadata for proper organization

## Migration from Current System

The current system creates bilingual formats regardless of original language. The new system will:

1. **Phase 1**: Implement new parsing logic alongside existing
2. **Phase 2**: Add configuration option to switch between old/new behavior
3. **Phase 3**: Make new behavior default
4. **Phase 4**: Remove old parsing logic

## Testing Strategy

1. **Unit Tests**: Test each parsing step independently
2. **Integration Tests**: Test full parsing pipeline with TMDB integration
3. **Real-world Tests**: Validate against actual media collections
4. **Regression Tests**: Ensure existing functionality is preserved
