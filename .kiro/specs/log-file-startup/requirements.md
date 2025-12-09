# Requirements Document

**GitHub Issue:** #43

## Introduction

Remove the startup prompt that asks users whether to save the existing log file. The application should start immediately without user interaction, automatically truncating and recreating the log file on each startup.

## Glossary

- **Application**: The movies terminal-based video library manager
- **Log File**: The file specified in config.yaml where application logs are written
- **Startup**: The initialization phase when the application begins execution

## Requirements

### Requirement 1

**User Story:** As a user, I want the application to start immediately without prompts, so that I can begin using it without interruption.

#### Acceptance Criteria

1. WHEN the application starts THEN the Application SHALL initialize without displaying any prompts to the user
2. WHEN the application starts and a log file exists THEN the Application SHALL truncate and recreate the log file without user confirmation
3. WHEN the application starts THEN the Application SHALL not preserve previous log file contents
