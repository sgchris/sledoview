use crate::db::{KeyInfo, SledViewer};
use anyhow::Result;
use colored::*;

/// Parse quoted arguments from a command line, handling escaped quotes
fn parse_quoted_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if escape_next {
            current_arg.push(ch);
            escape_next = false;
        } else {
            match ch {
                '\\' => {
                    escape_next = true;
                }
                '"' => {
                    if in_quotes {
                        // End of quoted string - always push even if empty
                        in_quotes = false;
                        args.push(current_arg.clone());
                        current_arg.clear();
                    } else {
                        // Start of quoted string
                        in_quotes = true;
                        if !current_arg.is_empty() {
                            args.push(current_arg.clone());
                            current_arg.clear();
                        }
                    }
                }
                ' ' | '\t' => {
                    if in_quotes {
                        current_arg.push(ch);
                    } else if !current_arg.is_empty() {
                        args.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                _ => {
                    current_arg.push(ch);
                }
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

/// Validate that a key contains only allowed characters for SLED
fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }

    if key.len() > 512 {
        return Err("Key too long (max 512 characters)".to_string());
    }

    // Allow alphanumeric, underscore, hyphen, dot, colon, and forward slash
    // These are commonly safe characters for database keys
    for ch in key.chars() {
        if !ch.is_ascii_alphanumeric() && !matches!(ch, '_' | '-' | '.' | ':' | '/' | ' ') {
            return Err(format!(
                "Invalid character '{}' in key. Allowed: a-z, A-Z, 0-9, _, -, ., :, /, space",
                ch
            ));
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum Command {
    Count,
    List { pattern: String, is_regex: bool },
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Search { pattern: String, is_regex: bool },
    Trees { pattern: String, is_regex: bool },
    Select { tree: String },
    Unselect,
    Help,
    Exit,
}

impl Command {
    pub fn parse(input: &str) -> Option<Command> {
        let args = parse_quoted_args(input);

        if args.is_empty() {
            return None;
        }

        match args[0].to_lowercase().as_str() {
            "count" => Some(Command::Count),
            "list" => {
                if args.len() == 1 {
                    Some(Command::List {
                        pattern: "*".to_string(),
                        is_regex: false,
                    })
                } else if args.len() == 2 {
                    Some(Command::List {
                        pattern: args[1].clone(),
                        is_regex: false,
                    })
                } else if args.len() == 3 && args[1] == "regex" {
                    Some(Command::List {
                        pattern: args[2].clone(),
                        is_regex: true,
                    })
                } else {
                    None
                }
            }
            "get" => {
                if args.len() >= 2 {
                    Some(Command::Get {
                        key: args[1].clone(),
                    })
                } else {
                    None
                }
            }
            "set" => {
                if args.len() >= 3 {
                    Some(Command::Set {
                        key: args[1].clone(),
                        value: args[2].clone(),
                    })
                } else {
                    None
                }
            }
            "delete" | "del" => {
                if args.len() >= 2 {
                    Some(Command::Delete {
                        key: args[1].clone(),
                    })
                } else {
                    None
                }
            }
            "search" => {
                if args.len() == 1 {
                    None
                } else if args.len() == 2 {
                    Some(Command::Search {
                        pattern: args[1].clone(),
                        is_regex: false,
                    })
                } else if args.len() == 3 && args[1] == "regex" {
                    Some(Command::Search {
                        pattern: args[2].clone(),
                        is_regex: true,
                    })
                } else {
                    None
                }
            }
            "trees" => {
                if args.len() == 1 {
                    Some(Command::Trees {
                        pattern: "*".to_string(),
                        is_regex: false,
                    })
                } else if args.len() == 2 {
                    Some(Command::Trees {
                        pattern: args[1].clone(),
                        is_regex: false,
                    })
                } else if args.len() == 3 && args[1] == "regex" {
                    Some(Command::Trees {
                        pattern: args[2].clone(),
                        is_regex: true,
                    })
                } else {
                    None
                }
            }
            "select" => {
                if args.len() >= 2 {
                    Some(Command::Select {
                        tree: args[1].clone(),
                    })
                } else {
                    None
                }
            }
            "unselect" => Some(Command::Unselect),
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

    pub fn execute(&self, viewer: &mut SledViewer) -> Result<()> {
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
                            format!("... and {} more keys (showing first 50)", total_count - 50)
                                .bright_yellow()
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
            Command::Set { key, value } => {
                // Validate the key first
                if let Err(error_msg) = validate_key(key) {
                    println!("{} {}", "Error:".bright_red().bold(), error_msg.red());
                    return Ok(());
                }

                match viewer.set_key(key, value) {
                    Ok(_) => {
                        println!(
                            "{} {} {} {}",
                            "✓".bright_green().bold(),
                            "Successfully set key".bright_green(),
                            key.bright_cyan().bold(),
                            "with value".bright_green()
                        );
                        let truncated_value = if value.len() > 50 {
                            format!("{}...", &value[..50])
                        } else {
                            value.clone()
                        };
                        println!(
                            "  {} {}",
                            "Value:".bright_blue(),
                            truncated_value.bright_white()
                        );
                    }
                    Err(e) => {
                        println!(
                            "{} {} {} {}",
                            "✗".bright_red().bold(),
                            "Failed to set key".bright_red(),
                            key.bright_cyan().bold(),
                            e.to_string().red()
                        );
                    }
                }
            }
            Command::Delete { key } => match viewer.delete_key(key) {
                Ok(existed) => {
                    if existed {
                        println!(
                            "{} {} {}",
                            "✓".bright_green().bold(),
                            "Successfully deleted key".bright_green(),
                            key.bright_cyan().bold()
                        );
                    } else {
                        println!(
                            "{} {} {}",
                            "!".bright_yellow().bold(),
                            "Key not found:".bright_yellow(),
                            key.bright_cyan().bold()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{} {} {} {}",
                        "✗".bright_red().bold(),
                        "Failed to delete key".bright_red(),
                        key.bright_cyan().bold(),
                        e.to_string().red()
                    );
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
            Command::Trees { pattern, is_regex } => {
                let trees = viewer.list_trees(pattern, *is_regex)?;
                if trees.is_empty() {
                    println!("{}", "No trees found matching the pattern.".yellow());
                } else {
                    let total_count = trees.len();
                    let display_trees = if total_count > 50 {
                        &trees[0..50]
                    } else {
                        &trees
                    };

                    println!(
                        "{} {} {}",
                        "Found".bright_blue(),
                        total_count.to_string().bright_yellow().bold(),
                        "trees:".bright_blue()
                    );

                    for tree_name in display_trees {
                        println!("  {}", tree_name.bright_cyan());
                    }

                    if total_count > 50 {
                        println!(
                            "{}",
                            format!("... and {} more trees (showing first 50)", total_count - 50)
                                .bright_yellow()
                        );
                    }
                }
            }
            Command::Select { tree } => match viewer.select_tree(tree) {
                Ok(_) => {
                    println!(
                        "{} {} {}",
                        "✓".bright_green().bold(),
                        "Selected tree:".bright_green(),
                        tree.bright_cyan().bold()
                    );
                }
                Err(e) => {
                    println!(
                        "{} {} {} {}",
                        "✗".bright_red().bold(),
                        "Failed to select tree".bright_red(),
                        tree.bright_cyan().bold(),
                        e.to_string().red()
                    );
                }
            },
            Command::Unselect => match viewer.unselect_tree() {
                Ok(was_selected) => {
                    if was_selected {
                        println!(
                            "{} {}",
                            "✓".bright_green().bold(),
                            "Tree unselected. Now working with the default tree.".bright_green()
                        );
                    } else {
                        println!(
                            "{} {}",
                            "!".bright_yellow().bold(),
                            "No tree was previously selected.".bright_yellow()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "{} {} {}",
                        "✗".bright_red().bold(),
                        "Failed to unselect tree:".bright_red(),
                        e.to_string().red()
                    );
                }
            },
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
        "{:<25} Show total number of records",
        "count".bright_green().bold()
    );
    println!(
        "{:<25} List keys matching pattern (default: *)",
        "list [pattern]".bright_green().bold()
    );
    println!(
        "{:<25} List keys matching regex pattern",
        "list regex <regex>".bright_green().bold()
    );
    println!(
        "{:<25} Get value and info for a specific key",
        "get <key>".bright_green().bold()
    );
    println!(
        "{:<25} Set/update a key-value pair",
        "set <key> <value>".bright_green().bold()
    );
    println!("{:<25} Delete a key", "delete <key>".bright_green().bold());
    println!(
        "{:<25} Search values matching pattern",
        "search <pattern>".bright_green().bold()
    );
    println!(
        "{:<25} Search values matching regex pattern",
        "search regex <regex>".bright_green().bold()
    );
    println!(
        "{:<25} Show this help message",
        "help".bright_green().bold()
    );
    println!();
    println!("{}", "Tree Management:".bright_blue().bold());
    println!(
        "{:<25} List all trees in the database",
        "trees".bright_green().bold()
    );
    println!(
        "{:<25} List trees matching glob pattern",
        "trees <pattern>".bright_green().bold()
    );
    println!(
        "{:<25} List trees matching regex pattern",
        "trees regex <pattern>".bright_green().bold()
    );
    println!(
        "{:<25} Select a tree to work with",
        "select <tree>".bright_green().bold()
    );
    println!(
        "{:<25} Unselect current tree (return to default)",
        "unselect".bright_green().bold()
    );

    println!();
    println!("{}", "Tree Usage:".bright_blue().bold());
    println!(
        "  {} Trees provide data isolation - keys in different trees are separate",
        "•".bright_blue()
    );
    println!(
        "  {} When a tree is selected, the prompt shows: {}",
        "•".bright_blue(),
        "[tree_name]>".bright_magenta()
    );
    println!(
        "  {} All CRUD operations work on the selected tree",
        "•".bright_blue()
    );
    println!(
        "  {} Use 'unselect' to return to the default tree",
        "•".bright_blue()
    );

    println!();
    println!("{}", "Advanced Usage:".bright_blue().bold());
    println!(
        "{:<25} Show key completions for a command",
        "complete <cmd>".bright_green().bold()
    );
    println!("{:<25} Exit the application", "exit".bright_green().bold());

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
        "set user_123 \"John Doe\"".bright_white()
    );
    println!(
        "  {} {}",
        ">".bright_black(),
        "set \"user name\" value".bright_white()
    );
    println!(
        "  {} {}",
        ">".bright_black(),
        "delete user_123".bright_white()
    );
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
    println!("  {} {}", ">".bright_black(), "trees".bright_white());
    println!("  {} {}", ">".bright_black(), "trees *_data".bright_white());
    println!(
        "  {} {}",
        ">".bright_black(),
        "select settings".bright_white()
    );
    println!(
        "  {} {}",
        "[settings]>".bright_magenta(),
        "list".bright_white()
    );
    println!(
        "  {} {}",
        "[settings]>".bright_magenta(),
        "unselect".bright_white()
    );

    println!();
    println!("{}", "Note:".bright_blue().bold());
    println!(
        "  {} Use quotes for keys/values with spaces: {} or {}",
        "•".bright_blue(),
        "\"key name\"".bright_yellow(),
        "\"value with spaces\"".bright_yellow()
    );
    println!(
        "  {} Escape quotes in values: {} → {}",
        "•".bright_blue(),
        "\"He said \\\"hello\\\"\"".bright_yellow(),
        "He said \"hello\"".bright_white()
    );
    println!("  {} Keys are auto-completed with TAB", "•".bright_blue());
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

        let cmd = Command::parse("get \"key with spaces\"");
        assert!(matches!(cmd, Some(Command::Get { key }) if key == "key with spaces"));
    }

    #[test]
    fn test_command_parse_set() {
        let cmd = Command::parse("set key value");
        assert!(
            matches!(cmd, Some(Command::Set { key, value }) if key == "key" && value == "value")
        );

        let cmd = Command::parse("set \"key with spaces\" \"value with spaces\"");
        assert!(
            matches!(cmd, Some(Command::Set { key, value }) if key == "key with spaces" && value == "value with spaces")
        );

        let cmd = Command::parse("set key \"value with \\\"quotes\\\"\"");
        assert!(
            matches!(cmd, Some(Command::Set { key, value }) if key == "key" && value == "value with \"quotes\"")
        );

        // Test incomplete set command
        let cmd = Command::parse("set key");
        assert!(cmd.is_none());

        let cmd = Command::parse("set");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_command_parse_delete() {
        let cmd = Command::parse("delete test_key");
        assert!(matches!(cmd, Some(Command::Delete { key }) if key == "test_key"));

        let cmd = Command::parse("del test_key");
        assert!(matches!(cmd, Some(Command::Delete { key }) if key == "test_key"));

        let cmd = Command::parse("delete \"key with spaces\"");
        assert!(matches!(cmd, Some(Command::Delete { key }) if key == "key with spaces"));

        // Test incomplete delete command
        let cmd = Command::parse("delete");
        assert!(cmd.is_none());
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
    fn test_parse_quoted_args() {
        // Simple unquoted arguments
        let args = parse_quoted_args("set key value");
        assert_eq!(args, vec!["set", "key", "value"]);

        // Quoted arguments with spaces
        let args = parse_quoted_args("set \"key with spaces\" \"value with spaces\"");
        assert_eq!(args, vec!["set", "key with spaces", "value with spaces"]);

        // Escaped quotes
        let args = parse_quoted_args("set key \"value with \\\"quotes\\\"\"");
        assert_eq!(args, vec!["set", "key", "value with \"quotes\""]);

        // Mixed quoted and unquoted
        let args = parse_quoted_args("set \"key name\" simple_value");
        assert_eq!(args, vec!["set", "key name", "simple_value"]);

        // Empty quotes
        let args = parse_quoted_args("set key \"\"");
        assert_eq!(args, vec!["set", "key", ""]);

        // Single word in quotes
        let args = parse_quoted_args("get \"key\"");
        assert_eq!(args, vec!["get", "key"]);
    }

    #[test]
    fn test_validate_key() {
        // Valid keys
        assert!(validate_key("user_123").is_ok());
        assert!(validate_key("config.database").is_ok());
        assert!(validate_key("app:settings").is_ok());
        assert!(validate_key("path/to/key").is_ok());
        assert!(validate_key("key with spaces").is_ok());

        // Invalid keys
        assert!(validate_key("").is_err());
        assert!(validate_key("key@invalid").is_err());
        assert!(validate_key("key#invalid").is_err());
        assert!(validate_key("key$invalid").is_err());

        // Too long key
        let long_key = "a".repeat(600);
        assert!(validate_key(&long_key).is_err());
    }

    #[test]
    fn test_command_parse_invalid() {
        let cmd = Command::parse("invalid_command");
        assert!(cmd.is_none());

        let cmd = Command::parse("list too many args here");
        assert!(cmd.is_none());

        let cmd = Command::parse("get");
        assert!(cmd.is_none());

        let cmd = Command::parse("set key");
        assert!(cmd.is_none());

        let cmd = Command::parse("delete");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_command_parse_trees() {
        let cmd = Command::parse("trees");
        assert!(
            matches!(cmd, Some(Command::Trees { pattern, is_regex }) if pattern == "*" && !is_regex)
        );

        let cmd = Command::parse("trees my_tree_*");
        assert!(
            matches!(cmd, Some(Command::Trees { pattern, is_regex }) if pattern == "my_tree_*" && !is_regex)
        );

        let cmd = Command::parse("trees regex my_tree_\\d+");
        assert!(
            matches!(cmd, Some(Command::Trees { pattern, is_regex }) if pattern == "my_tree_d+" && is_regex)
        );

        // Test trees command with "regex" as pattern (valid)
        let cmd = Command::parse("trees regex");
        assert!(
            matches!(cmd, Some(Command::Trees { pattern, is_regex }) if pattern == "regex" && !is_regex)
        );

        // Test trees command with too many args
        let cmd = Command::parse("trees regex pattern extra");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_command_parse_select() {
        let cmd = Command::parse("select my_tree");
        assert!(matches!(cmd, Some(Command::Select { tree }) if tree == "my_tree"));

        let cmd = Command::parse("select tree_with_underscore");
        assert!(matches!(cmd, Some(Command::Select { tree }) if tree == "tree_with_underscore"));

        // Test incomplete select command
        let cmd = Command::parse("select");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_command_parse_unselect() {
        let cmd = Command::parse("unselect");
        assert!(matches!(cmd, Some(Command::Unselect)));

        // Unselect doesn't take arguments
        let cmd = Command::parse("unselect extra_arg");
        assert!(matches!(cmd, Some(Command::Unselect)));
    }
}
