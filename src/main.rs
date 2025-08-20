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
