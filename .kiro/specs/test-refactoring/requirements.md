# Requirements Document

**GitHub Issue:** #49

## Introduction

The video library manager codebase currently contains inline unit tests within source files using `#[cfg(test)]` modules. As the codebase grows, these inline tests are making source files increasingly long and difficult to navigate. This feature will refactor the test organization by moving all inline unit tests to separate test files following Rust best practices, while maintaining test coverage and ensuring all tests continue to pass.

## Glossary

- **Inline Tests**: Unit tests defined within `#[cfg(test)]` modules at the end of source files
- **Test Module**: A separate `.rs` file in the `tests/` directory containing unit tests
- **Source File**: A `.rs` file in the `src/` directory containing application code
- **Test Coverage**: The set of functionality validated by unit tests

## Requirements

### Requirement 1

**User Story:** As a developer, I want unit tests moved to separate test files, so that source files remain focused and readable.

#### Acceptance Criteria

1. WHEN a developer opens a source file in the `src/` directory THEN the System SHALL contain only application code without `#[cfg(test)]` modules
2. WHEN tests are moved to separate files THEN the System SHALL preserve all existing test cases without modification to test logic
3. WHEN tests are relocated THEN the System SHALL maintain the same test coverage as before the refactoring
4. WHEN running `cargo test` THEN the System SHALL execute all tests successfully with the same pass/fail results as before refactoring

### Requirement 2

**User Story:** As a developer, I want tests organized in a clear structure, so that I can easily find and maintain tests for specific modules.

#### Acceptance Criteria

1. WHEN tests are moved THEN the System SHALL create test files in the `tests/` directory with names matching their corresponding source modules
2. WHEN a test file is created THEN the System SHALL use the naming pattern `{module_name}_tests.rs` for clarity
3. WHEN tests require access to module internals THEN the System SHALL properly import the necessary items from the library crate
4. WHEN multiple modules have tests THEN the System SHALL create separate test files for each module rather than combining them

### Requirement 3

**User Story:** As a developer, I want steering documentation updated, so that future tests follow the new organizational pattern.

#### Acceptance Criteria

1. WHEN steering documentation is updated THEN the System SHALL include guidelines prohibiting inline `#[cfg(test)]` modules in source files
2. WHEN steering documentation is updated THEN the System SHALL specify that new tests must be created in separate files in the `tests/` directory
3. WHEN steering documentation is updated THEN the System SHALL provide examples of the correct test file structure and naming conventions
4. WHEN steering documentation is updated THEN the System SHALL explain how to import and test module functionality from separate test files
