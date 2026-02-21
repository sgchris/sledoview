# SledoView Roadmap

This roadmap outlines the planned features and enhancements for SledoView. Our goal is to make SledoView the absolute best tool for interacting with Sled databases, offering features that go beyond standard CLI viewers and provide a massive advantage over generic database tools.

## 🧠 Smart Data Formatting & Serialization
*Sled stores raw bytes, which can be hard to read. SledoView should be smart enough to understand your data structures.*
- **Pluggable Deserializers**: Add support for interpreting values as JSON, Bincode, MessagePack, or Protobuf. 
  *Example: `set-format bincode` or `set-format json` to automatically deserialize bytes into readable text.*
- **Pretty Printing**: Automatically format and syntax-highlight JSON or structured data directly in the terminal.

## 🌐 Embedded Web UI
*The speed of a CLI with the convenience of a GUI, all in one binary.*
- **`serve` Command**: Run `sledoview serve --port 8080` to spin up a lightweight, embedded web server.
- **Visual Browser**: A beautiful, responsive local web interface to browse trees, search keys, and edit values visually.
- **Zero Dependencies**: Serve the UI directly from the Rust binary using embedded assets (HTML/JS/CSS), requiring no extra installations.

## 🔄 Data Portability & Migrations
*Make moving data in and out of Sled effortless.*
- **Import/Export**: Commands to export a specific tree to JSON/CSV and import it back.
  *Example: `export users users.json` and `import users users.json`.*
- **Native Backups**: Support for Sled's native `export()` and `import()` functions for full database snapshots and disaster recovery.
- **Tree Cloning**: Easily duplicate, rename, or merge trees within the same database.

## 🔍 Advanced Querying & Batching
*Go beyond simple key matching and manage data at scale.*
- **JSON Path Queries**: If the data format is set to JSON, allow querying by specific fields.
  *Example: `find .user.age > 18`.*
- **Batch Operations**: Commands like `delete-all <pattern>` to remove multiple keys at once without writing a custom script.

## 📊 Analytics & Health
*Understand your database performance, storage, and bottlenecks.*
- **`stats` Command**: Display database size, fragmentation percentage, and total key count.
- **Tree Size Estimation**: Show which trees are consuming the most disk space to help identify bloat.
- **Data Distribution**: Analyze and display the average size of keys and values to help developers optimize their storage patterns.
