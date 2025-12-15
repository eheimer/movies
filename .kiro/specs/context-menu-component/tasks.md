# Implementation Plan

- [x] 1. Create context menu component structure
  - Create src/components/context_menu.rs file with ContextMenu struct
  - Implement Component trait with render method signature
  - Add basic constructor method
  - _Requirements: 1.1, 1.2, 2.1, 2.2_

- [x] 2. Implement core rendering logic
  - [x] 2.1 Implement menu dimension calculation
    - Write calculate_menu_dimensions method to determine width and height
    - Handle label and hotkey width calculations
    - _Requirements: 3.5_

  - [x] 2.2 Implement hotkey formatting
    - Write format_hotkey method for F-keys and character keys
    - Support [F2], [F3] format for function keys and [S], [E] for characters
    - _Requirements: 4.2_

  - [x] 2.3 Implement border rendering
    - Create border cells using double-line Unicode characters (╔, ╗, ╚, ╝, ═, ║)
    - Handle top, bottom, and side border generation
    - _Requirements: 3.4_

  - [x] 2.4 Implement menu item cell creation
    - Write create_menu_item_cells method for individual menu items
    - Handle left-justified labels and right-justified hotkeys
    - Apply selection highlighting using theme colors
    - _Requirements: 3.2, 3.3_

- [x] 3. Complete render method implementation
  - [x] 3.1 Integrate all rendering components
    - Combine border rendering and menu item rendering
    - Handle empty menu gracefully
    - Use saturating arithmetic for dimension calculations
    - _Requirements: 4.3, 4.5_

  - [x] 3.2 Add UTF-8 character handling
    - Ensure proper visual width calculation for multi-byte characters
    - Handle character counting vs byte counting correctly
    - _Requirements: 1.5_

  - [x] 3.3 Preserve menu item integrity
    - Render all provided menu items without filtering
    - Maintain original menu item order and content
    - _Requirements: 4.1_

- [ ]* 3.4 Write unit tests for context menu component
  - Test dimension calculation with various menu configurations
  - Test hotkey formatting for different key types
  - Test border character generation
  - Test label and hotkey alignment
  - Test selection highlighting
  - Test UTF-8 character handling
  - Test empty menu handling
  - Test arithmetic safety with small dimensions
  - _Requirements: 1.5, 2.4, 3.2, 3.3, 3.4, 3.5, 4.1, 4.2, 4.5_

- [x] 4. Update components module
  - Add context_menu module to src/components/mod.rs
  - Export ContextMenu struct from components module
  - _Requirements: 2.3_

- [x] 5. Integrate with display module
  - [x] 5.1 Replace draw_context_menu function usage
    - Update display.rs to use ContextMenu component
    - Remove old draw_context_menu function
    - Maintain identical visual behavior
    - _Requirements: 1.3, 1.4_

  - [x] 5.2 Update display rendering pipeline
    - Integrate ContextMenu component into existing rendering flow
    - Ensure proper positioning and theme application
    - _Requirements: 3.1_

- [ ]* 5.3 Write integration tests
  - Test display module integration
  - Test menu item compatibility with existing MenuItem types
  - Test theme compatibility
  - _Requirements: 1.4, 2.4_

- [x] 6. Final verification
  - Ensure all tests pass, ask the user if questions arise.