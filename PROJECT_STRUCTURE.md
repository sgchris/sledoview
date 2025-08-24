# SledoView Project Structure

## Overview
SledoView is a comprehensive CLI tool for viewing and editing SLED databases. This document provides an overview of the project structure and components.

## Commands Reference

| Command | Description | Example |
|---------|-------------|---------|
| `count` | Show total records | `count` |
| `list [pattern]` | List keys (glob) | `list user_*` |
| `list regex <regex>` | List keys (regex) | `list regex user_\d+` |
| `get <key>` | Get key details | `get user_001` |
| `search <pattern>` | Search values (glob) | `search *@example.com` |
| `search regex <regex>` | Search values (regex) | `search regex \d{4}-\d{2}-\d{2}` |
| `help` | Show help | `help` |
| `exit` | Exit application | `exit` |

## Development Workflow

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo (included with Rust)

### Quick Commands
```bash
# Run all checks (format, lint, test)
.\scripts\dev.ps1 check

# Build release version
.\scripts\dev.ps1 build

# Create test database and run
.\scripts\dev.ps1 demo

# Run tests
.\scripts\dev.ps1 test

# Install locally
.\scripts\dev.ps1 install
```

### Manual Commands
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Build release
cargo build --release

# Create example database
cargo run --example create_test_db

# Run with example database
cargo run -- example_db
```

## Dependencies

### Runtime Dependencies
- `sled`: SLED database access
- `clap`: Command-line parsing
- `colored`: Terminal colors
- `anyhow`: Error handling
- `rustyline`: REPL functionality
- `regex`: Pattern matching
- `thiserror`: Error derive macros

### Development Dependencies
- `tempfile`: Temporary test databases
- `assert_cmd`: CLI testing
- `predicates`: Test assertions

## Design Principles

1. **Safety First**: Read-only operations, comprehensive validation
2. **User Experience**: Colored output, clear error messages, intuitive commands
3. **Robustness**: Extensive testing, proper error handling
4. **Performance**: Efficient database operations, smart memory usage
5. **Maintainability**: Clean code structure, good documentation

## Future Enhancements

Potential areas for expansion:
- Export functionality (JSON, CSV)
- Key/value filtering and transformation
- Database statistics and analysis
- Multi-database comparison
- Configuration file support
- Plugin system for custom commands

## License

Dual-licensed under MIT OR Apache-2.0 - choose the license that best fits your needs.
