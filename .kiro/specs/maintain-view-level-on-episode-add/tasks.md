# Implementation Plan

- [x] 1. Update handler function signatures to accept view_context parameter

  - [x] 1.1 Add `view_context: &ViewContext` parameter to `handle_series_select_mode()` in `src/handlers.rs`

    - Modify function signature
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 3.1, 3.3_

  - [x] 1.2 Add `view_context: &ViewContext` parameter to `handle_series_create_mode()` in `src/handlers.rs`

    - Modify function signature
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 3.1, 3.3_

  - [x] 1.3 Add `view_context: &ViewContext` parameter to `handle_edit_mode()` in `src/handlers.rs`
    - Modify function signature
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 3.1, 3.3_

- [x] 2. Implement context-aware entry reloading in handle_series_select_mode()

  - [x] 2.1 Replace hardcoded `get_entries()` call with match statement on view_context
    - Implement pattern matching for TopLevel, Series, and Season contexts
    - Call appropriate database function for each context
    - Handle database errors by falling back to `get_entries()`
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 3.1, 3.3_

- [x] 3. Implement context-aware entry reloading in handle_series_create_mode()

  - [x] 3.1 Replace hardcoded `get_entries()` call with match statement on view_context
    - Implement pattern matching for TopLevel, Series, and Season contexts
    - Call appropriate database function for each context
    - Handle database errors by falling back to `get_entries()`
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 3.1, 3.3_

- [x] 4. Implement context-aware entry reloading in handle_edit_mode()

  - [x] 4.1 Replace conditional entry loading logic with match statement on view_context
    - Remove existing series/season-based conditional logic
    - Implement pattern matching for TopLevel, Series, and Season contexts
    - Call appropriate database function for each context
    - Handle database errors by falling back to `get_entries()`
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 3.1, 3.3_

- [x] 5. Update main_loop() to pass view_context to handler functions

  - [x] 5.1 Pass `&view_context` to `handle_edit_mode()` call in main_loop()

    - Add parameter to function call
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_

  - [x] 5.2 Pass `&view_context` to `handle_series_select_mode()` call in main_loop()

    - Add parameter to function call
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_

  - [x] 5.3 Pass `&view_context` to `handle_series_create_mode()` call in main_loop()
    - Add parameter to function call
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_

- [x] 6. Verify compilation and fix any type errors
  - [x] 6.1 Run `cargo check` to identify compilation errors
    - Fix any type mismatches or missing parameters
    - Ensure all function calls are updated with new signatures
    - _Requirements: 3.1, 3.2, 3.3, 3.4_
