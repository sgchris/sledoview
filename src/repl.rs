use crate::commands::Command;
use crate::db::SledViewer;
use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub struct Repl {
    editor: DefaultEditor,
    viewer: SledViewer,
}

impl Repl {
    pub fn new(viewer: SledViewer) -> Self {
        let editor = DefaultEditor::new().expect("Failed to create readline editor");
        Self { editor, viewer }
    }

    pub fn run(&mut self) -> Result<()> {
        println!();
        println!(
            "{}",
            "Interactive SLED Database Viewer".bright_cyan().bold()
        );
        println!(
            "{}",
            "Type 'help' for available commands or 'exit' to quit.".bright_black()
        );
        println!();

        loop {
            let readline = self
                .editor
                .readline(&format!("{} ", ">".bright_green().bold()));

            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    if let Err(e) = self.editor.add_history_entry(line) {
                        eprintln!("Warning: Failed to add to history: {e}");
                    }

                    match Command::parse(line) {
                        Some(Command::Exit) => {
                            println!("{}", "Goodbye!".bright_green());
                            break;
                        }
                        Some(command) => {
                            if let Err(e) = command.execute(&self.viewer) {
                                println!(
                                    "{} {}",
                                    "Error:".bright_red().bold(),
                                    e.to_string().red()
                                );
                            }
                        }
                        None => {
                            println!(
                                "{} Unknown command: '{}'. Type 'help' for available commands.",
                                "Error:".bright_red().bold(),
                                line.bright_yellow()
                            );
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "Use 'exit' to quit.".bright_yellow());
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye!".bright_green());
                    break;
                }
                Err(err) => {
                    println!("{} {}", "Error:".bright_red().bold(), err);
                    break;
                }
            }
        }

        Ok(())
    }
}
