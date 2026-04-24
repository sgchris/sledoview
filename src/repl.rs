use crate::command_input::{command_names, completion_request, CompletionKind};
use crate::commands::Command;
use crate::db::SledViewer;
use crate::error::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::history::MemHistory;
use rustyline::{Context, Editor};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};
use std::cell::RefCell;
use std::rc::Rc;

const COMPLETION_LIMIT: usize = 100;

#[derive(Helper, Highlighter, Hinter, Validator)]
struct SledCompleter {
    viewer: Rc<RefCell<SledViewer>>,
    limit: usize,
}

impl SledCompleter {
    fn new(viewer: Rc<RefCell<SledViewer>>, limit: usize) -> Self {
        Self { viewer, limit }
    }

    fn find_completions(&self, line: &str) -> Vec<String> {
        let Some(request) = completion_request(line) else {
            return Vec::new();
        };

        match request.kind {
            CompletionKind::Command => Self::command_completions(&request.prefix),
            CompletionKind::Key => self
                .viewer
                .borrow()
                .complete_keys(&request.prefix, self.limit)
                .unwrap_or_default(),
            CompletionKind::Tree => self
                .viewer
                .borrow()
                .complete_trees(&request.prefix, self.limit)
                .unwrap_or_default(),
        }
    }

    fn command_completions(prefix: &str) -> Vec<String> {
        command_names()
            .iter()
            .copied()
            .filter(|command| command.starts_with(prefix))
            .map(str::to_string)
            .collect()
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
        let start = line_up_to_cursor
            .rfind([' ', '\t'])
            .map_or(0, |index| index + 1);
        let candidates = self
            .find_completions(line_up_to_cursor)
            .into_iter()
            .map(|completion| rustyline::completion::Pair {
                display: completion.clone(),
                replacement: completion,
            })
            .collect();

        Ok((start, candidates))
    }
}

pub struct Repl {
    editor: Editor<SledCompleter, MemHistory>,
    viewer: Rc<RefCell<SledViewer>>,
}

impl Repl {
    #[must_use]
    pub fn new(viewer: SledViewer) -> Self {
        let viewer = Rc::new(RefCell::new(viewer));
        let mut editor = Editor::<SledCompleter, MemHistory>::with_history(
            rustyline::Config::default(),
            MemHistory::new(),
        )
        .expect("Failed to create readline editor");
        let completer = SledCompleter::new(Rc::clone(&viewer), COMPLETION_LIMIT);
        editor.set_helper(Some(completer));

        Self { editor, viewer }
    }

    fn find_completions(&self, line: &str) -> Vec<String> {
        self.editor
            .helper()
            .map_or_else(Vec::new, |helper| helper.find_completions(line))
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
        let prefix = line
            .trim_end_matches([' ', '\t'])
            .split_whitespace()
            .last()
            .unwrap_or("");

        if prefix.is_empty() {
            return false;
        }

        self.find_completions(line)
            .iter()
            .any(|completion| completion != prefix)
    }

    #[allow(clippy::unnecessary_wraps)] // avoid breaking public API
    pub fn run(&mut self) -> Result<()> {
        print_intro();

        loop {
            // Create prompt that shows selected tree
            let prompt = match self.viewer.borrow().get_selected_tree() {
                Some(tree) => format!("[{tree}]> "),
                None => "> ".to_string(),
            };

            let readline = self.editor.readline(&prompt);

            match readline {
                Ok(line_raw) => {
                    let line = line_raw.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Check for tab completion command
                    if line == "tab" || line == "\\t" {
                        println!("{}", "Tab completion: Type your partial command (e.g., 'get user_') and I'll complete it.".bright_blue());
                        continue;
                    }

                    // Check for completion command (keep this for manual completion)
                    if let Some(completion_line) = line.strip_prefix("complete ") {
                        // Remove "complete "
                        self.show_completions(completion_line);
                        continue;
                    }

                    // Check for auto-completion opportunity
                    if self.should_show_completion_hint(line) {
                        if let Some(completed) = self.try_auto_complete(line) {
                            println!(
                                "{} {}",
                                "Auto-completed:".bright_green(),
                                completed.bright_white()
                            );
                            // Automatically execute the completed command
                            match Command::parse(&completed) {
                                Some(Command::Exit) => {
                                    println!("{}", "Goodbye!".bright_green());
                                    break;
                                }
                                Some(command) => {
                                    if let Err(e) = command.execute(&mut self.viewer.borrow_mut()) {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    } else if !command.is_usage_error() {
                                        let _ = self.editor.add_history_entry(&completed);
                                    }
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
                        }

                        // Couldn't find *one single* completion to use
                        let completions = self.find_completions(line);
                        if !completions.is_empty() {
                            println!(
                                "{} {} {}. {}",
                                "Found".bright_blue(),
                                completions.len().to_string().bright_yellow().bold(),
                                "possible completions".bright_blue(),
                                format!("Type 'complete {line}' to see them.").yellow()
                            );
                            continue;
                        }
                    }

                    match Command::parse(line) {
                        Some(Command::Exit) => {
                            println!("{}", "Goodbye!".bright_green());
                            break;
                        }
                        Some(command) => {
                            if let Err(e) = command.execute(&mut self.viewer.borrow_mut()) {
                                println!(
                                    "{} {}",
                                    "Error:".bright_red().bold(),
                                    e.to_string().red()
                                );
                            } else if !command.is_usage_error() {
                                let _ = self.editor.add_history_entry(line);
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
                    // Check if the line was empty (exit) or had content (cancel)
                    // For now, we'll just show a message and continue - rustyline
                    // doesn't give us access to the current line content on interrupt
                    println!("^C");
                    println!("{}", "Use 'exit' or Ctrl-D to quit.".bright_black());
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

        println!(
            "{} {} {}:",
            "Found".bright_blue(),
            completions.len().to_string().bright_yellow().bold(),
            "possible completions".bright_blue()
        );

        for (i, completion) in completions.iter().enumerate() {
            println!(
                "  {}: {}",
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
                    println!(
                        "{} {}",
                        "Auto-completed:".bright_green(),
                        completed.bright_white()
                    );
                }
            }
        }
    }
}

fn print_intro() {
    println!();
    println!(
        "{}",
        "Interactive SLED Database Client".bright_cyan().bold()
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
}
