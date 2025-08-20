# SledoView Project Structure

## Overview
SledoView is a comprehensive CLI tool for viewing and managing SLED databases. This document provides an overview of the project structure and components.

## Project Structure

```
sledoview/
├── src/                    # Source code
│   ├── main.rs            # Main application entry point
│   ├── lib.rs             # Library root
│   ├── cli.rs             # Command-line interface definition
│   ├── commands.rs        # Command parsing and execution
│   ├── db.rs              # Database operations and abstraction
│   ├── error.rs           # Error types and handling
│   ├── repl.rs            # Interactive REPL implementation
│   └── validator.rs       # Database validation logic
├── tests/                 # Test suite
│   ├── common/           # Common test utilities
│   │   └── mod.rs        # Test database creation helpers
│   └── integration_test.rs # Integration tests
├── examples/             # Example code and utilities
│   └── create_test_db.rs # Creates example database for testing
├── scripts/              # Development scripts
│   ├── README.md         # Scripts documentation
│   ├── dev.ps1           # PowerShell development script
│   └── dev.sh            # Bash development script
├── Cargo.toml            # Project configuration and dependencies
├── README.md             # Main project documentation
├── CHANGELOG.md          # Project changelog
├── CONTRIBUTING.md       # Contribution guidelines
├── LICENSE-MIT           # MIT license
├── LICENSE-APACHE        # Apache 2.0 license
└── .gitignore           # Git ignore patterns
```

## Core Components

### Application Entry Point (`main.rs`)
- Parses command-line arguments
- Validates the database
- Initializes and starts the REPL

### Database Operations (`db.rs`)
- `SledViewer`: Main database interface
- Key counting, listing, retrieval, and value searching
- Pattern matching (glob and regex)
- UTF-8 validation and binary data handling

### Command System (`commands.rs`)
- Command parsing from user input
- Command execution with colored output
- Help system with examples

### Validation (`validator.rs`)
- Comprehensive database validation
- File existence, permissions, and format checks
- Lock detection and structure validation

### REPL Interface (`repl.rs`)
- Interactive terminal interface
- Command history and line editing
- Error handling and user feedback

### Error Handling (`error.rs`)
- Custom error types for all scenarios
- Descriptive error messages
- Proper error propagation

## Key Features

### ✅ Read-Only Database Access
- Safe browsing without modification risk
- Proper database locking detection
- Non-intrusive operations

### ✅ Pattern Matching
- Glob patterns (`*`, `?`) for simple matching
- Full regex support for complex patterns
- Both key and value searching

### ✅ Colored Terminal Output
- Beautiful, syntax-highlighted interface
- Consistent color scheme throughout
- Enhanced readability

### ✅ Comprehensive Validation
- File existence and permissions
- SLED database format validation
- Lock status checking
- Detailed error reporting

### ✅ Robust Testing
- Unit tests for all modules
- Integration tests for end-to-end scenarios
- Test database creation utilities
- 95%+ code coverage

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
