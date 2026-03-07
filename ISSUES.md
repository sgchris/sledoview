# Issues

Top 9 issues identified in the `src` code, ordered from highest to lowest priority.

## [x] 1. Startup writability check mutates the database

The startup path calls `viewer.is_writable()`, and that method performs a real `insert`, `remove`, and `flush` against the database. A startup health check should not modify user data. This creates unnecessary write amplification and can leave behind a sentinel key if the process terminates at the wrong time.

Recommended change: remove the mutating probe and report write failures only when the user runs a mutating command.

## [x] 2. Tree selection silently creates missing trees

`select_tree()` validates a tree by calling `open_tree()`, but sled creates the tree if it does not already exist. That means `select some_missing_tree` changes the database instead of just selecting an existing tree.

Recommended change: validate tree selection against the existing tree list and only open a tree after confirming it already exists.

## [x] 3. Unicode-safe truncation is missing in display code

Several display paths truncate strings using byte indexing after checking `len()`. This can panic on valid UTF-8 data when values contain multi-byte characters.

Recommended change: replace direct slicing with character-boundary-safe truncation helpers.

## [ ] 4. REPL completion eagerly scans the full database too often

The REPL loads all keys and trees at startup and reloads them after many commands. For large databases this becomes an avoidable full scan and memory cost on the hot path.

Recommended change: make completion lazy, prefix-driven, or capped to a bounded number of suggestions.

## [ ] 5. `list` uses an N+1 read pattern

The `list` command first loads matching keys and then re-reads each key individually to build the value preview. This doubles the database work and degrades performance on larger datasets.

Recommended change: return key and preview data in a single database pass.

## [ ] 6. Validation and open flow are duplicated

The application opens the database during validation to test lock state, then opens it again for normal use. Lock detection logic is also duplicated in more than one module. This adds extra I/O, creates race windows, and complicates maintenance.

Recommended change: consolidate validation and opening into one typed constructor or initialization flow.

## [ ] 7. Error handling mixes typed domain errors with `anyhow`

The code defines a dedicated `SledoViewError`, but many core APIs still return `anyhow::Result`, and startup code has to downcast errors to recover structured meaning. This weakens the API design and makes error handling less idiomatic.

Recommended change: use `SledoViewError` throughout the library layers and convert at the binary boundary only if needed.

## [ ] 8. Key validation does not match sled's data model

The write path restricts keys to a narrow ASCII subset, while the rest of the code explicitly supports binary keys and non-UTF-8 display paths. That inconsistency makes the API harder to reason about and limits legitimate sled use cases.

Recommended change: either support binary/non-ASCII keys consistently or clearly separate textual CLI constraints from the underlying database model.

## [ ] 9. Command parsing is too permissive and partly ad hoc

The command parser is handwritten and some commands accept extra trailing arguments but silently ignore them. That makes mistakes harder to detect and leads to brittle CLI behavior.

Recommended change: tighten parsing so malformed commands fail explicitly, and consider a more structured parser for REPL input.