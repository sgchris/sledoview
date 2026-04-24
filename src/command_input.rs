#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLine {
    pub args: Vec<String>,
    pub ends_with_whitespace: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Command,
    Key,
    Tree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRequest {
    pub kind: CompletionKind,
    pub prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseInputError {
    UnterminatedQuote,
    TrailingEscape,
}

impl std::fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedQuote => write!(f, "Unterminated quoted string."),
            Self::TrailingEscape => write!(f, "Trailing escape character."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandKind {
    Count,
    List,
    Get,
    Set,
    Delete,
    Search,
    Trees,
    Select,
    Unselect,
    Help,
    Exit,
}

#[must_use]
pub fn command_names() -> &'static [&'static str] {
    &[
        "count", "list", "ls", "get", "set", "delete", "del", "search", "trees", "select",
        "unselect", "help", "exit", "quit", "q", "?",
    ]
}

pub fn parse_line(input: &str) -> std::result::Result<ParsedLine, ParseInputError> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            current_arg.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => {
                escape_next = true;
            }
            '"' => {
                if in_quotes {
                    in_quotes = false;
                    args.push(current_arg.clone());
                    current_arg.clear();
                } else {
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
            _ => current_arg.push(ch),
        }
    }

    if escape_next {
        return Err(ParseInputError::TrailingEscape);
    }

    if in_quotes {
        return Err(ParseInputError::UnterminatedQuote);
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    Ok(ParsedLine {
        args,
        ends_with_whitespace: input.ends_with([' ', '\t']),
    })
}

#[must_use]
pub fn completion_request(input: &str) -> Option<CompletionRequest> {
    let parsed = parse_line(input).ok()?;

    if parsed.args.is_empty() {
        return Some(CompletionRequest {
            kind: CompletionKind::Command,
            prefix: String::new(),
        });
    }

    if parsed.args.len() == 1 && !parsed.ends_with_whitespace {
        return Some(CompletionRequest {
            kind: CompletionKind::Command,
            prefix: parsed.args[0].clone(),
        });
    }

    let command = classify_command(&parsed.args[0])?;
    completion_target(command, &parsed.args, parsed.ends_with_whitespace)
}

fn classify_command(name: &str) -> Option<CommandKind> {
    match name.to_ascii_lowercase().as_str() {
        "count" => Some(CommandKind::Count),
        "list" | "ls" => Some(CommandKind::List),
        "get" => Some(CommandKind::Get),
        "set" => Some(CommandKind::Set),
        "delete" | "del" => Some(CommandKind::Delete),
        "search" => Some(CommandKind::Search),
        "trees" => Some(CommandKind::Trees),
        "select" => Some(CommandKind::Select),
        "unselect" => Some(CommandKind::Unselect),
        "help" | "?" => Some(CommandKind::Help),
        "exit" | "quit" | "q" => Some(CommandKind::Exit),
        _ => None,
    }
}

fn completion_target(
    command: CommandKind,
    args: &[String],
    ends_with_whitespace: bool,
) -> Option<CompletionRequest> {
    match command {
        CommandKind::Get | CommandKind::Delete | CommandKind::Set => {
            positional_completion(args, ends_with_whitespace, CompletionKind::Key)
        }
        CommandKind::Select => {
            positional_completion(args, ends_with_whitespace, CompletionKind::Tree)
        }
        CommandKind::List | CommandKind::Search => {
            pattern_completion(args, ends_with_whitespace, CompletionKind::Key)
        }
        CommandKind::Trees => pattern_completion(args, ends_with_whitespace, CompletionKind::Tree),
        CommandKind::Count | CommandKind::Unselect | CommandKind::Help | CommandKind::Exit => None,
    }
}

fn positional_completion(
    args: &[String],
    ends_with_whitespace: bool,
    kind: CompletionKind,
) -> Option<CompletionRequest> {
    if args.len() == 1 && ends_with_whitespace {
        Some(CompletionRequest {
            kind,
            prefix: String::new(),
        })
    } else if args.len() == 2 && !ends_with_whitespace {
        Some(CompletionRequest {
            kind,
            prefix: args[1].clone(),
        })
    } else {
        None
    }
}

fn pattern_completion(
    args: &[String],
    ends_with_whitespace: bool,
    kind: CompletionKind,
) -> Option<CompletionRequest> {
    if args.get(1).is_some_and(|arg| arg == "regex") {
        return None;
    }
    positional_completion(args, ends_with_whitespace, kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_rejects_unterminated_quote() {
        assert_eq!(
            parse_line("set \"unterminated").unwrap_err(),
            ParseInputError::UnterminatedQuote
        );
    }

    #[test]
    fn test_parse_line_rejects_trailing_escape() {
        assert_eq!(
            parse_line("set key value\\").unwrap_err(),
            ParseInputError::TrailingEscape
        );
    }

    #[test]
    fn test_parse_line_preserves_empty_quoted_argument() {
        let parsed = parse_line("set key \"\"").unwrap();
        assert_eq!(parsed.args, vec!["set", "key", ""]);
    }

    #[test]
    fn test_completion_request_for_partial_command() {
        assert_eq!(
            completion_request("se"),
            Some(CompletionRequest {
                kind: CompletionKind::Command,
                prefix: "se".to_string(),
            })
        );
    }

    #[test]
    fn test_completion_request_for_key_argument() {
        assert_eq!(
            completion_request("get user"),
            Some(CompletionRequest {
                kind: CompletionKind::Key,
                prefix: "user".to_string(),
            })
        );
    }

    #[test]
    fn test_completion_request_for_tree_argument() {
        assert_eq!(
            completion_request("select config"),
            Some(CompletionRequest {
                kind: CompletionKind::Tree,
                prefix: "config".to_string(),
            })
        );
    }

    #[test]
    fn test_completion_request_skips_regex_pattern_position() {
        assert_eq!(completion_request("list regex value"), None);
        assert_eq!(completion_request("trees regex value"), None);
    }
}
