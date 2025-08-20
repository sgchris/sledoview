use crate::db::{KeyInfo, SledViewer};
use anyhow::Result;
use colored::*;

pub enum Command {
    Count,
    List { pattern: String, is_regex: bool },
    Get { key: String },
    Search { pattern: String, is_regex: bool },
    Help,
    Exit,
}

impl Command {
    pub fn parse(input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return None;
        }

        match parts[0].to_lowercase().as_str() {
            "count" => Some(Command::Count),
            "list" => {
                if parts.len() == 1 {
                    Some(Command::List {
                        pattern: "*".to_string(),
                        is_regex: false,
                    })
                } else if parts.len() == 2 {
                    Some(Command::List {
                        pattern: parts[1].to_string(),
                        is_regex: false,
                    })
                } else if parts.len() == 3 && parts[1] == "regex" {
                    Some(Command::List {
                        pattern: parts[2].to_string(),
                        is_regex: true,
                    })
                } else {
                    None
                }
            }
            "get" => {
                if parts.len() >= 2 {
                    Some(Command::Get {
                        key: parts[1..].join(" "),
                    })
                } else {
                    None
                }
            }
            "search" => {
                if parts.len() == 1 {
                    None
                } else if parts.len() == 2 {
                    Some(Command::Search {
                        pattern: parts[1].to_string(),
                        is_regex: false,
                    })
                } else if parts.len() == 3 && parts[1] == "regex" {
                    Some(Command::Search {
                        pattern: parts[2].to_string(),
                        is_regex: true,
                    })
                } else {
                    None
                }
            }
            "help" | "?" => Some(Command::Help),
            "exit" | "quit" | "q" => Some(Command::Exit),
            _ => None,
        }
    }

    fn format_value_preview(info: &KeyInfo) -> String {
        if !info.is_utf8 {
            return "(binary data)".red().to_string();
        }

        if info.value.is_empty() {
            return "(empty)".bright_black().to_string();
        }

        // For short values, show them fully
        if info.value.len() <= 50 {
            return info.value.bright_green().to_string();
        }

        // For longer values, show a preview with truncation
        let preview = info.value.chars().take(47).collect::<String>();
        format!("{}...", preview).bright_green().to_string()
    }

    pub fn execute(&self, viewer: &SledViewer) -> Result<()> {
        match self {
            Command::Count => {
                let count = viewer.count()?;
                println!(
                    "{} {}",
                    "Total records:".bright_blue().bold(),
                    count.to_string().bright_yellow().bold()
                );
            }
            Command::List { pattern, is_regex } => {
                let keys = viewer.list_keys(pattern, *is_regex)?;
                if keys.is_empty() {
                    println!("{}", "No keys found matching the pattern.".yellow());
                } else {
                    let total_count = keys.len();
                    let display_keys = if total_count > 50 {
                        &keys[0..50]
                    } else {
                        &keys
                    };
                    
                    println!(
                        "{} {} {}",
                        "Found".bright_blue(),
                        total_count.to_string().bright_yellow().bold(),
                        "keys:".bright_blue()
                    );
                    
                    for (i, key) in display_keys.iter().enumerate() {
                        // Get value preview for each key
                        match viewer.get_key(key) {
                            Ok(info) => {
                                let preview = Self::format_value_preview(&info);
                                println!(
                                    "  {}: {} = {}",
                                    (i + 1).to_string().bright_black(),
                                    key.bright_white(),
                                    preview
                                );
                            }
                            Err(_) => {
                                // Key might have been deleted, just show key name
                                println!(
                                    "  {}: {} = {}",
                                    (i + 1).to_string().bright_black(),
                                    key.bright_white(),
                                    "(error reading value)".red()
                                );
                            }
                        }
                    }
                    
                    if total_count > 50 {
                        println!(
                            "{}",
                            format!(
                                "... and {} more keys (showing first 50)",
                                total_count - 50
                            ).bright_yellow()
                        );
                    }
                }
            }
            Command::Get { key } => match viewer.get_key(key) {
                Ok(info) => {
                    print_key_info(&info);
                }
                Err(e) => {
                    println!("{} {}", "Error:".bright_red().bold(), e.to_string().red());
                }
            },
            Command::Search { pattern, is_regex } => {
                let results = viewer.search_values(pattern, *is_regex)?;
                if results.is_empty() {
                    println!("{}", "No values found matching the pattern.".yellow());
                } else {
                    println!(
                        "{} {} {}",
                        "Found".bright_blue(),
                        results.len().to_string().bright_yellow().bold(),
                        "matches:".bright_blue()
                    );
                    for (i, pair) in results.iter().enumerate() {
                        println!(
                            "  {}: {} {} {}",
                            (i + 1).to_string().bright_black(),
                            pair.key.bright_cyan().bold(),
                            "=>".bright_black(),
                            truncate_value(&pair.value, 100).bright_white()
                        );
                    }
                }
            }
            Command::Help => {
                print_help();
            }
            Command::Exit => {
                println!("{}", "Goodbye!".bright_green());
            }
        }
        Ok(())
    }
}

fn print_key_info(info: &KeyInfo) {
    println!();
    println!("{}", "═".repeat(50).bright_cyan());
    println!(
        "{} {}",
        "Key:".bright_blue().bold(),
        info.key.bright_cyan().bold()
    );
    println!(
        "{} {} bytes",
        "Size:".bright_blue().bold(),
        info.size.to_string().bright_yellow()
    );
    println!(
        "{} {}",
        "UTF-8:".bright_blue().bold(),
        if info.is_utf8 {
            "Yes".bright_green()
        } else {
            "No".bright_red()
        }
    );
    println!("{}", "Value:".bright_blue().bold());
    println!("{}", "─".repeat(50).bright_black());

    if info.value.len() > 1000 {
        println!("{}", format!("{}...", &info.value[..1000]).bright_white());
        println!(
            "{}",
            format!(
                "(truncated, showing first 1000 characters of {})",
                info.value.len()
            )
            .bright_black()
        );
    } else {
        println!("{}", info.value.bright_white());
    }

    println!("{}", "═".repeat(50).bright_cyan());
    println!();
}

fn truncate_value(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        value.to_string()
    } else {
        format!("{}...", &value[..max_len])
    }
}

fn print_help() {
    println!();
    println!("{}", "Available Commands:".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_cyan());

    println!(
        "{:<20} Show total number of records",
        "count".bright_green().bold()
    );
    println!(
        "{:<20} List keys matching pattern (default: *)",
        "list [pattern]".bright_green().bold()
    );
    println!(
        "{:<20} List keys matching regex pattern",
        "list regex <regex>".bright_green().bold()
    );
    println!(
        "{:<20} Get value and info for a specific key",
        "get <key>".bright_green().bold()
    );
    println!(
        "{:<20} Search values matching pattern",
        "search <pattern>".bright_green().bold()
    );
    println!(
        "{:<20} Search values matching regex pattern",
        "search regex <regex>".bright_green().bold()
    );
    println!(
        "{:<20} Show this help message",
        "help".bright_green().bold()
    );
    println!(
        "{:<20} Show key completions for a command",
        "complete <cmd>".bright_green().bold()
    );
    println!("{:<20} Exit the application", "exit".bright_green().bold());

    println!();
    println!("{}", "Examples:".bright_blue().bold());
    println!("  {} {}", ">".bright_black(), "list user_*".bright_white());
    println!(
        "  {} {}",
        ">".bright_black(),
        "list regex user_[0-9]+".bright_white()
    );
    println!("  {} {}", ">".bright_black(), "get user_123".bright_white());
    println!(
        "  {} {}",
        ">".bright_black(),
        "search *@example.com".bright_white()
    );
    println!(
        "  {} {}",
        ">".bright_black(),
        "search regex \\d{4}-\\d{2}-\\d{2}".bright_white()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse_count() {
        let cmd = Command::parse("count");
        assert!(matches!(cmd, Some(Command::Count)));
    }

    #[test]
    fn test_command_parse_list() {
        let cmd = Command::parse("list");
        assert!(
            matches!(cmd, Some(Command::List { pattern, is_regex }) if pattern == "*" && !is_regex)
        );

        let cmd = Command::parse("list test*");
        assert!(
            matches!(cmd, Some(Command::List { pattern, is_regex }) if pattern == "test*" && !is_regex)
        );

        let cmd = Command::parse("list regex test.*");
        assert!(
            matches!(cmd, Some(Command::List { pattern, is_regex }) if pattern == "test.*" && is_regex)
        );
    }

    #[test]
    fn test_command_parse_get() {
        let cmd = Command::parse("get test_key");
        assert!(matches!(cmd, Some(Command::Get { key }) if key == "test_key"));

        let cmd = Command::parse("get key with spaces");
        assert!(matches!(cmd, Some(Command::Get { key }) if key == "key with spaces"));
    }

    #[test]
    fn test_command_parse_search() {
        let cmd = Command::parse("search *test*");
        assert!(
            matches!(cmd, Some(Command::Search { pattern, is_regex }) if pattern == "*test*" && !is_regex)
        );

        let cmd = Command::parse("search regex .*test.*");
        assert!(
            matches!(cmd, Some(Command::Search { pattern, is_regex }) if pattern == ".*test.*" && is_regex)
        );
    }

    #[test]
    fn test_command_parse_help() {
        let cmd = Command::parse("help");
        assert!(matches!(cmd, Some(Command::Help)));

        let cmd = Command::parse("?");
        assert!(matches!(cmd, Some(Command::Help)));
    }

    #[test]
    fn test_command_parse_exit() {
        let cmd = Command::parse("exit");
        assert!(matches!(cmd, Some(Command::Exit)));

        let cmd = Command::parse("quit");
        assert!(matches!(cmd, Some(Command::Exit)));

        let cmd = Command::parse("q");
        assert!(matches!(cmd, Some(Command::Exit)));
    }

    #[test]
    fn test_command_parse_invalid() {
        let cmd = Command::parse("invalid_command");
        assert!(cmd.is_none());

        let cmd = Command::parse("");
        assert!(cmd.is_none());

        let cmd = Command::parse("get");
        assert!(cmd.is_none());

        let cmd = Command::parse("search");
        assert!(cmd.is_none());
    }
}
