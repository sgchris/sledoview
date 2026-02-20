mod cli;
mod commands;
mod db;
mod error;
mod repl;
mod validator;

use anyhow::Result;
use clap::Parser;
use colored::*;

use cli::Args;
use db::SledViewer;
use error::SledoViewError;
use repl::Repl;
use validator::DatabaseValidator;

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "{}",
        "SledoView - SLED Database Viewer".bright_cyan().bold()
    );
    println!("{}", "═".repeat(35).bright_cyan());

    // Special case for creating test data
    if args.database_path.to_string_lossy() == "create-test" {
        create_test_database()?;
        return Ok(());
    }

    // Validate the database
    let validator = DatabaseValidator::new(&args.database_path);
    if let Err(e) = validator.validate() {
        startup_error_and_exit(e);
    }

    // Open the database
    let viewer = match SledViewer::new(&args.database_path) {
        Ok(v) => v,
        Err(e) => startup_error_and_exit(e),
    };

    // Check if database is writable
    if !viewer.is_writable() {
        println!(
            "{} {}",
            "⚠".bright_yellow().bold(),
            "Database opened in read-only mode (write operations will fail)".bright_yellow()
        );
    }

    println!(
        "{} {}",
        "✓".bright_green().bold(),
        format!(
            "Successfully opened database: {}",
            args.database_path.display()
        )
        .bright_green()
    );

    // Start the REPL
    let mut repl = Repl::new(viewer);
    repl.run()?;

    Ok(())
}

/// Print a user-friendly error message for startup failures, then exit.
fn startup_error_and_exit(err: anyhow::Error) -> ! {
    // Check for the DatabaseLocked variant specifically.
    let is_locked = err
        .downcast_ref::<SledoViewError>()
        .map(|e| matches!(e, SledoViewError::DatabaseLocked { .. }))
        .unwrap_or(false);

    if is_locked {
        eprintln!(
            "\n{} {}",
            "✗  Database is locked.".bright_red().bold(),
            "Another process has it open.".red()
        );
        eprintln!(
            "   {}",
            "Close the other application and try again.".yellow()
        );
    } else {
        eprintln!(
            "\n{} {}",
            "✗  Failed to open database:".bright_red().bold(),
            err.to_string().red()
        );
    }

    std::process::exit(1);
}

fn create_test_database() -> Result<()> {
    println!("Creating test database...");
    let db = sled::open("test.db")?;
    
    db.insert("user_1", "Alice Smith")?;
    db.insert("user_2", "Bob Johnson")?;
    db.insert("config_timeout", "30")?;
    db.insert("config_debug", "true")?;
    db.insert("data_large", "This is a longer text value that should be truncated in the preview display to demonstrate the truncation feature")?;
    db.insert("empty_key", "")?;
    
    // Add some binary data
    db.insert("binary_data", &[0u8, 1u8, 2u8, 255u8])?;
    
    db.flush()?;
    println!("✓ Test database 'test.db' created successfully!");
    println!("You can now run: {} {}", "cargo run test.db".bright_yellow(), "".bright_green());
    Ok(())
}
