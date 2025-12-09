# Design Document

## Overview

This feature removes the interactive startup prompt that asks users to save existing log files. The application will automatically truncate and recreate the log file on each startup, eliminating the need for user interaction during initialization.

## Architecture

The change affects two components:
1. **main.rs**: Remove the call to `prompt_save_existing_log()` and the conditional logic around it
2. **logger.rs**: Remove the `prompt_save_existing_log()` function as it will no longer be used

The existing `initialize_logger()` function already truncates the log file when opening it (using `.truncate(true)`), so no changes are needed to that function.

## Components and Interfaces

### Modified Components

**main.rs**
- Remove the check for existing log file
- Remove the call to `logger::prompt_save_existing_log()`
- Directly call `logger::initialize_logger()` without conditional logic

**logger.rs**
- Remove the `prompt_save_existing_log()` function
- No changes needed to `initialize_logger()` as it already truncates the file

### Unchanged Components

- `initialize_logger()`: Already uses `.truncate(true)` which overwrites existing files
- All logging functions (`log_error`, `log_warn`, `log_info`, `log_debug`)
- Log level filtering logic

## Data Models

No data model changes required.

## Test Cases

### Test Case 1: Application starts without prompts

When the application starts with an existing log file, the system should initialize without displaying any prompts.
**Validates: Requirements 1.1**

### Test Case 2: Log file truncation

When the application starts and a log file exists, the system should truncate and recreate the log file automatically.
**Validates: Requirements 1.2**

### Test Case 3: No log preservation

When the application starts, previous log file contents should not be preserved.
**Validates: Requirements 1.3**

## Error Handling

No new error handling required. The existing error handling in `initialize_logger()` will continue to handle file creation and write errors.

## Testing Strategy

No new tests are required for this change. This is a removal of functionality.

If any existing tests verify the `prompt_save_existing_log()` function behavior, they should be removed along with the function.
