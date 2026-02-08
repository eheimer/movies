use crossterm::event::KeyCode;
use movies::handlers::handle_torrent_search_input;
use movies::torrent_search::TorrentResult;
use movies::util::Mode;

#[test]
fn test_handle_torrent_search_input_character_input() {
    let mut mode = Mode::TorrentSearchInput;
    let mut search_query = String::new();
    let mut torrent_results: Vec<TorrentResult> = Vec::new();
    let mut selected_torrent_result = 0;
    let mut redraw = false;

    // Test character input
    handle_torrent_search_input(
        KeyCode::Char('t'),
        &mut mode,
        &mut search_query,
        &mut torrent_results,
        &mut selected_torrent_result,
        &mut redraw,
    );

    assert_eq!(search_query, "t");
    assert!(redraw);
    assert_eq!(mode, Mode::TorrentSearchInput);
}

#[test]
fn test_handle_torrent_search_input_backspace() {
    let mut mode = Mode::TorrentSearchInput;
    let mut search_query = String::from("test");
    let mut torrent_results: Vec<TorrentResult> = Vec::new();
    let mut selected_torrent_result = 0;
    let mut redraw = false;

    // Test backspace
    handle_torrent_search_input(
        KeyCode::Backspace,
        &mut mode,
        &mut search_query,
        &mut torrent_results,
        &mut selected_torrent_result,
        &mut redraw,
    );

    assert_eq!(search_query, "tes");
    assert!(redraw);
    assert_eq!(mode, Mode::TorrentSearchInput);
}

#[test]
fn test_handle_torrent_search_input_escape() {
    let mut mode = Mode::TorrentSearchInput;
    let mut search_query = String::from("test");
    let mut torrent_results: Vec<TorrentResult> = Vec::new();
    let mut selected_torrent_result = 0;
    let mut redraw = false;

    // Test escape
    handle_torrent_search_input(
        KeyCode::Esc,
        &mut mode,
        &mut search_query,
        &mut torrent_results,
        &mut selected_torrent_result,
        &mut redraw,
    );

    assert_eq!(mode, Mode::Browse);
    assert!(redraw);
}

#[test]
fn test_handle_torrent_search_input_enter_empty_query() {
    let mut mode = Mode::TorrentSearchInput;
    let mut search_query = String::new();
    let mut torrent_results: Vec<TorrentResult> = Vec::new();
    let mut selected_torrent_result = 0;
    let mut redraw = false;

    // Test enter with empty query (should do nothing)
    handle_torrent_search_input(
        KeyCode::Enter,
        &mut mode,
        &mut search_query,
        &mut torrent_results,
        &mut selected_torrent_result,
        &mut redraw,
    );

    assert_eq!(mode, Mode::TorrentSearchInput);
    assert!(!redraw);
}

#[test]
fn test_handle_torrent_search_results_navigation_up() {
    use movies::handlers::handle_torrent_search_results;
    
    let mut mode = Mode::TorrentSearchResults;
    let mut selected_result = 2;
    let mut status_message = String::new();
    let mut redraw = false;
    
    let torrent_results = vec![
        TorrentResult {
            name: "Movie 1".to_string(),
            uploaded: "2024-01-01".to_string(),
            size: "1.5 GB".to_string(),
            seeders: 100,
            leechers: 50,
            magnet_link: "magnet:?xt=urn:btih:1".to_string(),
        },
        TorrentResult {
            name: "Movie 2".to_string(),
            uploaded: "2024-01-02".to_string(),
            size: "2.0 GB".to_string(),
            seeders: 200,
            leechers: 75,
            magnet_link: "magnet:?xt=urn:btih:2".to_string(),
        },
        TorrentResult {
            name: "Movie 3".to_string(),
            uploaded: "2024-01-03".to_string(),
            size: "1.8 GB".to_string(),
            seeders: 150,
            leechers: 60,
            magnet_link: "magnet:?xt=urn:btih:3".to_string(),
        },
    ];
    
    handle_torrent_search_results(
        KeyCode::Up,
        &mut mode,
        &torrent_results,
        &mut selected_result,
        &mut status_message,
        &mut redraw,
    );
    
    assert_eq!(selected_result, 1);
    assert!(redraw);
    assert_eq!(mode, Mode::TorrentSearchResults);
}

#[test]
fn test_handle_torrent_search_results_navigation_down() {
    use movies::handlers::handle_torrent_search_results;
    
    let mut mode = Mode::TorrentSearchResults;
    let mut selected_result = 0;
    let mut status_message = String::new();
    let mut redraw = false;
    
    let torrent_results = vec![
        TorrentResult {
            name: "Movie 1".to_string(),
            uploaded: "2024-01-01".to_string(),
            size: "1.5 GB".to_string(),
            seeders: 100,
            leechers: 50,
            magnet_link: "magnet:?xt=urn:btih:1".to_string(),
        },
        TorrentResult {
            name: "Movie 2".to_string(),
            uploaded: "2024-01-02".to_string(),
            size: "2.0 GB".to_string(),
            seeders: 200,
            leechers: 75,
            magnet_link: "magnet:?xt=urn:btih:2".to_string(),
        },
    ];
    
    handle_torrent_search_results(
        KeyCode::Down,
        &mut mode,
        &torrent_results,
        &mut selected_result,
        &mut status_message,
        &mut redraw,
    );
    
    assert_eq!(selected_result, 1);
    assert!(redraw);
    assert_eq!(mode, Mode::TorrentSearchResults);
}

#[test]
fn test_handle_torrent_search_results_navigation_boundaries() {
    use movies::handlers::handle_torrent_search_results;
    
    let mut mode = Mode::TorrentSearchResults;
    let mut selected_result = 0;
    let mut status_message = String::new();
    let mut redraw = false;
    
    let torrent_results = vec![
        TorrentResult {
            name: "Movie 1".to_string(),
            uploaded: "2024-01-01".to_string(),
            size: "1.5 GB".to_string(),
            seeders: 100,
            leechers: 50,
            magnet_link: "magnet:?xt=urn:btih:1".to_string(),
        },
    ];
    
    // Try to go up from first item (should do nothing)
    handle_torrent_search_results(
        KeyCode::Up,
        &mut mode,
        &torrent_results,
        &mut selected_result,
        &mut status_message,
        &mut redraw,
    );
    
    assert_eq!(selected_result, 0);
    
    // Try to go down from last item (should do nothing)
    redraw = false;
    handle_torrent_search_results(
        KeyCode::Down,
        &mut mode,
        &torrent_results,
        &mut selected_result,
        &mut status_message,
        &mut redraw,
    );
    
    assert_eq!(selected_result, 0);
    assert!(!redraw);
}

#[test]
fn test_handle_torrent_search_results_escape() {
    use movies::handlers::handle_torrent_search_results;
    
    let mut mode = Mode::TorrentSearchResults;
    let mut selected_result = 0;
    let mut status_message = String::new();
    let mut redraw = false;
    
    let torrent_results = vec![
        TorrentResult {
            name: "Movie 1".to_string(),
            uploaded: "2024-01-01".to_string(),
            size: "1.5 GB".to_string(),
            seeders: 100,
            leechers: 50,
            magnet_link: "magnet:?xt=urn:btih:1".to_string(),
        },
    ];
    
    handle_torrent_search_results(
        KeyCode::Esc,
        &mut mode,
        &torrent_results,
        &mut selected_result,
        &mut status_message,
        &mut redraw,
    );
    
    assert_eq!(mode, Mode::Browse);
    assert!(redraw);
}
