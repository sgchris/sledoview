# SledoView

[![Crates.io](https://img.shields.io/crates/v/sledoview.svg)](https://crates.io/crates/sledoview)
[![Documentation](https://docs.rs/sledoview/badge.svg)](https://docs.rs/sledoview)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/yourusername/sledoview)

A powerful CLI tool for viewing and managing SLED databases with an interactive terminal interface.

## Features

- 🔍 **Interactive REPL** - Browse your SLED database with a user-friendly terminal interface
- 📊 **Database Statistics** - Get total record counts and key information
- 🔎 **Pattern Matching** - Search keys and values using glob patterns or regular expressions
- ✏️ **Database Modification** - Set and delete key-value pairs with proper validation
- 🎨 **Colorized Output** - Beautiful, colored terminal output for better readability
- � **Write Operations** - Safely modify database contents with immediate persistence
- ✅ **Comprehensive Validation** - Thorough database file validation and key validation
- 🧪 **Well Tested** - Extensive test suite ensuring reliability

## Installation

### From Crates.io

```bash
cargo install sledoview
```

### From Source

```bash
git clone https://github.com/yourusername/sledoview
cd sledoview
cargo install --path .
```

## Usage

### Basic Usage

```bash
sledoview /path/to/your/sled.db
```

Upon successful validation and opening, you'll see:

```
SledoView - SLED Database Viewer
═══════════════════════════════════
Validating database...
✓ Database validation passed
✓ Successfully opened database: /path/to/your/sled.db
✓ Database is writable - modification commands available

Interactive SLED Database Viewer
Type 'help' for available commands or 'exit' to quit.

> 
```

### Available Commands

#### `count`
Display the total number of records in the database.

```bash
> count
Total records: 1,532
```

#### `list [pattern]`
List all keys matching the specified pattern. Uses glob pattern matching by default.

**Examples:**
```bash
# List all keys
> list
> list *

# List keys starting with "user_"
> list user_*

# List keys ending with "_config"
> list *_config

# List keys containing "session"
> list *session*
```

#### `list regex <pattern>`
List keys matching a regular expression pattern.

**Examples:**
```bash
# List keys matching a regex pattern
> list regex user_\d+

# List keys with specific format
> list regex ^config_[a-z]+$

# List keys containing numbers
> list regex .*\d.*
```

#### `set <key> <value>`
Set or update a key-value pair in the database. The operation will be immediately persisted to disk.

**Key Validation Rules:**
- Must contain only alphanumeric characters, `_`, `-`, `.`, `:`, `/`, and spaces
- Maximum length of 512 characters
- Cannot be empty

**Quoting Support:**
- Use double quotes to include spaces in keys or values
- Escape quotes within quoted strings with backslash: `\"`
- Examples of valid quoted usage:
  - `"key with spaces"` 
  - `"value with \"quotes\""`
  - `"path/to/config"`

**Examples:**
```bash
# Set a simple key-value pair
> set user_name "John Doe"
✓ Successfully set key 'user_name'

# Set a key with spaces (using quotes)
> set "user settings" "{'theme': 'dark', 'lang': 'en'}"
✓ Successfully set key 'user settings'

# Update an existing key
> set config_timeout 3600
✓ Successfully set key 'config_timeout'

# Set a complex value with quotes
> set message "He said \"Hello, World!\""
✓ Successfully set key 'message'
```

#### `delete <key>`
Delete a key from the database. The operation will be immediately persisted to disk.

**Examples:**
```bash
# Delete a simple key
> delete user_temp
✓ Successfully deleted key 'user_temp'

# Delete a key with spaces (using quotes)  
> delete "temporary setting"
✓ Successfully deleted key 'temporary setting'

# Try to delete a non-existent key
> delete nonexistent
✗ Key 'nonexistent' not found
```
Retrieve detailed information about a specific key, including its value, size, and UTF-8 validity.

**Examples:**
```bash
> get user_123
═══════════════════════════════════════════════════════
Key: user_123
Size: 45 bytes
UTF-8: Yes
Value:
───────────────────────────────────────────────────────
{"name": "John Doe", "email": "john@example.com"}
═══════════════════════════════════════════════════════

> get config_settings
═══════════════════════════════════════════════════════
Key: config_settings  
Size: 156 bytes
UTF-8: Yes
Value:
───────────────────────────────────────────────────────
{"theme": "dark", "language": "en-US", "timeout": 3600}
═══════════════════════════════════════════════════════
```

#### `search <pattern>`
Search for entries where the **value** matches the specified pattern.

**Examples:**
```bash
# Search for values containing email addresses
> search *@example.com

# Search for values containing specific text
> search *John*

# Search for JSON values containing specific fields
> search *"theme"*
```

#### `search regex <pattern>`
Search for entries where the **value** matches a regular expression.

**Examples:**
```bash
# Search for email addresses using regex
> search regex \w+@\w+\.\w+

# Search for dates in YYYY-MM-DD format
> search regex \d{4}-\d{2}-\d{2}

# Search for JSON objects with specific structure
> search regex \{"name":\s*"[^"]+".*\}
```

#### `help`
Display the help message with all available commands.

#### `exit` / `quit` / `q`
Exit the application.

## Database Validation

SledoView performs comprehensive validation before opening a database:

- ✅ **File Existence** - Verifies the database path exists
- ✅ **Directory Structure** - Ensures it's a directory (SLED databases are directories)
- ✅ **SLED Format** - Validates the internal structure contains SLED-specific files
- ✅ **Read Permissions** - Checks file system permissions
- ✅ **Lock Status** - Ensures the database isn't locked by another process

## Output Examples

### Successful Database Opening
```
SledoView - SLED Database Viewer
═══════════════════════════════════
Validating database...
✓ Database validation passed
✓ Successfully opened database: /home/user/myapp.db
✓ Database is writable - modification commands available
```

### Command Examples
```bash
> count
Total records: 150

> list user_*
Found 3 keys:
  1: user_001
  2: user_002  
  3: user_admin

> set new_user "Alice Smith"
✓ Successfully set key 'new_user'

> get new_user
═══════════════════════════════════════════════════════
Key: new_user
Size: 11 bytes
UTF-8: Yes
Value:
───────────────────────────────────────────────────────
Alice Smith
═══════════════════════════════════════════════════════

> delete temp_key
✓ Successfully deleted key 'temp_key'

> search *@gmail.com
Found 5 matches:
  1: email_john => john.doe@gmail.com
  2: email_mary => mary.smith@gmail.com
  3: contact_primary => support@gmail.com
  4: backup_email => backup@gmail.com
  5: user_email_005 => user005@gmail.com
```

## Error Handling

SledoView provides clear, colored error messages for various scenarios:

- ❌ **Database not found**: Clear message when the specified path doesn't exist
- ❌ **Invalid SLED database**: Helpful guidance when the directory isn't a valid SLED database
- ❌ **Permission denied**: Clear indication of permission issues
- ❌ **Database locked**: Informative message when another process has locked the database
- ❌ **Invalid regex**: Helpful error messages for malformed regular expressions
- ❌ **Invalid key**: Clear validation messages for keys that don't meet requirements
- ❌ **Write permission errors**: Informative messages when write operations fail

## Development

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo

### Building

```bash
git clone https://github.com/yourusername/sledoview
cd sledoview
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Test Coverage

The project includes comprehensive tests covering:

- Database validation logic
- All CLI commands and their variations (including set/delete operations)
- Pattern matching (both glob and regex)
- Quote parsing and argument handling
- Key validation logic
- Write operations and persistence
- Error handling scenarios
- Binary data handling
- Edge cases (empty databases, non-existent keys, etc.)

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Guidelines

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Write tests** for your changes
4. **Ensure all tests pass** (`cargo test`)
5. **Follow Rust formatting conventions** (`cargo fmt`)
6. **Check for linting issues** (`cargo clippy`)
7. **Commit your changes** (`git commit -m 'Add some amazing feature'`)
8. **Push to the branch** (`git push origin feature/amazing-feature`)
9. **Open a Pull Request**

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Ensure code passes Clippy lints (`cargo clippy`)
- Add documentation for public APIs
- Write tests for new functionality

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- [SLED](https://github.com/spacejam/sled) - The embedded database that makes this tool possible
- [Rustyline](https://github.com/kkawakam/rustyline) - For the excellent REPL functionality
- [Colored](https://github.com/mackwic/colored) - For beautiful terminal colors

## Changelog

### v0.1.0
- Initial release
- Basic SLED database viewing functionality
- Set and delete operations with quote parsing
- Interactive REPL with colored output and TAB completion
- Pattern matching for keys and values
- Key validation and write permission checking
- Comprehensive database validation
- Full test suite with unit and integration tests
