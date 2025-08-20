# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- MIT OR Apache-2.0 dual licensing
- Contribution guidelines
- Example database creation script
- Full command reference with examples

[Unreleased]: https://github.com/yourusername/sledoview/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/sledoview/releases/tag/v0.1.0
