use crate::commands::Command;
use crate::db::SledViewer;
use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Context, Editor};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Highlighter, Hinter, Validator)]
struct SledCompleter {
    keys: Vec<String>,
}

impl SledCompleter {
    fn new() -> Self {
        Self { keys: Vec::new() }
    }

    fn update_keys(&mut self, keys: Vec<String>) {
        self.keys = keys;
    }
}

impl rustyline::completion::Completer for SledCompleter {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let line_up_to_cursor = &line[..pos];
        
        // Parse the command to see if we can complete keys
        let parts: Vec<&str> = line_up_to_cursor.split_whitespace().collect();
        
        if parts.len() >= 2 {
            let command = parts[0].to_lowercase();
            if command == "get" || command == "delete" || command == "del" || 
               (command == "set" && parts.len() == 2) ||
               (command == "list" && parts.len() >= 2 && parts[1] != "regex") || 
               (command == "search" && parts.len() >= 2 && parts[1] != "regex") {
                // We're completing a key - find the current word being typed
                let current_word = if let Some(last_space) = line_up_to_cursor.rfind(' ') {
                    &line_up_to_cursor[last_space + 1..]
                } else {
                    ""
                };
                
                let mut candidates = Vec::new();
                for key in &self.keys {
                    if key.starts_with(current_word) {
                        candidates.push(rustyline::completion::Pair {
                            display: key.clone(),
                            replacement: key.clone(),
                        });
                    }
                }
                
                // Calculate the start position for replacement
                let start = if let Some(last_space) = line_up_to_cursor.rfind(' ') {
                    last_space + 1
                } else {
                    0
                };
                
                return Ok((start, candidates));
            }
        }
        
        // Fallback to command completion
        let commands = vec!["count", "list", "get", "set", "delete", "del", "search", "help", "exit", "quit"];
        let mut candidates = Vec::new();
        
        if let Some(word_start) = line_up_to_cursor.rfind(' ') {
            let word = &line_up_to_cursor[word_start + 1..];
            for cmd in commands {
                if cmd.starts_with(word) {
                    candidates.push(rustyline::completion::Pair {
                        display: cmd.to_string(),
                        replacement: cmd.to_string(),
                    });
                }
            }
            Ok((word_start + 1, candidates))
        } else {
            for cmd in commands {
                if cmd.starts_with(line_up_to_cursor) {
                    candidates.push(rustyline::completion::Pair {
                        display: cmd.to_string(),
                        replacement: cmd.to_string(),
                    });
                }
            }
            Ok((0, candidates))
        }
    }
}

pub struct Repl {
    editor: Editor<SledCompleter, FileHistory>,
    viewer: SledViewer,
    keys: Vec<String>,
}

impl Repl {
    pub fn new(viewer: SledViewer) -> Self {
        let mut editor = Editor::<SledCompleter, FileHistory>::new().expect("Failed to create readline editor");
        let completer = SledCompleter::new();
        editor.set_helper(Some(completer));
        
        Self { 
            editor, 
            viewer,
            keys: Vec::new(),
        }
    }

    fn load_keys(&mut self) -> Result<()> {
        match self.viewer.list_keys("*", false) {
            Ok(keys) => {
                self.keys = keys.clone();
                // Update the completer with new keys
                if let Some(helper) = self.editor.helper_mut() {
                    helper.update_keys(keys);
                }
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
            if command == "get" || command == "delete" || command == "del" || 
               (command == "set" && parts.len() == 2) ||
               (command == "list" && parts.len() >= 2 && parts[1] != "regex") || 
               (command == "search" && parts.len() >= 2 && parts[1] != "regex") {
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

    fn try_auto_complete(&self, line: &str) -> Option<String> {
        let completions = self.find_completions(line);
        
        // If there's exactly one completion, auto-complete it
        if completions.len() == 1 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(prefix) = parts.last() {
                if let Some(pos) = line.rfind(prefix) {
                    return Some(format!("{}{}", &line[..pos], &completions[0]));
                }
            }
        }
        
        None
    }

    fn should_show_completion_hint(&self, line: &str) -> bool {
        if line.trim().is_empty() {
            return false;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let command = parts[0].to_lowercase();
            if command == "get" || command == "delete" || command == "del" || command == "list" || command == "search" || 
               (command == "set" && parts.len() == 2) {
                let prefix = parts.last().copied().unwrap_or("");
                // Show hint if we have a partial key that could be completed
                return !prefix.is_empty() && self.keys.iter().any(|k| k.starts_with(prefix) && k != prefix);
            }
        }
        
        false
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
            "Use TAB for completion, type partial keys and TAB to auto-complete!".bright_black()
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

                    // Check for tab completion command
                    if line == "tab" || line == "\\t" {
                        println!("{}", "Tab completion: Type your partial command (e.g., 'get user_') and I'll complete it.".bright_blue());
                        continue;
                    }

                    // Check for completion command (keep this for manual completion)
                    if line.starts_with("complete ") {
                        let completion_line = &line[9..]; // Remove "complete "
                        self.show_completions(completion_line);
                        continue;
                    }

                    // Check for auto-completion opportunity
                    if self.should_show_completion_hint(line) {
                        if let Some(completed) = self.try_auto_complete(line) {
                            println!("{} {}", "Auto-completed:".bright_green(), completed.bright_white());
                            // Automatically execute the completed command
                            match Command::parse(&completed) {
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
                                        completed.bright_yellow()
                                    );
                                }
                            }
                            continue;
                        } else {
                            let completions = self.find_completions(line);
                            if !completions.is_empty() {
                                println!("{} {} {}. {}", 
                                    "Found".bright_blue(),
                                    completions.len().to_string().bright_yellow().bold(),
                                    "possible completions".bright_blue(),
                                    format!("Type 'complete {}' to see them.", line).yellow()
                                );
                                continue;
                            }
                        }
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
