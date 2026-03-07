---
description: 'Rust programming language coding conventions and best practices'
applyTo: '**/*.rs'
---

# Rust Coding Conventions and Best Practices

Follow idiomatic Rust practices and community standards when writing Rust code. 

These instructions are based on [The Rust Book](https://doc.rust-lang.org/book/), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), [RFC 430 naming conventions](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md), and the broader Rust community at [users.rust-lang.org](https://users.rust-lang.org).

## General Instructions

- Always prioritize readability, safety, and maintainability.
- Use strong typing and leverage Rust's ownership system for memory safety.
- Break down complex functions into smaller, more manageable functions.
- For algorithm-related code, include explanations of the approach used.
- Write code with good maintainability practices, including comments on why certain design decisions were made.
- Handle errors gracefully using `Result<T, E>` and provide meaningful error messages.
- For external dependencies, mention their usage and purpose in documentation.
- Use consistent naming conventions following [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md).
- Write idiomatic, safe, and efficient Rust code that follows the borrow checker's rules.
- Ensure code compiles without warnings.
- Keep read paths, validation paths, and startup checks side-effect free unless mutation is explicitly part of the command contract.
- Preserve user data integrity: never probe writability, existence, or accessibility by performing temporary writes to the real database.
- Favor predictable CLI behavior: commands that look read-only must not create, delete, or mutate database state.
- Do not leave temporary files, alternate build directories, scratch outputs, or other disposable artifacts behind after prompt execution unless the user explicitly asked to keep them.

## Patterns to Follow

- Use modules (`mod`) and public interfaces (`pub`) to encapsulate logic.
- Handle errors properly using `?`, `match`, or `if let`.
- Use `serde` for serialization and `thiserror` or `anyhow` for custom errors.
- Implement traits to abstract services or external dependencies.
- Structure async code using `async/await` and `tokio` or `async-std`.
- Prefer enums over flags and states for type safety.
- Use builders for complex object creation.
- Split binary and library code (`main.rs` vs `lib.rs`) for testability and reuse.
- Use `rayon` for data parallelism and CPU-bound tasks.
- Use iterators instead of index-based loops as they're often faster and safer.
- Use `&str` instead of `String` for function parameters when you don't need ownership.
- Prefer borrowing and zero-copy operations to avoid unnecessary allocations.
- Keep the binary thin: argument parsing, printing, and process exit belong in `main.rs`; validation and business logic belong in library modules.
- Use one typed initialization flow for opening external resources. Validate and open once when possible instead of re-opening the same resource in multiple layers.
- Design APIs so command semantics match storage semantics. If an operation is documented as read-only, enforce that in code.
- Prefer explicit helper functions for string truncation and formatting that are safe on UTF-8 boundaries.
- For interactive tools, make completion and discovery features incremental or bounded; avoid full scans on every command.

### Ownership, Borrowing, and Lifetimes

- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary.
- Use `&mut T` when you need to modify borrowed data.
- Explicitly annotate lifetimes when the compiler cannot infer them.
- Use `Rc<T>` for single-threaded reference counting and `Arc<T>` for thread-safe reference counting.
- Use `RefCell<T>` for interior mutability in single-threaded contexts and `Mutex<T>` or `RwLock<T>` for multi-threaded contexts.

## Patterns to Avoid

- Don't use `unwrap()` or `expect()` unless absolutely necessary—prefer proper error handling.
- Avoid panics in library code—return `Result` instead.
- Don't rely on global mutable state—use dependency injection or thread-safe containers.
- Avoid deeply nested logic—refactor with functions or combinators.
- Don't ignore warnings—treat them as errors during CI.
- Avoid `unsafe` unless required and fully documented.
- Don't overuse `clone()`, use borrowing instead of cloning unless ownership transfer is needed.
- Avoid premature `collect()`, keep iterators lazy until you actually need the collection.
- Avoid unnecessary allocations—prefer borrowing and zero-copy operations.
- Avoid APIs whose validation step mutates external state, such as opening resources in a mode that creates files, trees, or records.
- Avoid hidden side effects in status checks like `is_writable`, `exists`, `validate`, or `select`.
- Avoid byte-indexed slicing of `String` or `&str` for user-visible truncation or previews.
- Avoid N+1 database access patterns when listing data that can be gathered in a single iteration.
- Avoid eager full-database scans for REPL completion, prompts, or hints on large datasets.
- Avoid handwritten parsers that silently ignore trailing or malformed arguments when a command should fail fast.
- Avoid mixing typed domain errors with `anyhow` across the same library boundary unless there is a clear, documented reason.

## Code Style and Formatting

- Follow the Rust Style Guide and use `rustfmt` for automatic formatting.
- Keep lines under 100 characters when possible.
- Place function and struct documentation immediately before the item using `///`.
- Use `cargo clippy` to catch common mistakes and enforce best practices.

## Error Handling

- Use `Result<T, E>` for recoverable errors and `panic!` only for unrecoverable errors.
- Prefer `?` operator over `unwrap()` or `expect()` for error propagation.
- Create custom error types using `thiserror` or implement `std::error::Error`.
- Use `Option<T>` for values that may or may not exist.
- Provide meaningful error messages and context.
- Error types should be meaningful and well-behaved (implement standard traits).
- Validate function arguments and return appropriate errors for invalid input.
- Prefer typed domain errors in library code. If `anyhow` is used, confine it to application boundaries or top-level orchestration layers.
- Do not downcast from `anyhow` in normal control flow when a typed error enum would model the behavior directly.
- Centralize platform-specific error classification logic, such as lock detection, so behavior stays consistent across modules.

## API Design Guidelines

### Common Traits Implementation
Eagerly implement common traits where appropriate:
- `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`
- Use standard conversion traits: `From`, `AsRef`, `AsMut`
- Collections should implement `FromIterator` and `Extend`
- Note: `Send` and `Sync` are auto-implemented by the compiler when safe; avoid manual implementation unless using `unsafe` code

### Type Safety and Predictability
- Use newtypes to provide static distinctions
- Arguments should convey meaning through types; prefer specific types over generic `bool` parameters
- Use `Option<T>` appropriately for truly optional values
- Functions with a clear receiver should be methods
- Only smart pointers should implement `Deref` and `DerefMut`
- Model read-only versus mutating operations explicitly in API shape and naming.
- Avoid `bool` parameters like `is_regex` in public APIs when an enum such as `PatternKind` would make call sites clearer.
- Return borrowed types like `Option<&str>` instead of `Option<&String>` where ownership is not needed.

### Future Proofing
- Use sealed traits to protect against downstream implementations
- Structs should have private fields
- Functions should validate their arguments
- All public types must implement `Debug`

## Testing and Documentation

- Write comprehensive unit tests using `#[cfg(test)]` modules and `#[test]` annotations.
- Use test modules alongside the code they test (`mod tests { ... }`).
- Write integration tests in `tests/` directory with descriptive filenames.
- Write clear and concise comments for each function, struct, enum, and complex logic.
- Ensure functions have descriptive names and include comprehensive documentation.
- Document all public APIs with rustdoc (`///` comments) following the [API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `#[doc(hidden)]` to hide implementation details from public documentation.
- Document error conditions, panic scenarios, and safety considerations.
- Examples should use `?` operator, not `unwrap()` or deprecated `try!` macro.
- Add tests for side-effect safety: validation, selection, and read-only commands must not create or modify on-disk state.
- Add tests for Unicode-heavy values and keys to catch invalid UTF-8 assumptions and byte-slicing panics.
- Add tests for large datasets or bounded completion behavior so REPL performance regressions are caught early.
- Add tests for malformed commands to ensure the parser rejects extra or invalid arguments consistently.

## Project Organization

- Use semantic versioning in `Cargo.toml`.
- Include comprehensive metadata: `description`, `license`, `repository`, `keywords`, `categories`.
- Use feature flags for optional functionality.
- Organize code into modules using `mod.rs` or named files.
- Keep `main.rs` or `lib.rs` minimal - move logic to modules.

## Quality Checklist

Before publishing or reviewing Rust code, ensure:

### Core Requirements
- [ ] **Naming**: Follows RFC 430 naming conventions
- [ ] **Traits**: Implements `Debug`, `Clone`, `PartialEq` where appropriate
- [ ] **Error Handling**: Uses `Result<T, E>` and provides meaningful error types
- [ ] **Documentation**: All public items have rustdoc comments with examples
- [ ] **Testing**: Comprehensive test coverage including edge cases

### Safety and Quality
- [ ] **Safety**: No unnecessary `unsafe` code, proper error handling
- [ ] **Performance**: Efficient use of iterators, minimal allocations
- [ ] **API Design**: Functions are predictable, flexible, and type-safe
- [ ] **Future Proofing**: Private fields in structs, sealed traits where appropriate
- [ ] **Tooling**: Code passes `cargo fmt`, `cargo clippy`, and `cargo test`
- [ ] **No Hidden Mutation**: Validation and read-oriented commands do not modify persistent state
- [ ] **No Duplicate Open Flow**: Resource validation and opening are not repeated across layers without need
- [ ] **UTF-8 Safety**: All user-visible truncation and preview logic respects character boundaries
- [ ] **Scalability**: Interactive features avoid unbounded full scans in common command paths