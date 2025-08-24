# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3] - 2025-08-24

### Added
- 🌳 **Tree Management System** - Complete support for SLED named trees
  - `trees` command to list all available trees with pattern matching
  - `trees regex <pattern>` for regex-based tree filtering
  - `select <tree>` command to switch between trees
  - `unselect` command to return to the default tree
  - Visual prompt indicators showing selected tree: `[tree_name]>`
  - Complete tree isolation - keys in different trees are separate
  - Automatic tree creation when selecting non-existent trees
- ✏️ **Write Operations** - Full CRUD capabilities for database modification
  - `set <key> <value>` command to create and update key-value pairs
  - `delete <key>` command to remove keys from the database
  - Immediate persistence to disk for all write operations
  - Key validation with comprehensive error checking
  - Support for quoted keys and values with escape sequences
  - Transactional safety with proper error handling
- 🎯 **Enhanced CRUD Operations** - All commands now work with tree selection
  - `count`, `list`, `get`, `set`, `delete`, `search` operations respect selected tree
  - Tree-aware tab completion for tree names
  - Comprehensive error handling for tree operations
- 📚 **Updated Documentation** - Extensive documentation for all functionality
  - Enhanced help system with tree command examples and write operation guides
  - Updated README with comprehensive usage examples
  - Tree management best practices and write operation safety

### Changed
- All existing commands now operate on the selected tree when one is active
- Enhanced REPL prompt to show selected tree context
- Improved tab completion to include tree names for relevant commands
- Updated help system with tree management and write operation examples
- Database validation now checks for write permissions

### Fixed
- Removed unused error variants to eliminate compiler warnings
- Improved error handling for tree operations
- Enhanced database safety checks

### Technical Improvements
- Added comprehensive test suite for tree functionality and write operations (31 total tests)
- Enhanced database abstraction layer with tree state management
- Improved command parsing to handle tree-related and write commands
- Added proper error types and handling for all operations
- Implemented key validation with security best practices

## [1.1.0] - 2025-01-19

### Legacy Entry
This version entry was incorrectly dated and has been superseded by v1.0.3.

## [0.1.0] - 2024-08-20

### Added
- Initial release of SledoView
- Interactive REPL interface with colored output
- Database validation with comprehensive checks
- `count` command to show total number of records
- `list` command with glob pattern matching
- `list regex` command with regular expression matching
- `get` command to retrieve key details and values
- `search` command to search values with patterns
- `search regex` command to search values with regular expressions
- `help` command with detailed usage examples
- Support for binary data detection and UTF-8 validation
- Proper error handling with descriptive messages
- Cross-platform compatibility (Windows, macOS, Linux)
- Comprehensive test suite with 95%+ coverage
- Command-line interface with `--help` and `--version` options

### Features
- **Read-only access**: Safe database browsing without modification risk
- **Pattern matching**: Both glob (`*`, `?`) and regex support
- **Colored output**: Beautiful terminal interface with syntax highlighting
- **Value truncation**: Smart truncation for large values in listings
- **Binary data handling**: Proper detection and display of non-UTF8 data
- **Database validation**: Thorough checks before opening
- **Cross-platform**: Works on Windows, macOS, and Linux

### Technical Details
- Built with Rust 2021 edition
- Uses SLED 0.34 for database access
- Rustyline for REPL functionality
- Colored terminal output with the `colored` crate
- Regex support with the `regex` crate
- Comprehensive error handling with `anyhow` and `thiserror`
- Command-line parsing with `clap` 4.0

### Documentation
- Comprehensive README with usage examples
- MIT license
- Contribution guidelines
- Example database creation script
- Full command reference with examples

[Unreleased]: https://github.com/sgchris/sledoview/compare/v1.0.3...HEAD
[1.0.3]: https://github.com/sgchris/sledoview/compare/v0.1.0...v1.0.3
[0.1.0]: https://github.com/sgchris/sledoview/releases/tag/v0.1.0
