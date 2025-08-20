use crate::commands::Command;
use crate::db::SledViewer;
use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;

pub struct Repl {
    editor: Editor<(), FileHistory>,
    viewer: SledViewer,
    keys: Vec<String>,
}

impl Repl {
    pub fn new(viewer: SledViewer) -> Self {
        let editor = Editor::<(), FileHistory>::new().expect("Failed to create readline editor");
        
        Self { 
            editor, 
            viewer,
            keys: Vec::new(),
        }
    }

    fn load_keys(&mut self) -> Result<()> {
        match self.viewer.list_keys("*", false) {
            Ok(keys) => {
                self.keys = keys;
                Ok(())
            }
            Err(e) => {
                eprintln!("Warning: Failed to load keys for completion: {}", e);
                Ok(())
            }
        }
    }

    fn find_completions(&self, line: &str) -> Vec<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 2 {
            let command = parts[0].to_lowercase();
            if command == "get" || (command == "list" && parts.len() >= 2 && parts[1] != "regex") || (command == "search" && parts.len() >= 2 && parts[1] != "regex") {
                // Find the current word being typed
                let prefix = parts.last().copied().unwrap_or("");
                
                let mut candidates = Vec::new();
                for key in &self.keys {
                    if key.starts_with(prefix) {
                        candidates.push(key.clone());
                    }
                }
                
                return candidates;
            }
        }
        
        Vec::new()
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
        println!(
            "{}",
            "Use 'complete <command>' for key completions, Ctrl-C to cancel current line.".bright_black()
        );
        println!();

        // Load keys for completion
        self.load_keys()?;

        loop {
            let readline = self.editor.readline("> ");

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

                    // Check for completion command (keep this for manual completion)
                    if line.starts_with("complete ") {
                        let completion_line = &line[9..]; // Remove "complete "
                        self.show_completions(completion_line);
                        continue;
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
                            // Reload keys after any command in case database changed
                            self.load_keys()?;
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
                    // Check if the line was empty (exit) or had content (cancel)
                    // For now, we'll just show a message and continue - rustyline
                    // doesn't give us access to the current line content on interrupt
                    println!("^C");
                    println!("{}", "Use 'exit' or Ctrl-D to quit.".bright_black());
                    continue;
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

    fn show_completions(&self, line: &str) {
        let completions = self.find_completions(line);
        if completions.is_empty() {
            println!("{}", "No completions available for this context.".yellow());
            return;
        }

        println!("{} {} {}:", 
            "Found".bright_blue(),
            completions.len().to_string().bright_yellow().bold(),
            "possible completions".bright_blue()
        );
        
        for (i, completion) in completions.iter().enumerate() {
            println!("  {}: {}", 
                (i + 1).to_string().bright_black(),
                completion.bright_white()
            );
        }
        
        if completions.len() == 1 {
            // Auto-complete if there's only one match
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(prefix) = parts.last() {
                if let Some(pos) = line.rfind(prefix) {
                    let completed = format!("{}{}", &line[..pos], &completions[0]);
                    println!("{} {}", "Auto-completed:".bright_green(), completed.bright_white());
                }
            }
        }
    }
}
