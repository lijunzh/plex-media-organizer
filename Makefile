# Plex Media Organizer - Development Makefile

.PHONY: help build test clean fmt clippy check-all release install

# Default target
help:
	@echo "🚀 Plex Media Organizer - Development Commands"
	@echo ""
	@echo "📦 Build Commands:"
	@echo "  build      - Build the project in debug mode"
	@echo "  release    - Build the project in release mode"
	@echo "  clean      - Clean build artifacts"
	@echo ""
	@echo "🧪 Testing Commands:"
	@echo "  test       - Run all tests"
	@echo "  test-verbose - Run tests with output"
	@echo ""
	@echo "🔧 Code Quality:"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  clippy     - Run clippy linting"
	@echo "  check-all  - Run all quality checks"
	@echo ""
	@echo "📱 Installation:"
	@echo "  install    - Install to cargo bin directory"
	@echo ""
	@echo "📊 Development:"
	@echo "  run        - Run the application"
	@echo "  run-setup  - Run setup command"
	@echo "  run-help   - Show application help"

# Build commands
build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

# Testing commands
test:
	cargo test

test-verbose:
	cargo test -- --nocapture

# Code quality commands
fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

check-all: fmt clippy test
	@echo "✅ All quality checks passed!"

# Installation
install:
	cargo install --path .

# Development commands
run:
	cargo run

run-setup:
	cargo run -- setup

run-help:
	cargo run -- help

# Quick development workflow
dev: fmt clippy test build
	@echo "✅ Development build ready!"

# Release preparation
release-prep: clean fmt clippy test release
	@echo "✅ Release build ready!"
	@echo "📦 Binary location: target/release/plex-media-organizer"
