# Implementation Plan

- [x] 1. Establish baseline and prepare library crate
  - Run `cargo test` to establish baseline (all tests should pass)
  - Review `src/lib.rs` to identify which modules are already exposed
  - Make note of any modules that need to be made public for test access
  - _Requirements: 1.3, 1.4_

- [x] 2. Move path_resolver tests
  - Create `tests/path_resolver_tests.rs`
  - Copy all test functions from `src/path_resolver.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::path_resolver::*`, `use tempfile::TempDir`, etc.)
  - Remove the `#[cfg(test)]` module from `src/path_resolver.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 3. Move episode_field tests
  - Create `tests/episode_field_tests.rs`
  - Copy all test functions from `src/episode_field.rs` `#[cfg(test)]` module, preserving test documentation comments
  - Add necessary imports (`use movies::episode_field::*`, `use movies::dto::EpisodeDetail`, etc.)
  - Remove the `#[cfg(test)]` module from `src/episode_field.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 4. Move config tests
  - Create `tests/config_tests.rs`
  - Copy all test functions from `src/config.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::config::*`, `use tempfile::TempDir`, etc.)
  - Remove the `#[cfg(test)]` module from `src/config.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 5. Move theme tests
  - Create `tests/theme_tests.rs`
  - Copy all test functions from `src/theme.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::theme::*`, etc.)
  - Remove the `#[cfg(test)]` module from `src/theme.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 6. Move menu tests
  - Create `tests/menu_tests.rs`
  - Copy all test functions from `src/menu.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::menu::*`, etc.)
  - Remove the `#[cfg(test)]` module from `src/menu.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 7. Move logger tests
  - Create `tests/logger_tests.rs`
  - Copy all test functions from `src/logger.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::logger::*`, `use tempfile::TempDir`, etc.)
  - Remove the `#[cfg(test)]` module from `src/logger.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 8. Move display tests
  - Create `tests/display_tests.rs`
  - Copy all test functions from `src/display.rs` `#[cfg(test)]` module
  - Add necessary imports (`use movies::display::*`, etc.)
  - Remove the `#[cfg(test)]` module from `src/display.rs`
  - Run `cargo test` to verify tests still pass
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4_

- [x] 9. Verify all inline tests removed
  - Search all source files in `src/` for `#[cfg(test)]` to confirm none remain
  - Run `cargo test` to verify complete test suite passes
  - Review test output to confirm all expected tests are running
  - _Requirements: 1.1, 1.3, 1.4_

- [x] 10. Update steering documentation
  - Create or update `.kiro/steering/testing.md` with guidelines prohibiting inline tests
  - Include correct test file naming convention (`{module_name}_tests.rs`)
  - Provide examples of proper import patterns from the library crate
  - Explain rationale for separate test files
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 11. Final verification
  - Run `cargo test` one final time to ensure all tests pass
  - Run `cargo clippy` to check for any warnings
  - Review all changes to confirm requirements are met
  - _Requirements: 1.3, 1.4_
