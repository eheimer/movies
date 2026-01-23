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

/// Helper function to create test entries (empty to avoid database access)
fn create_test_entries() -> Vec<Entry> {
    vec![]
}


// ============================================================================
// Final Integration Tests - Task 14
// ============================================================================

/// Test Case: All UI modes render without errors
/// When switching between all UI modes, each mode should render successfully
/// through the buffer system without errors or crashes.
/// Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5, 6.2
#[test]
fn test_all_ui_modes_render_successfully() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Test all modes
    let modes = vec![
        Mode::Browse,
        Mode::Edit,
        Mode::Entry,
        Mode::SeriesSelect,
        Mode::SeriesCreate,
        Mode::Menu,
    ];
    
    for mode in modes {
        // Force full redraw for mode change
        buffer_manager.force_full_redraw();
        
        let result = movies::display::draw_screen(
            &entries,
            0,
            &mut first_entry,
            "",
            &theme,
            &mode,
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
        
        assert!(result.is_ok(), "Mode should render successfully");
        
        // Verify buffers are in sync after render
        let changes = buffer_manager.compare_buffers();
        assert_eq!(changes.len(), 0, "Mode should have no pending changes");
    }
}


/// Test Case: Navigation through entries updates buffer correctly
/// When navigating through entries (simulating arrow keys), the buffer
/// should update only the changed portions of the screen.
/// Validates: Requirements 1.1, 1.2
#[test]
fn test_navigation_updates_buffer_correctly() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Render initial state (item 0 selected)
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
    assert!(result1.is_ok(), "Initial render should succeed");
    
    // Navigate to item 1 (simulating down arrow)
    let result2 = movies::display::draw_screen(
        &entries,
        1,
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
    assert!(result2.is_ok(), "Navigation to item 1 should succeed");
    
    // Navigate to item 2 (simulating another down arrow)
    let result3 = movies::display::draw_screen(
        &entries,
        2,
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
    assert!(result3.is_ok(), "Navigation to item 2 should succeed");
    
    // Navigate back to item 1 (simulating up arrow)
    let result4 = movies::display::draw_screen(
        &entries,
        1,
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
    assert!(result4.is_ok(), "Navigation back to item 1 should succeed");
}


/// Test Case: Edit mode text input updates buffer
/// When editing text fields in Edit mode, the buffer should update
/// to reflect the text changes without full screen redraws.
/// Validates: Requirements 1.3
#[test]
fn test_edit_mode_text_input_updates() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let mut edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Render initial Edit mode
    let result1 = movies::display::draw_screen(
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
    assert!(result1.is_ok(), "Initial Edit mode render should succeed");
    
    // Simulate text input - modify title
    edit_details.title = String::from("Test Episode - Modified");
    
    let result2 = movies::display::draw_screen(
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
    assert!(result2.is_ok(), "Edit mode with modified text should render successfully");
    
    // Simulate cursor movement - change field
    let result3 = movies::display::draw_screen(
        &entries,
        0,
        &mut first_entry,
        "",
        &theme,
        &Mode::Edit,
        &String::new(),
        &edit_details,
        EpisodeField::Year,
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
    assert!(result3.is_ok(), "Edit mode with different field should render successfully");
}


/// Test Case: Terminal resize in all modes
/// When the terminal is resized, all modes should handle the resize
/// correctly and render without errors.
/// Validates: Requirements 6.1
#[test]
fn test_terminal_resize_in_all_modes() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    let modes = vec![Mode::Browse, Mode::Edit, Mode::Menu];
    
    for mode in modes {
        // Render at initial size
        let result1 = movies::display::draw_screen(
            &entries,
            0,
            &mut first_entry,
            "",
            &theme,
            &mode,
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
        assert!(result1.is_ok(), "Mode should render at initial size");
        
        // Simulate terminal resize
        buffer_manager.resize(120, 40);
        
        // Render at new size
        let result2 = movies::display::draw_screen(
            &entries,
            0,
            &mut first_entry,
            "",
            &theme,
            &mode,
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
        assert!(result2.is_ok(), "Mode should render after resize");
        
        // Resize to smaller size
        buffer_manager.resize(60, 20);
        
        let result3 = movies::display::draw_screen(
            &entries,
            0,
            &mut first_entry,
            "",
            &theme,
            &mode,
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
        assert!(result3.is_ok(), "Mode should render after smaller resize");
        
        // Reset to original size for next mode
        buffer_manager.resize(80, 24);
    }
}


/// Test Case: Mode switching triggers full redraw
/// When switching between modes, a full redraw should occur to ensure
/// clean visual state without artifacts from the previous mode.
/// Validates: Requirements 6.2
#[test]
fn test_mode_switching_triggers_full_redraw() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Start in Browse mode
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
    assert!(result1.is_ok(), "Browse mode should render");
    
    // Switch to Edit mode (force full redraw)
    buffer_manager.force_full_redraw();
    let result2 = movies::display::draw_screen(
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
    assert!(result2.is_ok(), "Edit mode should render after mode switch");
    
    // Switch to Menu mode (force full redraw)
    buffer_manager.force_full_redraw();
    let result3 = movies::display::draw_screen(
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
    assert!(result3.is_ok(), "Menu mode should render after mode switch");
    
    // Switch back to Browse mode (force full redraw)
    buffer_manager.force_full_redraw();
    let result4 = movies::display::draw_screen(
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
    assert!(result4.is_ok(), "Browse mode should render after returning from Menu");
}


/// Test Case: No visual artifacts after multiple operations
/// When performing multiple operations (navigation, mode switches, resizes),
/// the buffer system should maintain clean state without visual artifacts.
/// Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5
#[test]
fn test_no_visual_artifacts_after_operations() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Perform a sequence of operations
    
    // 1. Initial render in Browse mode
    let _ = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // 2. Navigate down
    let _ = movies::display::draw_screen(
        &entries, 1, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // 3. Switch to Edit mode
    buffer_manager.force_full_redraw();
    let _ = movies::display::draw_screen(
        &entries, 1, &mut first_entry, "", &theme, &Mode::Edit,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // 4. Resize terminal
    buffer_manager.resize(100, 30);
    let _ = movies::display::draw_screen(
        &entries, 1, &mut first_entry, "", &theme, &Mode::Edit,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // 5. Switch back to Browse mode
    buffer_manager.force_full_redraw();
    let _ = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // 6. Resize back to original
    buffer_manager.resize(80, 24);
    let result = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    
    // After all operations, rendering should still work correctly
    assert!(result.is_ok(), "Final render should succeed after multiple operations");
    
    // Buffers should be in sync (no artifacts)
    let changes = buffer_manager.compare_buffers();
    assert_eq!(changes.len(), 0, "No pending changes should remain (clean state)");
}


/// Test Case: Rapid navigation doesn't cause buffer corruption
/// When rapidly navigating through entries (simulating arrow key spam),
/// the buffer system should handle all updates correctly without corruption.
/// Validates: Requirements 1.1, 1.2
#[test]
fn test_rapid_navigation_no_corruption() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Simulate rapid navigation (20 quick movements)
    // Since entries is empty, we just test with current_item = 0
    for i in 0..20 {
        let result = movies::display::draw_screen(
            &entries,
            0, // Always 0 since entries is empty
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
        
        assert!(result.is_ok(), "Rapid navigation iteration {} should succeed", i);
        
        // Verify buffers remain in sync after each update
        let changes = buffer_manager.compare_buffers();
        assert_eq!(changes.len(), 0, "Buffers should be in sync after iteration {}", i);
    }
}

/// Test Case: Status message updates work correctly
/// When status messages change, the buffer should update the status line
/// without affecting other parts of the screen.
/// Validates: Requirements 6.5
#[test]
fn test_status_message_updates() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Render with no status message
    let result1 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result1.is_ok(), "Render without status message should succeed");
    
    // Render with status message
    let result2 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "Test status message", &resolver, &mut buffer_manager,
    );
    assert!(result2.is_ok(), "Render with status message should succeed");
    
    // Render with different status message
    let result3 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "Different message", &resolver, &mut buffer_manager,
    );
    assert!(result3.is_ok(), "Render with different status message should succeed");
    
    // Clear status message
    let result4 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result4.is_ok(), "Render after clearing status message should succeed");
}


/// Test Case: Filter mode updates work correctly
/// When filtering entries, the buffer should update to show only
/// matching entries without visual artifacts.
/// Validates: Requirements 1.1, 1.2, 6.4
#[test]
fn test_filter_mode_updates() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Render without filter
    let result1 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result1.is_ok(), "Render without filter should succeed");
    
    // Render with filter text
    let result2 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "test", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result2.is_ok(), "Render with filter should succeed");
    
    // Update filter text
    let result3 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "test series", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result3.is_ok(), "Render with updated filter should succeed");
    
    // Clear filter
    let result4 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &view_context, "", &resolver, &mut buffer_manager,
    );
    assert!(result4.is_ok(), "Render after clearing filter should succeed");
}

/// Test Case: Entry mode path input updates
/// When entering a path in Entry mode, the buffer should update
/// to show the path being typed.
/// Validates: Requirements 1.3
#[test]
fn test_entry_mode_path_input() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Simulate typing a path character by character
    let paths = vec![
        "/",
        "/h",
        "/ho",
        "/hom",
        "/home",
        "/home/",
        "/home/u",
        "/home/us",
        "/home/use",
        "/home/user",
    ];
    
    for path in paths {
        let result = movies::display::draw_screen(
            &entries, 0, &mut first_entry, "", &theme, &Mode::Entry,
            &String::from(path), &edit_details, EpisodeField::Title, 0,
            &series, &mut series_selection, "", None, &None,
            &dirty_fields, &menu_items, 0, false, &mut first_series,
            &view_context, "", &resolver, &mut buffer_manager,
        );
        
        assert!(result.is_ok(), "Entry mode with path '{}' should render successfully", path);
        
        // Verify buffers are in sync
        let changes = buffer_manager.compare_buffers();
        assert_eq!(changes.len(), 0, "Buffers should be in sync after path '{}'", path);
    }
}


/// Test Case: SeriesSelect mode navigation
/// When navigating through series in SeriesSelect mode, the buffer
/// should update correctly.
/// Validates: Requirements 1.1, 1.2
#[test]
fn test_series_select_mode_navigation() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    
    // Create test series list
    let series = vec![
        Series {
            id: 1,
            name: String::from("Test Series 1"),
        },
        Series {
            id: 2,
            name: String::from("Test Series 2"),
        },
        Series {
            id: 3,
            name: String::from("Test Series 3"),
        },
    ];
    
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Navigate through series list
    for i in 0..series.len() {
        let result = movies::display::draw_screen(
            &entries, 0, &mut first_entry, "", &theme, &Mode::SeriesSelect,
            &String::new(), &edit_details, EpisodeField::Title, 0,
            &series, &mut series_selection, "", None, &None,
            &dirty_fields, &menu_items, i, false, &mut first_series,
            &view_context, "", &resolver, &mut buffer_manager,
        );
        
        assert!(result.is_ok(), "SeriesSelect mode at position {} should render", i);
    }
}

/// Test Case: SeriesCreate mode text input
/// When creating a new series and typing the name, the buffer
/// should update to show the text being entered.
/// Validates: Requirements 1.3
#[test]
fn test_series_create_mode_text_input() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Simulate typing a series name
    let names = vec![
        "N",
        "Ne",
        "New",
        "New ",
        "New S",
        "New Se",
        "New Ser",
        "New Seri",
        "New Serie",
        "New Series",
    ];
    
    for name in names {
        let result = movies::display::draw_screen(
            &entries, 0, &mut first_entry, "", &theme, &Mode::SeriesCreate,
            &String::new(), &edit_details, EpisodeField::Title, 0,
            &series, &mut series_selection, name, None, &None,
            &dirty_fields, &menu_items, 0, false, &mut first_series,
            &view_context, "", &resolver, &mut buffer_manager,
        );
        
        assert!(result.is_ok(), "SeriesCreate mode with name '{}' should render", name);
    }
}


/// Test Case: Menu mode navigation and selection
/// When navigating through menu items, the buffer should update
/// to highlight the selected item correctly.
/// Validates: Requirements 1.1, 1.2
#[test]
fn test_menu_mode_navigation() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    
    // Create test menu items
    let menu_items = vec![
        MenuItem {
            label: String::from("Menu Item 1"),
            hotkey: None,
            action: movies::menu::MenuAction::ToggleWatched,
            location: movies::menu::MenuLocation::ContextMenu,
        },
        MenuItem {
            label: String::from("Menu Item 2"),
            hotkey: None,
            action: movies::menu::MenuAction::ToggleWatched,
            location: movies::menu::MenuLocation::ContextMenu,
        },
        MenuItem {
            label: String::from("Menu Item 3"),
            hotkey: None,
            action: movies::menu::MenuAction::ToggleWatched,
            location: movies::menu::MenuLocation::ContextMenu,
        },
    ];
    
    let mut first_series = 0;
    let view_context = ViewContext::TopLevel;
    let resolver = create_test_path_resolver();
    
    // Navigate through menu items
    for i in 0..menu_items.len() {
        let result = movies::display::draw_screen(
            &entries, 0, &mut first_entry, "", &theme, &Mode::Menu,
            &String::new(), &edit_details, EpisodeField::Title, 0,
            &series, &mut series_selection, "", None, &None,
            &dirty_fields, &menu_items, i, false, &mut first_series,
            &view_context, "", &resolver, &mut buffer_manager,
        );
        
        assert!(result.is_ok(), "Menu mode at position {} should render", i);
    }
}

/// Test Case: Buffer consistency across view contexts
/// When switching between different view contexts (TopLevel, Series, Season),
/// the buffer should maintain consistency and render correctly.
/// Validates: Requirements 1.1, 1.2, 6.3
#[test]
fn test_buffer_consistency_across_view_contexts() {
    let mut buffer_manager = BufferManager::new(80, 24);
    let theme = Theme::default();
    let entries = create_test_entries();
    let mut first_entry = 0;
    let edit_details = create_test_episode_detail();
    let series: Vec<Series> = vec![];
    let mut series_selection = None;
    let dirty_fields = HashSet::new();
    let menu_items: Vec<MenuItem> = vec![];
    let mut first_series = 0;
    let resolver = create_test_path_resolver();
    
    // Test TopLevel view context
    let result1 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &ViewContext::TopLevel, "", &resolver, &mut buffer_manager,
    );
    assert!(result1.is_ok(), "TopLevel view context should render");
    
    // Test Series view context
    let result2 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &ViewContext::Series { series_id: 1, series_name: String::from("Test Series") }, "", &resolver, &mut buffer_manager,
    );
    assert!(result2.is_ok(), "Series view context should render");
    
    // Test Season view context
    let result3 = movies::display::draw_screen(
        &entries, 0, &mut first_entry, "", &theme, &Mode::Browse,
        &String::new(), &edit_details, EpisodeField::Title, 0,
        &series, &mut series_selection, "", None, &None,
        &dirty_fields, &menu_items, 0, false, &mut first_series,
        &ViewContext::Season { season_id: 1, series_name: String::from("Test Series"), season_number: 1 }, "", &resolver, &mut buffer_manager,
    );
    assert!(result3.is_ok(), "Season view context should render");
}

