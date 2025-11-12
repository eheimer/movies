# Requirements Document

## Introduction

This feature introduces a context-sensitive menu system to replace the current always-visible menu items on the second header line. The menu will be accessible via F1 key and will display available actions for the currently selected item. This change improves screen real estate usage while maintaining easy access to all functionality through a collapsible menu interface.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **Header**: The top section of the terminal UI displaying instructions and menu options
- **Context Menu**: A collapsible menu that displays available actions for the currently selected item
- **Menu Item**: An individual action available in the context menu with an optional hotkey
- **Selection Cursor**: A visual indicator showing which menu item is currently selected
- **Focus**: The current input target (either the main list or the context menu)
- **Episode**: An individual video file entry in the Application

## Requirements

### Requirement 1

**User Story:** As a user, I want the second header line to show only `[F1] Menu` instead of all menu options, so that the interface is cleaner and less cluttered

#### Acceptance Criteria

1. THE Application SHALL display `[F1] Menu` on the second header line
2. THE Application SHALL maintain all existing first header line content unchanged
3. THE Application SHALL hide all context-sensitive menu items from the second header line that were previously visible

### Requirement 2

**User Story:** As a user, I want to press F1 to open a context menu, so that I can see all available actions for the selected item

#### Acceptance Criteria

1. WHEN the user presses F1 key, THE Application SHALL display the context menu
2. WHEN the context menu is displayed, THE Application SHALL remember the currently selected Episode
3. WHEN the context menu is displayed, THE Application SHALL transfer focus to the context menu
4. THE Application SHALL display the context menu surrounded by double-line ASCII characters
5. THE Application SHALL display each menu item on a separate line within the context menu

### Requirement 3

**User Story:** As a user, I want to navigate the context menu with arrow keys, so that I can select the action I want to perform

#### Acceptance Criteria

1. WHEN the context menu has focus, THE Application SHALL allow up arrow key to move the selection cursor to the previous menu item
2. WHEN the context menu has focus, THE Application SHALL allow down arrow key to move the selection cursor to the next menu item
3. THE Application SHALL display a selection cursor similar to the series selection list cursor for the currently selected menu item
4. WHEN the selection cursor is on the first menu item and up arrow is pressed, THE Application SHALL wrap the selection cursor to the last menu item
5. WHEN the selection cursor is on the last menu item and down arrow is pressed, THE Application SHALL wrap the selection cursor to the first menu item

### Requirement 4

**User Story:** As a user, I want to activate menu items using their hotkeys, so that I can quickly perform actions without navigating the menu

#### Acceptance Criteria

1. WHEN the context menu is displayed and a menu item has an assigned hotkey, THE Application SHALL activate that menu item when the hotkey is pressed
2. THE Application SHALL display the hotkey assignment for each menu item in the context menu
3. WHEN a hotkey is pressed for a menu item, THE Application SHALL execute the action and close the context menu

### Requirement 5

**User Story:** As a user, I want to close the context menu with ESC, so that I can return to browsing without performing an action

#### Acceptance Criteria

1. WHEN the context menu is displayed, THE Application SHALL change the ESC hotkey text to `[ESC] close menu`
2. WHEN the user presses ESC while the context menu is displayed, THE Application SHALL close the context menu
3. WHEN the context menu is closed, THE Application SHALL restore focus to the previously selected Episode
4. WHEN the context menu is closed, THE Application SHALL restore the ESC hotkey text to its original value

### Requirement 6

**User Story:** As a developer, I want a modular menu system, so that I can easily add new menu items to either the first header line or the context menu

#### Acceptance Criteria

1. THE Application SHALL provide a data structure that defines menu items with their properties including label, hotkey, and visibility location
2. THE Application SHALL provide a mechanism to determine which menu items are available based on the current context
3. THE Application SHALL separate the menu item definition from the rendering logic
4. THE Application SHALL allow menu items to be marked as either first-line or context-menu items
5. THE Application SHALL provide a simple interface for adding new menu items without modifying multiple code locations
