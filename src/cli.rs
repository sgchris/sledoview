use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "sledoview",
    about = "A CLI tool for viewing and managing SLED databases",
    version,
    author = "Your Name <your.email@example.com>"
)]
pub struct Args {
    /// Path to the SLED database file
    #[arg(help = "Path to the SLED database file")]
    pub database_path: PathBuf,
}
