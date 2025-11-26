# Requirements Document

## Introduction

This feature reorganizes the menu helper display system to simplify the UI by consolidating all menu helpers onto a single line. The [F1] Menu helper will be moved to the front of the first line for better visibility, the [CTRL][L] rescan functionality will be moved into the context menu with a new [S] hotkey, and all support for a second line of menu helpers will be removed. Items that don't fit on the first line will automatically appear in the context menu instead.

## Glossary

- **Menu Helper**: A keyboard shortcut indicator displayed in the terminal UI (e.g., "[F1] Menu", "[CTRL][L] rescan")
- **First Line**: The primary horizontal line in the header where menu helpers are displayed
- **Context Menu**: The F1 menu that displays available actions based on the current context
- **Menu System**: The centralized system for managing menu items, their locations, hotkeys, and availability
- **MenuLocation**: An enum that determines where a menu item appears (FirstLine or ContextMenu)

## Requirements

### Requirement 1

**User Story:** As a user, I want the [F1] Menu helper to appear at the front of the first line, so that I can easily find the menu access point.

#### Acceptance Criteria

1. WHEN the application renders the header, THE Menu System SHALL display the [F1] Menu helper as the first item on the first line
2. THE Menu System SHALL maintain the [F1] Menu helper position regardless of other menu items present
3. THE Menu System SHALL ensure the [F1] Menu helper remains visible at all times during normal operation

### Requirement 2

**User Story:** As a user, I want the rescan functionality moved to the context menu with an [S] hotkey, so that I can access it through the menu system instead of a CTRL combination.

#### Acceptance Criteria

1. THE Menu System SHALL remove the [CTRL][L] rescan helper from the first line display
2. THE Menu System SHALL add a rescan menu item to the context menu with [S] as the hotkey
3. WHEN the user presses [S] in browse mode, THE Menu System SHALL execute the rescan operation
4. WHEN the user opens the F1 menu and presses [S], THE Menu System SHALL execute the rescan operation
5. THE Menu System SHALL remove all handling for the [CTRL][L] hotkey combination for rescan

### Requirement 3

**User Story:** As a user, I want all menu helpers consolidated on a single line, so that the interface is cleaner and less cluttered.

#### Acceptance Criteria

1. THE Display Module SHALL render all menu helpers on a single line in the header
2. THE Display Module SHALL remove all code that renders a second line of menu helpers
3. THE Display Module SHALL remove all logic that determines which items appear on a second line
4. THE Menu System SHALL place menu items that exceed the first line width into the context menu instead

### Requirement 4

**User Story:** As a user, I want menu items that don't fit on the first line to automatically appear in the context menu, so that I can still access all functionality without visual overflow.

#### Acceptance Criteria

1. THE Menu System SHALL calculate the available width for first-line menu helpers
2. WHEN the total width of first-line menu items exceeds the available terminal width, THE Menu System SHALL move overflow items to the context menu
3. THE Menu System SHALL prioritize items based on their defined order when determining which items fit on the first line
4. THE Menu System SHALL ensure the [F1] Menu helper always remains on the first line regardless of width constraints
5. THE Menu System SHALL update the context menu dynamically to include overflow items
