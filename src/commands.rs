use crate::command_input::parse_line;
use crate::db::{format_key_bytes, KeyInfo, SledViewer};
use crate::error::Result;
use colored::*;

/// Validate that a CLI key is non-empty, reasonably sized, and terminal-safe.
fn validate_key(key: &str) -> std::result::Result<(), String> {
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }

    if key.chars().count() > 512 {
        return Err("Key too long (max 512 characters)".to_string());
    }

    if let Some(ch) = key.chars().find(|ch| ch.is_control()) {
        return Err(format!(
            "Invalid control character U+{:04X} in key. Use printable UTF-8 text only.",
            u32::from(ch)
        ));
    }

    Ok(())
}

fn usage_error(message: &str, usage: &str) -> Command {
    Command::UsageError {
        message: message.to_string(),
        usage: usage.to_string(),
    }
}

fn parse_fixed_arity_command<F>(
    args: &[String],
    expected: usize,
    message: &str,
    usage: &str,
    build: F,
) -> Command
where
    F: FnOnce(&[String]) -> Command,
{
    if args.len() == expected {
        build(args)
    } else {
        usage_error(message, usage)
    }
}

#[derive(Debug)]
pub enum Command {
    Count,
    List {
        pattern: String,
        is_regex: bool,
    },
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Delete {
        key: String,
    },
    Search {
        pattern: String,
        is_regex: bool,
    },
    Trees {
        pattern: String,
        is_regex: bool,
    },
    Select {
        tree: String,
    },
    Unselect,
    Help,
    Exit,
    /// A known command was typed but with wrong/missing arguments.
    UsageError {
        message: String,
        usage: String,
    },
    Clear,
}

impl Command {
    #[must_use]
    pub fn parse(input: &str) -> Option<Command> {
        let args = match parse_line(input) {
            Ok(parsed) => parsed.args,
            Err(error) => {
                return Some(usage_error(
                    &error.to_string(),
                    "See 'help' for command syntax.",
                ));
            }
        };

        if args.is_empty() {
            return None;
        }

        match args[0].to_lowercase().as_str() {
            "count" => Some(parse_fixed_arity_command(
                &args,
                1,
                "Invalid arguments for 'count'.",
                "count",
                |_| Command::Count,
            )),
            "list" | "ls" => {
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
                    Some(Command::UsageError {
                        message: "Invalid arguments for 'list'.".to_string(),
                        usage: "list [pattern]  |  list regex <pattern>".to_string(),
                    })
                }
            }
            "get" => Some(parse_fixed_arity_command(
                &args,
                2,
                "Invalid arguments for 'get'.",
                "get <key>",
                |args| Command::Get {
                    key: args[1].clone(),
                },
            )),
            "set" => Some(parse_fixed_arity_command(
                &args,
                3,
                "Invalid arguments for 'set'.",
                "set <key> <value>",
                |args| Command::Set {
                    key: args[1].clone(),
                    value: args[2].clone(),
                },
            )),
            "delete" | "del" => Some(parse_fixed_arity_command(
                &args,
                2,
                "Invalid arguments for 'delete'.",
                "delete <key>",
                |args| Command::Delete {
                    key: args[1].clone(),
                },
            )),
            "search" => {
                if args.len() == 1 {
                    Some(usage_error(
                        "'search' requires a pattern argument.",
                        "search <pattern>  |  search regex <pattern>",
                    ))
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
                    Some(usage_error(
                        "Invalid arguments for 'search'.",
                        "search <pattern>  |  search regex <pattern>",
                    ))
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
                    Some(Command::UsageError {
                        message: "Invalid arguments for 'trees'.".to_string(),
                        usage: "trees [pattern]  |  trees regex <pattern>".to_string(),
                    })
                }
            }
            "select" => Some(parse_fixed_arity_command(
                &args,
                2,
                "Invalid arguments for 'select'.",
                "select <tree>",
                |args| Command::Select {
                    tree: args[1].clone(),
                },
            )),
            "unselect" => Some(parse_fixed_arity_command(
                &args,
                1,
                "Invalid arguments for 'unselect'.",
                "unselect",
                |_| Command::Unselect,
            )),
            "help" | "?" => Some(parse_fixed_arity_command(
                &args,
                1,
                "Invalid arguments for 'help'.",
                "help | ?",
                |_| Command::Help,
            )),
            "exit" | "quit" | "q" => Some(parse_fixed_arity_command(
                &args,
                1,
                "Invalid arguments for 'exit'.",
                "exit | quit | q",
                |_| Command::Exit,
            )),
            "clear" => Some(parse_fixed_arity_command(
                &args,
                1,
                "Invalid arguments for 'clear'.",
                "clear",
                |_| Command::Clear,
            )),
            _ => None,
        }
    }

    fn format_value_preview(value: &str, is_utf8: bool) -> String {
        if !is_utf8 {
            return "(binary data)".red().to_string();
        }

        if value.is_empty() {
            return "(empty)".bright_black().to_string();
        }

        // For short values, show them fully
        if value.len() <= 50 {
            return value.bright_green().to_string();
        }

        // For longer values, show a preview with truncation
        let preview = value.chars().take(47).collect::<String>();
        format!("{preview}...").bright_green().to_string()
    }

    /// Returns `true` for `UsageError` variants, so callers can skip adding
    /// them to command history.
    #[must_use]
    pub fn is_usage_error(&self) -> bool {
        matches!(self, Command::UsageError { .. })
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
            Command::List { pattern, is_regex } => cmd_list(viewer, pattern, *is_regex)?,
            Command::Get { key } => cmd_get(viewer, key),
            Command::Set { key, value } => cmd_set(viewer, key, value),
            Command::Delete { key } => cmd_delete(viewer, key),
            Command::Search { pattern, is_regex } => cmd_search(viewer, pattern, *is_regex)?,
            Command::Trees { pattern, is_regex } => cmd_trees(viewer, pattern, *is_regex)?,
            Command::Select { tree } => cmd_select(viewer, tree),
            Command::Unselect => {
                cmd_unselect(viewer)?;
            }
            Command::Help => {
                print_help();
            }
            Command::Exit => {
                println!("{}", "Goodbye!".bright_green());
            }
            Command::UsageError { message, usage } => {
                println!("{} {}", "Error:".bright_red().bold(), message.red());
                println!("  {} {}", "Usage:".bright_blue(), usage.bright_white());
            }
            Command::Clear => {
                print!("\x1B[2J\x1B[3J\x1B[H");
                // Flush the buffer to ensure the command is immediately sent.
                let mut stdout = std::io::stdout();
                _ = std::io::Write::flush(&mut stdout);
            }
        }
        Ok(())
    }
}

fn cmd_list(viewer: &mut SledViewer, pattern: &str, is_regex: bool) -> Result<()> {
    let result = viewer.list_key_summaries(pattern, is_regex, 50)?;
    if result.total_count == 0 {
        println!("{}", "No keys found matching the pattern.".yellow());
    } else {
        let total_count = result.total_count;

        println!(
            "{} {} {}",
            "Found".bright_blue(),
            total_count.to_string().bright_yellow().bold(),
            "keys:".bright_blue()
        );

        for (i, item) in result.items.iter().enumerate() {
            let key_display = format_key_bytes(&item.key);
            let preview = Command::format_value_preview(&item.value, item.is_utf8);
            println!(
                "  {}: {} = {}",
                (i + 1).to_string().bright_black(),
                key_display.bright_white(),
                preview
            );
        }

        if total_count > 50 {
            println!(
                "{}",
                format!("... and {} more keys (showing first 50)", total_count - 50)
                    .bright_yellow()
            );
        }
    }
    Ok(())
}

fn cmd_get(viewer: &mut SledViewer, key: &str) {
    // Try exact string key first; if not found and the argument
    // looks like a hex string, fall back to matching binary keys
    // by their trailing hex digits (e.g. `get 61F8` finds
    // the key whose full hex ends with "61F8").
    let key_result = viewer.get_key(key).or_else(|original_err| {
        let looks_like_hex = !key.is_empty() && key.chars().all(|c| c.is_ascii_hexdigit());
        if looks_like_hex {
            viewer
                .find_key_by_hex_suffix(key)
                .and_then(|opt| opt.ok_or(original_err))
        } else {
            Err(original_err)
        }
    });
    match key_result {
        Ok(info) => print_key_info(&info),
        Err(e) => {
            println!("{} {}", "Error:".bright_red().bold(), e.to_string().red());
        }
    }
}

fn cmd_set(viewer: &mut SledViewer, key: &str, value: &str) {
    // Validate the key first
    if let Err(error_msg) = validate_key(key) {
        println!("{} {}", "Error:".bright_red().bold(), error_msg.red());
        return;
    }

    match viewer.set_key(key, value) {
        Ok(()) => {
            println!(
                "{} {} {} {}",
                "✓".bright_green().bold(),
                "Successfully set key".bright_green(),
                key.bright_cyan().bold(),
                "with value".bright_green()
            );
            let truncated_value = truncate_with_ellipsis(value, 50);
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

fn cmd_delete(viewer: &mut SledViewer, key: &str) {
    match viewer.delete_key(key) {
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
    }
}

fn cmd_search(viewer: &mut SledViewer, pattern: &str, is_regex: bool) -> Result<()> {
    let results = viewer.search_values(pattern, is_regex)?;
    if results.is_empty() {
        println!("{}", "No values found matching the pattern.".yellow());
    } else {
        let total_count = results.len();
        let display_results = if total_count > 50 {
            &results[0..50]
        } else {
            &results[..]
        };
        println!(
            "{} {} {}",
            "Found".bright_blue(),
            total_count.to_string().bright_yellow().bold(),
            "matches:".bright_blue()
        );
        for (i, pair) in display_results.iter().enumerate() {
            println!(
                "  {}: {} {} {}",
                (i + 1).to_string().bright_black(),
                pair.key.bright_cyan().bold(),
                "=>".bright_black(),
                truncate_value(&pair.value, 100).bright_white()
            );
        }
        if total_count > 50 {
            println!(
                "{}",
                format!(
                    "... and {} more matches (showing first 50)",
                    total_count - 50
                )
                .bright_yellow()
            );
        }
    }
    Ok(())
}

fn cmd_trees(viewer: &mut SledViewer, pattern: &str, is_regex: bool) -> Result<()> {
    let trees = viewer.list_trees(pattern, is_regex)?;
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
    Ok(())
}

fn cmd_unselect(viewer: &mut SledViewer) -> Result<()> {
    let was_selected = viewer.unselect_tree()?;
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
    Ok(())
}

fn cmd_select(viewer: &mut SledViewer, tree: &str) {
    match viewer.select_tree(tree) {
        Ok(()) => {
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

    if exceeds_char_limit(&info.value, 1000) {
        println!(
            "{}",
            truncate_with_ellipsis(&info.value, 1000).bright_white()
        );
        println!(
            "{}",
            format!(
                "(truncated, showing first 1000 characters of {})",
                info.value.chars().count()
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
    truncate_with_ellipsis(value, max_len)
}

fn truncate_chars(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

fn exceeds_char_limit(value: &str, max_chars: usize) -> bool {
    value.chars().nth(max_chars).is_some()
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if !exceeds_char_limit(value, max_chars) {
        return value.to_string();
    }

    format!("{}...", truncate_chars(value, max_chars))
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
    println!("{:<25} Clear the screen", "clear".bright_green().bold());
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
        "[tree_name]>".bright_cyan()
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
        "[settings]>".bright_white(),
        "list".bright_white()
    );
    println!(
        "  {} {}",
        "[settings]>".bright_white(),
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
    println!(
        "  {} Keys accept printable UTF-8 text; quote them if they contain spaces",
        "•".bright_blue()
    );
    println!(
        "  {} Binary keys remain readable via {} using their trailing hex digits",
        "•".bright_blue(),
        "get <hex-suffix>".bright_yellow()
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

        let cmd = Command::parse("count extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
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

        let cmd = Command::parse("get test_key extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
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
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("set");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("set key value extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
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
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("delete test_key extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
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

        let cmd = Command::parse("help extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_command_parse_exit() {
        let cmd = Command::parse("exit");
        assert!(matches!(cmd, Some(Command::Exit)));

        let cmd = Command::parse("quit");
        assert!(matches!(cmd, Some(Command::Exit)));

        let cmd = Command::parse("q");
        assert!(matches!(cmd, Some(Command::Exit)));

        let cmd = Command::parse("exit now");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_parse_quoted_args() {
        // Simple unquoted arguments
        let args = parse_line("set key value").unwrap().args;
        assert_eq!(args, vec!["set", "key", "value"]);

        // Quoted arguments with spaces
        let args = parse_line("set \"key with spaces\" \"value with spaces\"")
            .unwrap()
            .args;
        assert_eq!(args, vec!["set", "key with spaces", "value with spaces"]);

        // Escaped quotes
        let args = parse_line("set key \"value with \\\"quotes\\\"\"")
            .unwrap()
            .args;
        assert_eq!(args, vec!["set", "key", "value with \"quotes\""]);

        // Mixed quoted and unquoted
        let args = parse_line("set \"key name\" simple_value").unwrap().args;
        assert_eq!(args, vec!["set", "key name", "simple_value"]);

        // Empty quotes
        let args = parse_line("set key \"\"").unwrap().args;
        assert_eq!(args, vec!["set", "key", ""]);

        // Single word in quotes
        let args = parse_line("get \"key\"").unwrap().args;
        assert_eq!(args, vec!["get", "key"]);
    }

    #[test]
    fn test_command_parse_rejects_unterminated_quote() {
        let cmd = Command::parse("set \"unterminated");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_command_parse_rejects_trailing_escape() {
        let cmd = Command::parse("set key value\\");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_validate_key() {
        // Valid keys
        assert!(validate_key("user_123").is_ok());
        assert!(validate_key("config.database").is_ok());
        assert!(validate_key("app:settings").is_ok());
        assert!(validate_key("path/to/key").is_ok());
        assert!(validate_key("key with spaces").is_ok());
        assert!(validate_key("config_日本").is_ok());
        assert!(validate_key("café").is_ok());
        assert!(validate_key("key@still-valid").is_ok());

        // Invalid keys
        assert!(validate_key("").is_err());
        assert!(validate_key("line\nbreak").is_err());
        assert!(validate_key("tab\tkey").is_err());

        // Too long key
        let long_key = "a".repeat(600);
        assert!(validate_key(&long_key).is_err());
    }

    #[test]
    fn test_command_parse_invalid() {
        let cmd = Command::parse("invalid_command");
        assert!(cmd.is_none());

        let cmd = Command::parse("list too many args here");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("get");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("set key");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("delete");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
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
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_command_parse_select() {
        let cmd = Command::parse("select my_tree");
        assert!(matches!(cmd, Some(Command::Select { tree }) if tree == "my_tree"));

        let cmd = Command::parse("select tree_with_underscore");
        assert!(matches!(cmd, Some(Command::Select { tree }) if tree == "tree_with_underscore"));

        // Test incomplete select command
        let cmd = Command::parse("select");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));

        let cmd = Command::parse("select my_tree extra");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_command_parse_unselect() {
        let cmd = Command::parse("unselect");
        assert!(matches!(cmd, Some(Command::Unselect)));

        // Unselect doesn't take arguments
        let cmd = Command::parse("unselect extra_arg");
        assert!(matches!(cmd, Some(Command::UsageError { .. })));
    }

    #[test]
    fn test_truncate_value_is_unicode_safe() {
        let value = "hello 👋🌍";
        let truncated = truncate_value(value, 7);

        assert_eq!(truncated, "hello 👋...");
        assert!(truncated.is_char_boundary(truncated.len()));
    }

    #[test]
    fn test_truncate_value_handles_cjk_and_accented_text() {
        assert_eq!(truncate_value("café résumé", 6), "café ...");
        assert_eq!(truncate_value("日本語テキスト", 4), "日本語テ...");
        assert_eq!(truncate_value("short", 10), "short");
    }

    #[test]
    fn test_exceeds_char_limit_uses_character_boundaries() {
        assert!(exceeds_char_limit("áéíóú", 3));
        assert!(!exceeds_char_limit("áéí", 3));
        assert!(exceeds_char_limit("😀😀😀😀", 3));
    }
}
