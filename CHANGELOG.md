# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Library-facing database and validator APIs now return `SledoViewError` directly instead of erasing errors into `anyhow` through the core layers
- CLI key validation now accepts printable UTF-8 text keys while keeping the existing key length cap as a CLI policy
- Known REPL commands now reject extra trailing arguments instead of silently ignoring them

### Fixed
- Startup lock handling no longer depends on downcasting `anyhow::Error` to recover `DatabaseLocked`
- UTF-8 text keys such as `config_日本` and `café` can now be written through the CLI consistently with the existing read path

## [1.1.0] - 2026-02-20

### Added
- **Binary key support** - Keys that are not valid UTF-8 are now displayed as uppercase hex (`AABBCCDD....EEFF`), with truncation for long keys
  - New `list_keys_raw()` and `get_key_bytes()` API methods for raw-byte key access
  - `format_key_bytes()` and `format_key_bytes_full()` helpers for consistent binary key display
  - `find_key_by_hex_suffix()` to locate a binary key from its truncated hex display (e.g. `get 61F8`)
- **`ls` alias** - `ls` now works as a shorthand for the `list` command, including tab completion
- **Usage error messages** - New `UsageError` command variant returns a descriptive message and usage hint when a known command is called with wrong or missing arguments
- **Improved database lock detection** - Cross-platform lock error recognition covering POSIX `EWOULDBLOCK`/`EAGAIN` and Windows `ERROR_LOCK_VIOLATION` (33) / `ERROR_SHARING_VIOLATION` (32), plus message-text fallback
- **User-friendly startup errors** - `startup_error_and_exit()` prints a clear, coloured message when the database cannot be opened (including a dedicated hint for lock conflicts)

### Changed
- History is now stored in memory (`MemHistory`) instead of on disk; entries are only recorded for commands that succeed or produce a non-usage error
- `list_keys()` now returns keys in sorted order
- Regex creation refactored into a shared `create_regex()` helper, eliminating duplicated glob-to-regex conversion across all query methods
- `load_keys()` and `load_trees()` no longer propagate errors (warnings printed instead), simplifying call-sites
- Updated dependency versions

### Fixed
- Database lock errors on Windows were previously not detected at all; they now produce the dedicated `DatabaseLocked` error with a clear remediation message
- `list` tab completion was not triggered for the `ls` alias

### Technical Improvements
- Applied Clippy lints throughout: `if let` over `match` for single-arm patterns, string interpolation (`format!("{x}")` → `format!("{x}")`), `#[must_use]` on pure functions, and `#[allow(clippy::unnecessary_wraps)]` where public API stability requires it
- `list_trees()` return type tightened to `Result<Vec<String>, SledoViewError>` (no longer erased via `anyhow`)
- `KeyInfo.key` documentation clarified to state the full/truncated hex convention for binary keys

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

[Unreleased]: https://github.com/sgchris/sledoview/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/sgchris/sledoview/compare/v1.0.3...v1.1.0
[1.0.3]: https://github.com/sgchris/sledoview/compare/v0.1.0...v1.0.3
[0.1.0]: https://github.com/sgchris/sledoview/releases/tag/v0.1.0
