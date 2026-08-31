# SledoView

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/sgchris/sledoview)

A powerful console application for performing CRUD operations on SLED database files through an interactive terminal interface.

## Features

- 🔍 **Interactive REPL** - Browse your SLED database with a user-friendly terminal interface
- 🌳 **Sled Tree Management** - Work with trees
- 🔎 **Pattern Matching** - Search keys, values, and trees using glob patterns or regular expressions
- ✏️ **CRUD Operations** - Create, Read, Update, and Delete key-value pairs in any tree

## Example

- Connect
- list trees
- select a tree
- create a new key-value pair
- find the new key
- get the value

<img width="652" height="703" alt="image" src="https://github.com/user-attachments/assets/3ff710c7-d336-41db-95af-16aa6e4c9da5" />

## Installation

### From Source

```bash
git clone https://github.com/sgchris/sledoview
cd sledoview
cargo build --release
```

## Usage

### Basic Usage

```bash
sledoview /path/to/your/sled.db
```

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

---
- [SLED database](https://github.com/spacejam/sled) - The embedded database that makes this tool possible


<!-- Security scan triggered at 2026-08-31 17:21:13 -->