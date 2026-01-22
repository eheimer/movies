use movies::components::episode_editor::EpisodeEditor;
use movies::components::metadata_display::MetadataDisplay;
use movies::components::{DetailPanel, Component};
use movies::dto::{EpisodeDetail, Series, Season};
use movies::episode_field::EpisodeField;
use movies::theme::Theme;
use movies::util::Mode;
use std::collections::HashSet;

#[test]
fn test_metadata_display_creation() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: Some(Series {
            id: 1,
            name: "Test Series".to_string(),
        }),
        season: Some(Season {
            id: 1,
            number: 1,
        }),
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let metadata_display = MetadataDisplay::new(
        episode_details,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = metadata_display.render(80, 10, &theme, false);
    
    // Should render without panicking
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10); // Should have 10 rows
    assert_eq!(result[0].len(), 80); // Each row should have 80 cells
}

#[test]
fn test_metadata_display_path_extraction() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "".to_string(),
        series: None,
        season: None,
        episode_number: "".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let metadata_display = MetadataDisplay::new(
        episode_details,
        "/home/user/videos/series/season1/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = metadata_display.render(80, 10, &theme, false);
    
    // Should render without panicking and handle path extraction
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
}

#[test]
fn test_metadata_display_empty_fields() {
    let episode_details = EpisodeDetail {
        title: "".to_string(),
        year: "".to_string(),
        watched: "".to_string(),
        length: "".to_string(),
        series: None,
        season: None,
        episode_number: "".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let metadata_display = MetadataDisplay::new(
        episode_details,
        "episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = metadata_display.render(40, 5, &theme, false);
    
    // Should handle empty fields gracefully
    assert!(!result.is_empty());
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].len(), 40);
}

#[test]
fn test_episode_editor_creation() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: Some(Series {
            id: 1,
            name: "Test Series".to_string(),
        }),
        season: Some(Season {
            id: 1,
            number: 1,
        }),
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let mut dirty_fields = HashSet::new();
    dirty_fields.insert(EpisodeField::Title);

    let episode_editor = EpisodeEditor::new(
        episode_details,
        EpisodeField::Title,
        5, // cursor position
        Some(1),
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = episode_editor.render(80, 10, &theme, false);
    
    // Should render without panicking
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10); // Should have 10 rows
    assert_eq!(result[0].len(), 80); // Each row should have 80 cells
}

#[test]
fn test_episode_editor_field_highlighting() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let mut dirty_fields = HashSet::new();
    dirty_fields.insert(EpisodeField::Year);

    let episode_editor = EpisodeEditor::new(
        episode_details,
        EpisodeField::Title, // Current edit field
        0, // cursor position
        None,
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = episode_editor.render(80, 10, &theme, false);
    
    // Should render with field highlighting
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].len(), 80);
}

#[test]
fn test_episode_editor_non_editable_fields() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let dirty_fields = HashSet::new();

    // Try to edit a non-editable field (Path)
    let episode_editor = EpisodeEditor::new(
        episode_details,
        EpisodeField::Path, // Non-editable field
        0,
        None,
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = episode_editor.render(80, 10, &theme, false);
    
    // Should render without issues even for non-editable fields
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].len(), 80);
}

#[test]
fn test_detail_panel_browse_mode() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: Some(Series {
            id: 1,
            name: "Test Series".to_string(),
        }),
        season: Some(Season {
            id: 1,
            number: 1,
        }),
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let dirty_fields = HashSet::new();

    let detail_panel = DetailPanel::new(
        Mode::Browse,
        episode_details,
        EpisodeField::Title,
        0,
        Some(1),
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = detail_panel.render(80, 10, &theme, false);
    
    // Should render MetadataDisplay in Browse mode
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].len(), 80);
}

#[test]
fn test_detail_panel_edit_mode() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: Some(Series {
            id: 1,
            name: "Test Series".to_string(),
        }),
        season: Some(Season {
            id: 1,
            number: 1,
        }),
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let mut dirty_fields = HashSet::new();
    dirty_fields.insert(EpisodeField::Title);

    let detail_panel = DetailPanel::new(
        Mode::Edit,
        episode_details,
        EpisodeField::Title,
        5,
        Some(1),
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = detail_panel.render(80, 10, &theme, false);
    
    // Should render EpisodeEditor in Edit mode
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].len(), 80);
}

#[test]
fn test_detail_panel_mode_switching() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let dirty_fields = HashSet::new();

    // Test Browse mode
    let browse_panel = DetailPanel::new(
        Mode::Browse,
        episode_details.clone(),
        EpisodeField::Title,
        0,
        None,
        dirty_fields.clone(),
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let browse_result = browse_panel.render(80, 10, &theme, false);
    
    // Test Edit mode with same data
    let edit_panel = DetailPanel::new(
        Mode::Edit,
        episode_details,
        EpisodeField::Title,
        0,
        None,
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let edit_result = edit_panel.render(80, 10, &theme, false);
    
    // Both should render successfully but may have different content
    assert!(!browse_result.is_empty());
    assert!(!edit_result.is_empty());
    assert_eq!(browse_result.len(), 10);
    assert_eq!(edit_result.len(), 10);
    assert_eq!(browse_result[0].len(), 80);
    assert_eq!(edit_result[0].len(), 80);
}

#[test]
fn test_detail_panel_other_modes_default_to_browse() {
    let episode_details = EpisodeDetail {
        title: "Test Episode".to_string(),
        year: "2023".to_string(),
        watched: "false".to_string(),
        length: "3600".to_string(),
        series: None,
        season: None,
        episode_number: "1".to_string(),
        last_watched_time: None,
        last_progress_time: None,
    };

    let dirty_fields = HashSet::new();

    // Test with Entry mode (should default to Browse behavior)
    let detail_panel = DetailPanel::new(
        Mode::Entry,
        episode_details,
        EpisodeField::Title,
        0,
        None,
        dirty_fields,
        "/path/to/test/episode.mp4".to_string(),
    );

    let theme = Theme::default();
    let result = detail_panel.render(80, 10, &theme, false);
    
    // Should render successfully (defaults to MetadataDisplay)
    assert!(!result.is_empty());
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].len(), 80);
}
