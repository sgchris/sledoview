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
    validator.validate()?;

    // Open the database
    let viewer = SledViewer::new(&args.database_path)?;

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
