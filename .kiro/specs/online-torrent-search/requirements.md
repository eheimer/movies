# Requirements Document

## Introduction

This feature adds the ability to search for movies on The Pirate Bay torrent site directly from within the application. Users can search for content not in their library, browse results, and initiate downloads via magnet links that open in their system's default torrent client.

## Glossary

- **Torrent_Search_System**: The component responsible for querying torrent providers and displaying results
- **Magnet_Link**: A URI scheme that identifies torrent files by their content hash
- **Torrent_Client**: External application (e.g., Transmission, qBittorrent) registered with the OS to handle magnet links
- **Search_Provider**: The torrent indexing service (The Pirate Bay) used to find content
- **Browse_Mode**: The main application mode for navigating the video library

## Requirements

### Requirement 1: Menu Access

**User Story:** As a user, I want to access the online search feature from the menu, so that I can find content not in my library.

#### Acceptance Criteria

1. WHEN the user is in Browse_Mode, THE Torrent_Search_System SHALL display a "Search Online" menu item
2. WHEN the user is not in Browse_Mode, THE Torrent_Search_System SHALL hide the "Search Online" menu item
3. WHEN the user selects the "Search Online" menu item, THE Torrent_Search_System SHALL transition to the search input mode

### Requirement 2: Search Input

**User Story:** As a user, I want to enter a movie title to search for, so that I can find specific content.

#### Acceptance Criteria

1. WHEN the search input mode is active, THE Torrent_Search_System SHALL display an input field for the search query
2. WHEN the search input mode is active, THE Torrent_Search_System SHALL display text indicating "Searching: The Pirate Bay"
3. WHEN the user types characters, THE Torrent_Search_System SHALL append them to the search query
4. WHEN the user presses Enter with a non-empty query, THE Torrent_Search_System SHALL initiate a search and transition to results mode
5. WHEN the user presses ESC, THE Torrent_Search_System SHALL cancel the search and return to Browse_Mode

### Requirement 3: Search Execution

**User Story:** As a user, I want the application to search The Pirate Bay for movies, so that I can find torrents to download.

#### Acceptance Criteria

1. WHEN a search is initiated, THE Torrent_Search_System SHALL query The Pirate Bay using the provided search term
2. WHEN querying the Search_Provider, THE Torrent_Search_System SHALL filter results to the movies/video category only
3. WHEN search results are received, THE Torrent_Search_System SHALL retain only the top 5 results by seeders
4. WHEN the search fails, THE Torrent_Search_System SHALL display an error message and return to Browse_Mode

### Requirement 4: Results Display

**User Story:** As a user, I want to see search results with relevant information, so that I can choose the best torrent to download.

#### Acceptance Criteria

1. WHEN displaying search results, THE Torrent_Search_System SHALL show the torrent title
2. WHEN displaying search results, THE Torrent_Search_System SHALL show the upload date
3. WHEN displaying search results, THE Torrent_Search_System SHALL show the file size
4. WHEN displaying search results, THE Torrent_Search_System SHALL show the seeder count
5. WHEN displaying search results, THE Torrent_Search_System SHALL show the leecher count
6. WHEN displaying search results, THE Torrent_Search_System SHALL limit the display to 5 results maximum

### Requirement 5: Result Selection

**User Story:** As a user, I want to navigate and select search results, so that I can choose which torrent to download.

#### Acceptance Criteria

1. WHEN in results mode, THE Torrent_Search_System SHALL highlight the currently selected result
2. WHEN the user presses the down arrow key, THE Torrent_Search_System SHALL move selection to the next result
3. WHEN the user presses the up arrow key, THE Torrent_Search_System SHALL move selection to the previous result
4. WHEN the user presses ESC, THE Torrent_Search_System SHALL cancel and return to Browse_Mode
5. WHEN the user presses Enter on a selected result, THE Torrent_Search_System SHALL initiate the download

### Requirement 6: Download Initiation

**User Story:** As a user, I want to open magnet links in my torrent client, so that I can download the selected content.

#### Acceptance Criteria

1. WHEN the user selects a result and presses Enter, THE Torrent_Search_System SHALL extract the Magnet_Link from the selected result
2. WHEN opening a Magnet_Link, THE Torrent_Search_System SHALL use the operating system's default handler for magnet URIs
3. WHEN a download is initiated, THE Torrent_Search_System SHALL return to Browse_Mode
4. WHEN a download is initiated, THE Torrent_Search_System SHALL display a status message "Initiated download: <torrent name>"
5. IF the Magnet_Link cannot be opened, THEN THE Torrent_Search_System SHALL display an error message and remain in results mode
