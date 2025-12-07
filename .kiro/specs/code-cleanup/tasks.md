# Implementation Plan

- [x] 1. Fix compiler warnings in source code
  - Remove or use unused variables in `src/main.rs` and `src/handlers.rs`
  - Remove unused functions and types in `src/database.rs` and `src/video_metadata.rs`
  - Remove unused enum variants in `src/path_resolver.rs`
  - Verify zero warnings with `cargo build`
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [x] 2. Document doctest workaround
  - Add a developer note about the `LD_LIBRARY_PATH` workaround for running doctests
  - Include the workaround in a CONTRIBUTING.md or developer documentation section
  - _Requirements: 2.1, 2.2_

- [x] 3. Verify all tests pass
  - Run `cargo test` to ensure all unit and integration tests pass
  - Run `cargo test --doc` with the LD_LIBRARY_PATH workaround to verify doctest infrastructure works
  - _Requirements: 2.3_

- [x] 4. Rewrite README.md for non-technical users
  - Create a new README focused on movie enthusiasts with moderate technical ability
  - Use friendly, approachable language
  - Include practical examples and clear installation instructions
  - Explain features without deep technical jargon
  - Move technical/architectural details to a separate developer documentation file if needed
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

- [x] 5. Update steering documentation for accuracy
  - Fix `structure.md` to reference `config.yaml` instead of `config.json`
  - Verify all file paths and module names match current codebase
  - Update any outdated patterns or references
  - _Requirements: 4.1, 4.2_

- [x] 6. Simplify steering documentation
  - Reduce verbosity in `menu-features.md` (currently 300+ lines)
  - Consolidate redundant information across all steering files
  - Remove overly detailed examples where simpler ones suffice
  - Preserve essential patterns and conventions
  - Measure token count reduction by comparing file sizes
  - _Requirements: 4.3, 4.4, 4.5_

- [x] 7. Final verification
  - Run `cargo build` and confirm zero warnings
  - Run `cargo test` and confirm all tests pass
  - Review README with target audience in mind
  - Verify steering docs are accurate and concise
  - _Requirements: All_
