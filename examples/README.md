# Example Database Generators

This directory contains examples that generate sample SLED databases for testing and demonstrating sledoview functionality.

## Available Examples

### create_test_db.rs
Creates a basic SLED database with sample user data, configuration settings, and session information. This database uses only the default tree.

**Usage:**
```bash
cargo run --example create_test_db
```

**Output:** Creates `example_db/` directory with a simple SLED database containing 15 records.

### create_sample_with_trees.rs
Creates a comprehensive SLED database with multiple named trees, demonstrating advanced database organization and various data types.

**Usage:**
```bash
cargo run --example create_sample_with_trees
```

**Output:** Creates `sample_with_trees.db/` directory with a complex multi-tree database.

## Database Structure (create_sample_with_trees.rs)

The multi-tree database contains the following trees:

### Default Tree (6 entries)
- Basic user data (`user:001` to `user:005`)
- Administrative account (`admin:root`)

### Named Trees

#### settings (10 entries)
Application configuration and preferences:
- `app.theme`, `app.language`, `app.timeout`
- `ui.sidebar_width`, `ui.show_tooltips`
- `security.session_timeout`, `security.max_attempts`
- `db.backup_interval`
- `logging.level`, `logging.file_rotation`

#### sessions (5 entries)
User session data with JSON structures:
- Session tokens with user IDs, timestamps, and IP addresses
- Example: `{"user_id": "001", "created": "2024-01-01T10:00:00Z", "ip": "192.168.1.100"}`

#### cache (9 entries)
Temporary cached data and API responses:
- Weather data for different cities (JSON format)
- GitHub API responses
- Computed values (factorial, fibonacci)
- Binary data (PNG headers)
- Processing status indicators

#### logs (10 entries)
Application logs with timestamps:
- INFO, DEBUG, WARN, and ERROR level messages
- System events and user activities
- Timestamps in ISO 8601 format

#### metrics (12 entries)
Performance metrics and statistics:
- CPU and memory usage over time
- Request and error counts
- Response time statistics (average, P95)

#### configuration (12 entries)
Structured configuration parameters:
- Database connection settings
- Server configuration
- External service URLs
- Feature flags

#### binary_data (9 entries)
Various binary formats and data types:
- File format headers (PNG, JPEG, PDF)
- Binary sequences (0-255 bytes)
- Encrypted-looking data
- UUIDs and hash values (MD5, SHA-256)

#### test_data (13 entries)
Edge cases and test scenarios:
- Empty keys and values
- Unicode strings with emojis and international characters
- Very long keys and values
- JSON, XML, and malformed data
- Numbers in various formats (int, float, scientific, hex, binary)

## Testing with sledoview

After generating the sample databases, you can test them with sledoview:

```bash
# Test the basic database
cargo run -- example_db

# Test the multi-tree database
cargo run -- sample_with_trees.db
```

### Example Commands

Once in the sledoview interactive shell, try these commands:

```bash
# List all keys
list

# Search for specific patterns
search user*
search *.json
search regex \d{4}-\d{2}-\d{2}

# Get specific values
get user:001
get app.theme

# Count total records
count

# View help
help
```

## Purpose and Use Cases

These example databases are designed to:

1. **Test sledoview functionality** - Verify that sledoview correctly handles various data types and tree structures
2. **Demonstrate SLED features** - Show how to use named trees for data organization
3. **Provide realistic test data** - Include common patterns like logs, configuration, sessions, and metrics
4. **Test edge cases** - Include problematic data like empty values, unicode characters, and binary data
5. **Performance testing** - Provide datasets of various sizes for performance evaluation

## Data Types Included

The sample databases include:

- **Text data**: User names, configuration values, log messages
- **JSON structures**: API responses, session data, weather information
- **Binary data**: File headers, encrypted data, hash values
- **Timestamps**: ISO 8601 formatted dates and times
- **Numbers**: Integers, floats, scientific notation, hex, binary
- **Unicode**: International characters and emojis
- **Edge cases**: Empty strings, null bytes, very long strings

## Notes

- All databases are created in the current directory
- Existing databases with the same name will be overwritten
- The databases use SLED's default configuration
- All data is flushed to disk for persistence
- Binary data is stored as byte arrays
- Large byte arrays (>32 bytes) are converted to slices for SLED compatibility