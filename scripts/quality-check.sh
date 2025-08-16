#!/bin/bash

# Plex Media Organizer - Quality Check Script
# Run this before committing to ensure code quality

echo "🔍 Running quality checks..."

# Check formatting
echo "📝 Checking code formatting..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
echo "🔧 Running clippy checks..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy checks failed. Fix the warnings and try again."
    exit 1
fi

# Run tests
echo "🧪 Running tests..."
cargo test
if [ $? -ne 0 ]; then
    echo "❌ Tests failed. Fix the failing tests and try again."
    exit 1
fi

echo "✅ All quality checks passed!"
echo "�� Ready to commit!"
