# Implementation Plan

- [x] 1. Remove unused struct fields from Entry enum

  - Remove `series_id` field from Entry::Season variant in src/util.rs
  - Remove `episode_number` field from Entry::Episode variant in src/util.rs
  - Update all database query functions in src/database.rs that create Entry::Season to remove series_id parameter
  - Update all database query functions in src/database.rs that create Entry::Episode to remove episode_number parameter
  - Update any usage in src/handlers.rs that references these fields
  - _Requirements: 1.1, 1.2, 3.1, 3.2_

- [x] 2. Remove unused variable assignment

  - Remove the initial assignment `let mut entry_path = String::new();` on line 33 of src/main.rs
  - Verify that entry_path is still properly initialized on line 62 when needed
  - _Requirements: 1.1, 1.2_

- [x] 3. Remove unused functions and obsolete comments

  - Remove the `import_episode` function from src/database.rs (line 110)
  - Remove the `pad_string_as_number` function from src/util.rs (line 74)
  - Remove the TODO comment above `pad_string_as_number` (lines 72-74 in src/util.rs)
  - _Requirements: 2.1, 2.2_

- [x] 4. Remove unused methods from PathResolver

  - Remove the `get_root_dir` method from src/path_resolver.rs (line 211)
  - Remove the `get_executable_dir` method from src/path_resolver.rs (line 219)
  - _Requirements: 2.1, 2.2_

- [x] 5. Verify cleanup is complete
  - Run `cargo build` and confirm zero warnings about unused code
  - Run `cargo test` to verify all 5 tests still pass
  - _Requirements: 4.1, 4.2_
