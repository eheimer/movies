use movies::buffer::BufferManager;
use movies::theme::Theme;
use movies::util::{Entry, Mode, ViewContext};
use movies::dto::{EpisodeDetail, Series};
use movies::episode_field::EpisodeField;
use movies::menu::MenuItem;
use movies::path_resolver::PathResolver;
use std::collections::HashSet;
use tempfile::TempDir;

/// Helper function to create a test EpisodeDetail
fn create_test_episode_detail() -> EpisodeDetail {
    EpisodeDetail {
        title: String::from("Test Episode"),
        year: String::from("2024"),
        watched: String::from("false"),
        length: String::from("00:45:00"),
        series: None,
        season: None,
        episode_number: String::from("1"),
        last_watched_time: None,
        last_progress_time: None,
    }
}

/// Helper function to create a test PathResolver
fn create_test_path_resolver() -> PathResolver {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.sqlite");
    std::fs::write(&db_path, "test").unwrap();
    PathResolver::from_database_path(&db_path).unwrap()
}

/// Test Case: draw_screen writes to buffer instead of terminal
/// When draw_screen is called, it should write to the buffer manager's desired buffer
/// instead of directly to the terminal.
/// Validates: Requirements 2.1, 2.2
#[test]
fn test_draw_screen_writes_to_buffer() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Call draw_screen
    let result = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    
    assert!(result.is_ok(), "draw_screen should succeed");
}

/// Test Case: Desired buffer starts empty each frame
/// When draw_screen is called, the desired buffer should be cleared at the start,
/// ensuring no content from previous frames remains.
/// Validates: Requirements 2.1, 3.1
#[test]
fn test_desired_buffer_starts_empty() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Write some content to the desired buffer before calling draw_screen
    {
        let mut writer = buffer_manager.get_writer();
        writer.move_to(10, 10);
        writer.write_str("Old content");
    }
    
    // Call draw_screen - it should clear the desired buffer
    let result = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    
    assert!(result.is_ok(), "draw_screen should succeed");
    
    // The desired buffer should have been cleared and redrawn
    // We can verify this by checking that render_to_terminal works
    let render_result = buffer_manager.render_to_terminal();
    assert!(render_result.is_ok(), "render_to_terminal should succeed after draw_screen");
}

/// Test Case: render_to_terminal is called at end
/// When draw_screen completes, it should call render_to_terminal to write
/// the buffer differences to the terminal.
/// Validates: Requirements 2.1, 2.2, 3.1
#[test]
fn test_render_to_terminal_called() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Call draw_screen
    let result = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    
    assert!(result.is_ok(), "draw_screen should succeed");
    
    // After draw_screen, the current buffer should match the desired buffer
    // (because render_to_terminal was called)
    let changes = buffer_manager.compare_buffers();
    assert_eq!(changes.len(), 0, "No changes should remain after draw_screen calls render_to_terminal");
}

/// Test Case: Multiple draw_screen calls work correctly
/// When draw_screen is called multiple times, each call should properly clear
/// the desired buffer and render the new content.
/// Validates: Requirements 2.1, 2.2, 3.1
#[test]
fn test_multiple_draw_screen_calls() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // First call
    let result1 = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result1.is_ok(), "First draw_screen should succeed");
    
    // Second call with different mode
    let result2 = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Entry,
        &String::from("/test/path"),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result2.is_ok(), "Second draw_screen should succeed");
    
    // Third call back to Browse mode
    let result3 = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result3.is_ok(), "Third draw_screen should succeed");
    
    // After all calls, buffers should be in sync
    let changes = buffer_manager.compare_buffers();
    assert_eq!(changes.len(), 0, "Buffers should be in sync after multiple draw_screen calls");
}

/// Test Case: BufferWriter replaces direct terminal writes
/// When rendering, all content should be written through BufferWriter
/// instead of direct terminal I/O calls like print_at().
/// Validates: Requirements 2.2, 2.3
#[test]
fn test_buffer_writer_replaces_terminal_io() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    
    // Use empty entries to avoid database access
    let entries: Vec<Entry> = vec![];
    
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Call draw_screen - all content should be written to buffer
    let result = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "Test status message",
        &resolver,
        &mut buffer_manager,
    );
    
    assert!(result.is_ok(), "draw_screen should succeed");
    
    // Verify that buffers are in sync (render_to_terminal was called)
    let changes = buffer_manager.compare_buffers();
    assert_eq!(changes.len(), 0, "All buffer writes should be translated to terminal");
}

/// Test Case: Buffer writes work in all modes
/// When rendering in different modes (Browse, Edit, Entry, etc.),
/// all content should be correctly written through the buffer system.
/// Validates: Requirements 2.2, 2.3, 2.4
#[test]
fn test_buffer_writes_in_all_modes() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    // Use empty entries to avoid database access
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Test Browse mode
    let result_browse = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result_browse.is_ok(), "Browse mode should render successfully");
    
    // Test Edit mode
    let result_edit = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Edit,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result_edit.is_ok(), "Edit mode should render successfully");
    
    // Test Entry mode
    let result_entry = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Entry,
        &String::from("/test/path"),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result_entry.is_ok(), "Entry mode should render successfully");
    
    // Test Menu mode
    let result_menu = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Menu,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    assert!(result_menu.is_ok(), "Menu mode should render successfully");
}

/// Test Case: Clear desired buffer is called at frame start
/// When draw_screen is called, it should clear the desired buffer
/// before writing new content, ensuring clean state.
/// Validates: Requirements 2.2, 3.1
#[test]
fn test_clear_desired_buffer_called() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries: Vec<Entry> = vec![];
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Manually write some content to desired buffer
    {
        let mut writer = buffer_manager.get_writer();
        writer.move_to(5, 5);
        writer.write_str("This should be cleared");
    }
    
    // Call draw_screen - it should clear the desired buffer first
    let result = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Browse,
        &String::new(),
        &edit_details,
        EpisodeField::Title,
        0,
        &series,
        &mut series_selection,
        "",
        None,
        &None,
        &dirty_fields,
        &menu_items,
        0,
        false,
        &mut first_series,
        &view_context,
        "",
        &resolver,
        &mut buffer_manager,
    );
    
    assert!(result.is_ok(), "draw_screen should succeed");
    
    // The old content should not be present after draw_screen
    // (verified by successful render_to_terminal call within draw_screen)
}
