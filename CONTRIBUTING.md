# Contributing to SledoView

We love your input! We want to make contributing to SledoView as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

## Pull Requests

Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. **Fork the repo** and create your branch from `main`.
2. **Add tests** if you've added code that should be tested.
3. **Ensure the test suite passes** by running `cargo test`.
4. **Ensure your code is formatted** by running `cargo fmt`.
5. **Ensure your code passes linting** by running `cargo clippy`.
6. **Update documentation** if you've changed APIs.
7. **Issue that pull request!**

## Development Setup

### Prerequisites

- Rust 1.70+ (2021 edition)
- Git

### Setting Up the Development Environment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/sledoview.git
   cd sledoview
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Create a test database** (optional):
   ```bash
   cargo run --example create_test_db
   ```

5. **Run the application**:
   ```bash
   cargo run -- example_db
   ```

### Code Style

We follow standard Rust conventions:

- **Formatting**: Use `cargo fmt` to format your code
- **Linting**: Ensure `cargo clippy` passes without warnings
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- **Documentation**: Add doc comments for public APIs
- **Error Handling**: Use `Result` types and proper error handling

### Testing Guidelines

- **Write tests** for all new functionality
- **Integration tests** go in the `tests/` directory
- **Unit tests** go in the same file as the code they test, in a `#[cfg(test)]` mod tests` block
- **Test coverage**: Aim for high test coverage of new code
- **Test data**: Use the helper functions in `tests/common/` for creating test databases

#### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Documentation

- **API documentation**: Use doc comments (`///`) for public APIs
- **README**: Update if you change functionality
- **CHANGELOG**: Add entries for notable changes
- **Examples**: Add examples for new features

### Commit Messages

We prefer clear, descriptive commit messages:

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Examples:
```
Add regex support for value searching

- Implement search regex command
- Add comprehensive tests for regex patterns
- Update documentation with regex examples

Fixes #123
```

## Bug Reports

We use GitHub issues to track bugs. Report a bug by [opening a new issue](https://github.com/yourusername/sledoview/issues).

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

### Bug Report Template

```markdown
**Summary**: Brief description of the bug

**Environment**:
- OS: [e.g., Windows 10, macOS 12, Ubuntu 20.04]
- Rust version: [e.g., 1.70.0]
- SledoView version: [e.g., 0.1.0]

**Steps to Reproduce**:
1. Run command: `sledoview /path/to/db`
2. Type command: `list *`
3. See error

**Expected Behavior**: 
What you expected to happen

**Actual Behavior**: 
What actually happened

**Additional Context**:
Add any other context about the problem here
```

## Feature Requests

We welcome feature requests! Please provide:

- **Use case**: Why would this feature be useful?
- **Description**: What should the feature do?
- **Examples**: How would you use it?
- **Alternatives**: Are there workarounds?

## Security Issues

If you discover a security vulnerability, please send an email to [security@example.com] instead of using the public issue tracker.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code:

- **Be respectful** and inclusive
- **Be patient** with newcomers
- **Be constructive** in feedback
- **Focus on the code**, not the person

## Development Guidelines

### Performance

- **Benchmark critical paths** when making performance changes
- **Profile before optimizing** - don't guess where bottlenecks are
- **Consider memory usage** especially when dealing with large databases
- **Test with realistic data sizes**

### Error Handling

- **Use appropriate error types** from the `error` module
- **Provide helpful error messages** that guide users toward solutions
- **Handle all error cases** - avoid unwrap() in library code
- **Test error conditions** - ensure error paths are covered

### Database Safety

- **Read-only operations only** - this tool should never modify databases
- **Proper database locking** - ensure we don't interfere with other processes
- **Validate input** - check database format before accessing
- **Handle corruption gracefully** - provide helpful error messages

### Platform Compatibility

- **Test on multiple platforms** when possible
- **Use cross-platform dependencies** 
- **Avoid platform-specific code** unless necessary
- **Document platform-specific behavior**

## Getting Help

- **Documentation**: Check the README and inline documentation
- **Issues**: Search existing issues before creating new ones
- **Discussions**: Use GitHub Discussions for questions and ideas

## Recognition

Contributors will be:
- Listed in the project's contributors
- Mentioned in release notes for significant contributions
- Thanked in the project README

Thank you for contributing to SledoView! 🎉
