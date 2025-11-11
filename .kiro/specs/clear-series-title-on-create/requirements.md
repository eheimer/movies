# Requirements Document

## Introduction

This document specifies the requirements for fixing a bug in the video library manager where creating a new series does not clear out previous text from the series title input field. When a user enters the series creation mode, any text that was previously entered should be cleared to provide a clean slate for entering the new series name.

## Glossary

- **Application**: The terminal-based video file browser and library manager
- **SeriesCreate Mode**: The application mode where users can enter a new series name
- **SeriesSelect Mode**: The application mode where users can select an existing series or create a new one
- **Series Title Field**: The text input field where users enter the name of a new series
- **Cursor Position**: The position of the text cursor within the series title field

## Requirements

### Requirement 1

**User Story:** As a user creating multiple series, I want the series title field to be empty each time I enter series creation mode, so that I don't have to manually delete previous text before entering a new series name.

#### Acceptance Criteria

1. WHEN the user transitions from SeriesSelect Mode to SeriesCreate Mode, THE Application SHALL clear the Series Title Field to an empty string
2. WHEN the user transitions from SeriesSelect Mode to SeriesCreate Mode, THE Application SHALL reset the Cursor Position to zero
3. WHEN the Series Title Field is cleared, THE Application SHALL trigger a screen redraw to display the empty field
4. WHEN the user cancels series creation and returns to SeriesSelect Mode, THE Application SHALL clear the Series Title Field to an empty string
5. WHEN the user successfully creates a series, THE Application SHALL clear the Series Title Field to an empty string before returning to Browse Mode
