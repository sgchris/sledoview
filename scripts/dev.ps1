# SledoView Development Script
# Usage: .\scripts\dev.ps1 <command>

param(
    [Parameter(Mandatory=$true)]
    [string]$Command
)

switch ($Command.ToLower()) {
    "test" {
        Write-Host "Running tests..." -ForegroundColor Green
        cargo test
    }
    "build" {
        Write-Host "Building release version..." -ForegroundColor Green
        cargo build --release
    }
    "dev-build" {
        Write-Host "Building development version..." -ForegroundColor Green
        cargo build
    }
    "check" {
        Write-Host "Running all checks..." -ForegroundColor Green
        Write-Host "1. Formatting..." -ForegroundColor Yellow
        cargo fmt --check
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Formatting check failed. Run 'cargo fmt' to fix." -ForegroundColor Red
            exit 1
        }
        
        Write-Host "2. Linting..." -ForegroundColor Yellow
        cargo clippy -- -D warnings
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Clippy check failed." -ForegroundColor Red
            exit 1
        }
        
        Write-Host "3. Testing..." -ForegroundColor Yellow
        cargo test
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Tests failed." -ForegroundColor Red
            exit 1
        }
        
        Write-Host "All checks passed!" -ForegroundColor Green
    }
    "format" {
        Write-Host "Formatting code..." -ForegroundColor Green
        cargo fmt
    }
    "lint" {
        Write-Host "Running clippy..." -ForegroundColor Green
        cargo clippy
    }
    "clean" {
        Write-Host "Cleaning build artifacts..." -ForegroundColor Green
        cargo clean
        if (Test-Path "example_db") {
            Remove-Item -Recurse -Force "example_db"
            Write-Host "Removed example database" -ForegroundColor Yellow
        }
    }
    "demo" {
        Write-Host "Creating example database and running demo..." -ForegroundColor Green
        cargo run --example create_test_db
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Example database created. You can now run:" -ForegroundColor Yellow
            Write-Host "  cargo run -- example_db" -ForegroundColor Cyan
            Write-Host "or" -ForegroundColor Yellow
            Write-Host "  .\target\release\sledoview.exe example_db" -ForegroundColor Cyan
        }
    }
    "install" {
        Write-Host "Installing sledoview..." -ForegroundColor Green
        cargo install --path .
    }
    "doc" {
        Write-Host "Generating documentation..." -ForegroundColor Green
        cargo doc --open
    }
    "coverage" {
        Write-Host "Running tests with coverage..." -ForegroundColor Green
        # Note: Requires cargo-tarpaulin: cargo install cargo-tarpaulin
        if (Get-Command cargo-tarpaulin -ErrorAction SilentlyContinue) {
            cargo tarpaulin --out Html
            Write-Host "Coverage report generated in tarpaulin-report.html" -ForegroundColor Green
        } else {
            Write-Host "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin" -ForegroundColor Red
        }
    }
    "help" {
        Write-Host "Available commands:" -ForegroundColor Green
        Write-Host "  test       - Run all tests" -ForegroundColor Yellow
        Write-Host "  build      - Build release version" -ForegroundColor Yellow
        Write-Host "  dev-build  - Build development version" -ForegroundColor Yellow
        Write-Host "  check      - Run format, lint, and test checks" -ForegroundColor Yellow
        Write-Host "  format     - Format code with rustfmt" -ForegroundColor Yellow
        Write-Host "  lint       - Run clippy linter" -ForegroundColor Yellow
        Write-Host "  clean      - Clean build artifacts" -ForegroundColor Yellow
        Write-Host "  demo       - Create example DB and show run instructions" -ForegroundColor Yellow
        Write-Host "  install    - Install sledoview binary" -ForegroundColor Yellow
        Write-Host "  doc        - Generate and open documentation" -ForegroundColor Yellow
        Write-Host "  coverage   - Run tests with coverage (requires cargo-tarpaulin)" -ForegroundColor Yellow
        Write-Host "  help       - Show this help message" -ForegroundColor Yellow
    }
    default {
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Write-Host "Run '.\scripts\dev.ps1 help' for available commands" -ForegroundColor Yellow
        exit 1
    }
}
