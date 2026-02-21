# 📖 SledoView Documentation

SledoView is a powerful, interactive Command Line Interface (CLI) tool designed for viewing, exploring, and editing [SLED](https://github.com/spacejam/sled) databases. It provides a rich REPL (Read-Eval-Print Loop) environment with full CRUD operations and multi-tree support.

---

## 🛠️ 1. How to Install and Build

Since SledoView is a CLI tool written in Rust, you will need the Rust toolchain installed on your system to build it from source.

### Prerequisites
- **Rust 1.70+** (2021 edition)
- **Cargo** (included with Rust)

### Building from Source
To build the optimized release version of the tool, run the following command in the root of the project:
```bash
cargo build --release
```
The compiled executable will be available in the `target/release/` directory.

### Installing Locally
To install SledoView globally on your system so you can run it from anywhere in your terminal:
```bash
cargo install --path .
```
*(Alternatively, on Windows, you can use the provided development script: `.\scripts\dev.ps1 install`)*

---

## 🚀 2. How to Connect to the DB / How to Run

SledoView operates directly on the SLED database files on your disk. To connect to a database, simply pass the path to the database directory as an argument when launching the tool.

### Basic Usage
```bash
sledoview <path_to_database>
```

### Example
```bash
# Connect to a database located in the 'example_db' folder
sledoview ./example_db
```

Once connected, SledoView will open an interactive prompt (`sledoview>`) where you can start typing commands to interact with your data.

---

## 📚 3. Command Reference

SledoView provides a comprehensive set of commands to navigate and modify your database. 

### 🌳 Tree Management
SLED databases can contain multiple isolated keyspaces called "trees".
| Command | Description | Example |
|---------|-------------|---------|
| `trees [pattern]` | List all trees (supports glob patterns) | `trees` or `trees data_*` |
| `trees regex <regex>` | List trees matching a regular expression | `trees regex ^user_.*` |
| `select <tree>` | Switch context to a specific tree | `select settings` |
| `unselect` | Return to the default (root) tree | `unselect` |

### 🔑 Keys & Values (CRUD Operations)
These commands operate on the currently selected tree.
| Command | Description | Example |
|---------|-------------|---------|
| `list [pattern]` | List keys (supports glob patterns) | `list user_*` |
| `list regex <regex>` | List keys matching a regular expression | `list regex user_\d+` |
| `get <key>` | Retrieve the value and details of a specific key | `get user_001` |
| `set <key> <value>` | Create a new key or update an existing one | `set user_001 "John Doe"` |
| `delete <key>` | Delete a key-value pair | `delete user_001` |

### 🔍 Searching & Analytics
| Command | Description | Example |
|---------|-------------|---------|
| `search <pattern>` | Search for values matching a glob pattern | `search *@example.com` |
| `search regex <regex>` | Search for values matching a regex | `search regex \d{4}-\d{2}-\d{2}` |
| `count` | Show the total number of records in the current tree | `count` |

### ⚙️ General
| Command | Description | Example |
|---------|-------------|---------|
| `help` | Display the help menu with all available commands | `help` |
| `exit` | Safely close the database and exit SledoView | `exit` |

---
*Tip: SledoView supports colored terminal output and tab-completion to make your database exploration as smooth as possible!*
