# SledoView Downloads

This folder contains the pre-built executables for different platforms.

## Files

- `sledoview.exe` - Windows executable (x86_64)
- `sledoview` - Linux executable (x86_64)
- macOS executable (to be added later)

## Building Executables

To build the executables from source:

### Windows
```bash
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/sledoview.exe website/downloads/
```

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/sledoview website/downloads/
```

### macOS
```bash
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/sledoview website/downloads/sledoview-macos
```

## Website Integration

The website's download buttons link directly to these files:

- Windows users download `sledoview.exe`
- Linux users download `sledoview`
- macOS users currently get the Linux version (temporary)

## File Sizes

Typical file sizes after building:
- Windows: ~8-12 MB
- Linux: ~8-12 MB
- macOS: ~8-12 MB

## Cross-compilation

You may need to install the target toolchains:

```bash
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
```

For Windows cross-compilation on Linux, you may also need:
```bash
sudo apt-get install gcc-mingw-w64-x86-64
```
