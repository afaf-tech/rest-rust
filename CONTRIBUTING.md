# Contributing to Afaf Rest Rust

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Issue Labels](#issue-labels)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior via GitHub Security Advisories.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.75 or higher recommended)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup default stable
  ```

- **PostgreSQL** (version 12 or higher)
  - macOS: `brew install postgresql && brew services start postgresql`
  - Ubuntu/Debian: `sudo apt install postgresql postgresql-contrib`
  - Create a database: `createdb rest_rust_db`

- **SQLx CLI**
  ```bash
  cargo install sqlx-cli --no-default-features --features rustls,postgres
  ```

- **Optional: Docker & Docker Compose** (for containerized development)
  - [Install Docker](https://docs.docker.com/get-docker/)

## Development Setup

### Option 1: Native Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/afaf-tech/rest-rust.git
   cd rest-rust
   ```

2. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env with your PostgreSQL credentials
   ```

3. **Run database migrations**
   ```bash
   sqlx migrate run
   ```

4. **Start the development server**
   ```bash
   cargo run server
   ```

5. **Verify it's working**
   - API: http://localhost:9000
   - Swagger UI: http://localhost:9000/swagger-ui/

### Option 2: Docker Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/afaf-tech/rest-rust.git
   cd rest-rust
   ```

2. **Start the Docker environment**
   ```bash
   make docker-up
   # Or: docker-compose up -d
   ```

3. **Run migrations**
   ```bash
   make migrate
   ```

The application will be available at http://localhost:9000 with hot reload enabled.

## Code Style Guidelines

We use standard Rust tooling to maintain consistent code style:

### Formatting with rustfmt

All code must be formatted with `rustfmt`:

```bash
# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt --check
```

### Linting with Clippy

Code must pass Clippy lints without warnings:

```bash
# Run clippy
cargo clippy -- -D warnings

# Run clippy on all targets including tests
cargo clippy --all-targets -- -D warnings
```

### General Guidelines

- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Write documentation comments (`///`) for public APIs
- Keep functions focused and small
- Prefer returning `Result` over panicking
- Use meaningful variable and function names

## Testing Requirements

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for a specific module
cargo test users
```

### Test Requirements for PRs

Before submitting a PR, ensure:

1. **All existing tests pass**: `cargo test`
2. **New features have tests**: Add unit and/or integration tests
3. **Test coverage**: Aim for meaningful test coverage, not just line coverage
4. **Integration tests**: For repository/handler changes, add integration tests in `tests/`

### Test Patterns

- Use unique identifiers (UUIDs) in test data to avoid conflicts
- Clean up test data or use transactions
- Test both success and error cases
- See existing tests in `tests/` for examples

## Pull Request Process

### Before Submitting

1. **Create an issue first** for significant changes to discuss the approach
2. **Fork the repository** and create a feature branch
3. **Follow the branch naming convention**:
   - `feature/description` for new features
   - `fix/description` for bug fixes
   - `docs/description` for documentation
   - `refactor/description` for refactoring

### Submission Checklist

- [ ] Code compiles without errors (`cargo check`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated if needed
- [ ] CHANGELOG.md is updated for user-facing changes
- [ ] Commit messages are clear and descriptive

### PR Review Process

1. Submit the PR against the `master` branch
2. Fill out the PR template completely
3. Wait for CI checks to pass
4. Address review feedback
5. Once approved, a maintainer will merge the PR

### Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Keep the first line under 72 characters
- Reference issues when applicable ("Fix #123")

## Issue Labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `enhancement` | New feature or request |
| `documentation` | Documentation improvements |
| `good first issue` | Good for newcomers |
| `help wanted` | Extra attention is needed |
| `question` | Further information is requested |
| `wontfix` | This will not be worked on |
| `duplicate` | This issue or PR already exists |

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/afaf-tech/rest-rust/discussions)
- **Bug Reports**: Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml)
- **Feature Requests**: Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml)

Thank you for contributing! 🎉
