#!/bin/bash

# Plex Media Organizer - Real World Data Test
# Tests our parser against actual movie directory structure

set -e

echo "🎬 Plex Media Organizer - Real World Data Test"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if movie directory file exists
MOVIE_FILE="../test-data/movie_directory.txt"
if [ ! -f "$MOVIE_FILE" ]; then
    echo -e "${RED}❌ Movie directory file not found: $MOVIE_FILE${NC}"
    exit 1
fi

echo -e "${BLUE}📁 Testing against: $MOVIE_FILE${NC}"
echo ""

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
echo -e "${BLUE}📂 Created temporary test directory: $TEMP_DIR${NC}"

# Function to extract filenames from tree output
extract_filenames() {
    local file="$1"
    grep -E '\.(mkv|mp4|avi|mov|wmv|flv)$' "$file" | \
    sed 's/.*├── \(.*\)/\1/' | \
    sed 's/.*└── \(.*\)/\1/' | \
    sed 's/.*│   ├── \(.*\)/\1/' | \
    sed 's/.*│   └── \(.*\)/\1/' | \
    grep -v '^$' | \
    head -100  # Limit to first 100 for testing
}

# Function to create test file
create_test_file() {
    local filename="$1"
    local test_path="$TEMP_DIR/$filename"
    touch "$test_path"
    echo "$test_path"
}

# Function to test a single file
test_file() {
    local file_path="$1"
    local filename=$(basename "$file_path")
    
    echo -e "${BLUE}🧪 Testing: $filename${NC}"
    
    # Run our parser
    local output
    output=$(cargo run -- test "$file_path" 2>&1 || echo "PARSING_FAILED")
    
    if echo "$output" | grep -q "PARSING_FAILED"; then
        echo -e "${RED}❌ Failed to parse${NC}"
        return 1
    elif echo "$output" | grep -q "✅ Parsing successful"; then
        echo -e "${GREEN}✅ Parsed successfully${NC}"
        
        # Extract key information
        local title=$(echo "$output" | grep "Title:" | sed 's/.*Title: \(.*\)/\1/')
        local year=$(echo "$output" | grep "Year:" | sed 's/.*Year: \(.*\)/\1/')
        local quality=$(echo "$output" | grep "Quality:" | sed 's/.*Quality: \(.*\)/\1/')
        local source=$(echo "$output" | grep "Source:" | sed 's/.*Source: \(.*\)/\1/')
        local confidence=$(echo "$output" | grep "Confidence:" | sed 's/.*Confidence: \(.*\)/\1/')
        
        echo "   📝 Title: $title"
        echo "   📅 Year: $year"
        echo "   🎯 Quality: $quality"
        echo "   📺 Source: $source"
        echo "   🎯 Confidence: $confidence"
        
        return 0
    else
        echo -e "${YELLOW}⚠️  Unexpected output${NC}"
        echo "$output" | head -5
        return 2
    fi
}

# Main test execution
echo -e "${BLUE}🔍 Extracting filenames from tree output...${NC}"
filenames=$(extract_filenames "$MOVIE_FILE")
total_files=$(echo "$filenames" | wc -l | tr -d ' ')
echo -e "${BLUE}📊 Found $total_files movie files to test${NC}"
echo ""

# Test statistics
successful=0
failed=0
unexpected=0
total_tested=0

# Test each filename
echo -e "${BLUE}🚀 Starting parser tests...${NC}"
echo ""

for filename in $filenames; do
    if [ -n "$filename" ]; then
        test_file_path=$(create_test_file "$filename")
        result=$(test_file "$test_file_path")
        
        case $? in
            0) ((successful++));;
            1) ((failed++));;
            *) ((unexpected++));;
        esac
        
        ((total_tested++))
        
        # Clean up test file
        rm -f "$test_file_path"
        
        echo ""
        
        # Progress indicator
        if [ $((total_tested % 10)) -eq 0 ]; then
            echo -e "${BLUE}📈 Progress: $total_tested/$total_files files tested${NC}"
        fi
    fi
done

# Clean up temp directory
rm -rf "$TEMP_DIR"

# Final results
echo "================================================"
echo -e "${BLUE}📊 TEST RESULTS SUMMARY${NC}"
echo "================================================"
echo -e "${GREEN}✅ Successfully parsed: $successful${NC}"
echo -e "${RED}❌ Failed to parse: $failed${NC}"
echo -e "${YELLOW}⚠️  Unexpected results: $unexpected${NC}"
echo -e "${BLUE}📁 Total files tested: $total_tested${NC}"

if [ $total_tested -gt 0 ]; then
    success_rate=$(echo "scale=1; $successful * 100 / $total_tested" | bc -l 2>/dev/null || echo "0")
    echo -e "${BLUE}📈 Success rate: ${success_rate}%${NC}"
fi

echo ""
echo -e "${BLUE}🎯 Coverage Analysis:${NC}"
echo "This test shows how well our current Movie MVP parser handles"
echo "your real-world filename patterns. Areas for improvement:"
echo ""
echo "• Chinese-English bilingual patterns"
echo "• Complex quality indicators (H264, H265, DDP5.1, etc.)"
echo "• Multi-part movies and collections"
echo "• Various source formats (BluRay, WEB-DL, HDTV, etc.)"
echo "• Audio and subtitle track information"

echo ""
echo -e "${GREEN}🎉 Test completed!${NC}"
