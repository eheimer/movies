# Design Document

## Overview

This design outlines the refactoring of inline unit tests from source files to separate test files in the `tests/` directory. The refactoring will maintain all existing test coverage while improving code organization and readability. The project currently has inline tests in seven source modules: `config.rs`, `menu.rs`, `logger.rs`, `path_resolver.rs`, `episode_field.rs`, `display.rs`, and `theme.rs`.

## Architecture

### Current State

The codebase uses Rust's standard inline testing pattern with `#[cfg(test)]` modules at the end of source files:

```rust
// src/module.rs
pub fn some_function() { ... }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_some_function() { ... }
}
```

### Target State

Tests will be moved to separate files in the `tests/` directory following the naming pattern `{module_name}_tests.rs`:

```rust
// tests/module_tests.rs
use movies::{module_function, ModuleType};

#[test]
fn test_some_function() { ... }
```

The `tests/` directory will contain:
- `integration_tests.rs` (existing, unchanged)
- `config_tests.rs` (new)
- `menu_tests.rs` (new)
- `logger_tests.rs` (new)
- `path_resolver_tests.rs` (new)
- `episode_field_tests.rs` (new)
- `display_tests.rs` (new)
- `theme_tests.rs` (new)

## Components and Interfaces

### Library Crate Exposure

The `src/lib.rs` file must expose all modules and types that tests need to access. Currently, the library crate exposes:

```rust
pub mod config;
pub mod theme;
// ... other modules
```

Tests will import from the library crate:

```rust
use movies::config::Config;
use movies::theme::Theme;
```

### Test File Structure

Each test file will follow this structure:

1. **Imports**: Import necessary items from the `movies` crate and standard library
2. **Helper Functions**: Any test-specific helper functions (if needed)
3. **Test Functions**: Individual `#[test]` annotated functions

### Module-Specific Considerations

#### path_resolver_tests.rs
- Requires `tempfile` crate for temporary directory creation
- Tests file system operations and path validation
- Uses `PathResolver` struct and associated error types

#### episode_field_tests.rs
- Tests duration formatting logic
- Uses `EpisodeDetail` DTO and `EpisodeField` enum
- Includes test case documentation comments

#### config_tests.rs, theme_tests.rs, logger_tests.rs, menu_tests.rs, display_tests.rs
- Each requires specific imports from their respective modules
- May need access to structs, enums, and functions
- Some may require `tempfile` for file-based testing

## Data Models

No changes to data models are required. All existing structs, enums, and types remain unchanged. The refactoring only affects test organization.

## Test Cases

### Test Case 1: All inline tests moved

When examining source files in `src/`, they should contain no `#[cfg(test)]` modules.
**Validates: Requirements 1.1**

### Test Case 2: Test logic preservation

When running the test suite after refactoring, all tests should pass with identical behavior to before the refactoring.
**Validates: Requirements 1.2, 1.3**

### Test Case 3: Test file naming convention

When listing files in the `tests/` directory, new test files should follow the `{module_name}_tests.rs` naming pattern.
**Validates: Requirements 2.2**

### Test Case 4: Module separation

When examining test files, each source module should have its own corresponding test file rather than combining tests from multiple modules.
**Validates: Requirements 2.4**

### Test Case 5: Proper imports

When examining test files, they should import necessary items from the `movies` library crate using `use movies::module::Type` syntax.
**Validates: Requirements 2.3**

### Test Case 6: Steering documentation updated

When reading steering documentation, it should prohibit inline `#[cfg(test)]` modules and specify the correct test organization pattern.
**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

## Error Handling

### Compilation Errors

If tests fail to compile after moving:
- Verify all necessary items are public in `src/lib.rs`
- Check that imports in test files match the module structure
- Ensure helper types and functions are accessible

### Test Failures

If tests fail after moving:
- Verify test logic was copied exactly without modification
- Check that all dependencies (like `tempfile`) are available
- Ensure test data and fixtures are accessible from the new location

### Missing Dependencies

Some tests may require:
- `tempfile` crate (already in dev-dependencies)
- Access to private module internals (may require making some items `pub` or `pub(crate)`)

## Testing Strategy

### Verification Approach

1. **Pre-refactoring baseline**: Run `cargo test` to establish baseline (all tests should pass)
2. **Incremental migration**: Move tests one module at a time
3. **Per-module verification**: After each module's tests are moved, run `cargo test` to verify
4. **Post-refactoring verification**: Run full test suite to confirm all tests pass
5. **Code review**: Verify no `#[cfg(test)]` modules remain in source files

### Test Organization

- **Unit tests**: All existing inline tests become unit tests in separate files
- **Integration tests**: Existing `integration_tests.rs` remains unchanged
- **Test utilities**: If common test helpers are needed, they can be placed in `tests/common/mod.rs`

### Coverage Maintenance

Test coverage must remain identical:
- Same number of test functions
- Same test assertions
- Same test data and scenarios
- Same edge cases covered

## Implementation Notes

### Rust Testing Conventions

Rust supports two test locations:
1. **Inline tests** (`#[cfg(test)]` in source files): Traditional pattern, but increases file size
2. **Tests directory**: Separate test files that import from the library crate

The `tests/` directory is compiled as a separate crate, so:
- Tests can only access public items from the library
- Each file in `tests/` is a separate test binary
- Tests in `tests/` are integration-style tests by convention, but can test individual functions

### Making Items Public

Some items currently tested may not be public. Options:
1. Make them `pub` if they should be part of the public API
2. Make them `pub(crate)` if they should only be crate-internal
3. Restructure tests to test through public interfaces only

### Test Documentation

Existing test documentation comments (like those in `episode_field.rs`) should be preserved when moving tests to maintain traceability to requirements.

## Steering Documentation Updates

A new steering file `.kiro/steering/testing.md` will be created (or updated if it exists) with:

1. **Prohibition of inline tests**: Clear statement that `#[cfg(test)]` modules should not be added to source files
2. **Test file location**: All tests must be in the `tests/` directory
3. **Naming convention**: Test files must follow `{module_name}_tests.rs` pattern
4. **Import pattern**: Examples of how to import from the library crate
5. **Test organization**: Guidelines for structuring test files
6. **Rationale**: Explanation of why this pattern improves maintainability

Example content:

```markdown
# Testing Guidelines

## Test Organization

All unit tests must be placed in separate files in the `tests/` directory.

### Prohibited Pattern

Do NOT add inline test modules to source files:

```rust
// ❌ WRONG - Do not do this
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() { ... }
}
```

### Correct Pattern

Create a separate test file in `tests/`:

```rust
// ✅ CORRECT - tests/module_tests.rs
use movies::module::function_to_test;

#[test]
fn test_something() { ... }
}
```

### Naming Convention

Test files must follow the pattern: `{module_name}_tests.rs`

Examples:
- `config_tests.rs` for testing `src/config.rs`
- `database_tests.rs` for testing `src/database.rs`

### Importing from the Library

Tests import from the `movies` crate:

```rust
use movies::config::Config;
use movies::database::get_entries;
```

### Rationale

Separate test files:
- Keep source files focused and readable
- Make it easier to navigate large codebases
- Follow Rust community best practices for larger projects
- Maintain clear separation between implementation and tests
```
