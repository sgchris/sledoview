# Development Scripts for SledoView

This directory contains helpful scripts for development and testing.

## Available Scripts

### PowerShell Scripts (Windows)

- `dev.ps1` - Main development script with multiple commands
- `test-db.ps1` - Create and test with example database

### Usage

```powershell
# Run all tests
.\scripts\dev.ps1 test

# Build release version
.\scripts\dev.ps1 build

# Create test database and run app
.\scripts\dev.ps1 demo

# Run all checks (format, lint, test)
.\scripts\dev.ps1 check

# Clean build artifacts
.\scripts\dev.ps1 clean
```
