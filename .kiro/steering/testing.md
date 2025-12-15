# Testing Guidelines

## Test Organization

- All tests must be in separate files in the `tests/` directory
- Do NOT use inline `#[cfg(test)]` modules in source files
- Use naming pattern: `{module_name}_tests.rs` (e.g., `config_tests.rs` for `src/config.rs`)
- Import from the library crate: `use movies::module::Type;`

## Test Volume

- Keep tests minimal and focused on core functionality
- Avoid excessive edge case testing
- Do not include "Correctness Properties" or detailed "Test Cases" sections in design documents
- Focus on essential functionality testing only

## Spec Design Approach

- Design documents should focus on architecture, components, and interfaces
- Avoid detailed test case specifications in design documents
- Keep testing strategy sections brief and high-level
- Emphasize practical implementation over theoretical correctness
