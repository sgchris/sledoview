#!/bin/bash
# SledoView Development Script
# Usage: ./scripts/dev.sh <command>

set -e

COMMAND=${1:-help}

case $COMMAND in
    "test")
        echo "Running tests..."
        cargo test
        ;;
    "build")
        echo "Building release version..."
        cargo build --release
        ;;
    "dev-build")
        echo "Building development version..."
        cargo build
        ;;
    "check")
        echo "Running all checks..."
        echo "1. Formatting..."
        cargo fmt --check || { echo "Formatting check failed. Run 'cargo fmt' to fix."; exit 1; }
        
        echo "2. Linting..."
        cargo clippy -- -D warnings || { echo "Clippy check failed."; exit 1; }
        
        echo "3. Testing..."
        cargo test || { echo "Tests failed."; exit 1; }
        
        echo "All checks passed!"
        ;;
    "format")
        echo "Formatting code..."
        cargo fmt
        ;;
    "lint")
        echo "Running clippy..."
        cargo clippy
        ;;
    "clean")
        echo "Cleaning build artifacts..."
        cargo clean
        if [ -d "example_db" ]; then
            rm -rf example_db
            echo "Removed example database"
        fi
        ;;
    "demo")
        echo "Creating example database and running demo..."
        cargo run --example create_test_db
        echo "Example database created. You can now run:"
        echo "  cargo run -- example_db"
        echo "or"
        echo "  ./target/release/sledoview example_db"
        ;;
    "install")
        echo "Installing sledoview..."
        cargo install --path .
        ;;
    "doc")
        echo "Generating documentation..."
        cargo doc --open
        ;;
    "coverage")
        echo "Running tests with coverage..."
        if command -v cargo-tarpaulin &> /dev/null; then
            cargo tarpaulin --out Html
            echo "Coverage report generated in tarpaulin-report.html"
        else
            echo "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
            exit 1
        fi
        ;;
    "help")
        echo "Available commands:"
        echo "  test       - Run all tests"
        echo "  build      - Build release version"
        echo "  dev-build  - Build development version"
        echo "  check      - Run format, lint, and test checks"
        echo "  format     - Format code with rustfmt"
        echo "  lint       - Run clippy linter"
        echo "  clean      - Clean build artifacts"
        echo "  demo       - Create example DB and show run instructions"
        echo "  install    - Install sledoview binary"
        echo "  doc        - Generate and open documentation"
        echo "  coverage   - Run tests with coverage (requires cargo-tarpaulin)"
        echo "  help       - Show this help message"
        ;;
    *)
        echo "Unknown command: $COMMAND"
        echo "Run './scripts/dev.sh help' for available commands"
        exit 1
        ;;
esac
