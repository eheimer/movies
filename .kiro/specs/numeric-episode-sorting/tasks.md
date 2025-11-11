# Implementation Plan

- [x] 1. Update get_entries() function to use numeric episode sorting

  - Modify the SQL query ORDER BY clause to sort episode_number numerically
  - Use CASE expression to place NULL episode_numbers after numbered episodes
  - Use CAST to convert episode_number to INTEGER for numeric comparison
  - Maintain alphabetical name sorting as tertiary sort criterion
  - _Requirements: 1.1, 2.1, 2.2, 2.3, 3.1, 3.2, 3.5_

- [x] 2. Update get_entries_for_season() function to use numeric episode sorting
  - Modify the SQL query ORDER BY clause to sort episode_number numerically
  - Use CASE expression to place NULL episode_numbers after numbered episodes
  - Use CAST to convert episode_number to INTEGER for numeric comparison
  - Maintain alphabetical name sorting as tertiary sort criterion
  - _Requirements: 1.2, 2.1, 2.2, 2.3, 3.3, 3.4, 3.5_
