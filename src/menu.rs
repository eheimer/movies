use crossterm::event::KeyCode;
use crate::dto::EpisodeDetail;
use crate::util::{can_repeat_action, Entry, LastAction, ViewContext};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub hotkey: Option<KeyCode>,
    pub action: MenuAction,
    pub location: MenuLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuLocation {
    FirstLine,           // Always visible in first header line
    FirstLinePreferred,  // Prefer first line, overflow to context menu if needed
    ContextMenu,         // Only visible in F1 context menu
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    Edit,
    ToggleWatched,
    AssignToSeries,
    RepeatAction,
    Rescan,
    ClearSeriesData,
    UnwatchAll,
}

pub struct MenuContext {
    pub selected_entry: Option<Entry>,
    pub episode_detail: EpisodeDetail,
    pub last_action: Option<LastAction>,
    pub view_context: ViewContext,
}

/// Define all menu items with their properties
fn define_all_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            label: "edit".to_string(),
            hotkey: Some(KeyCode::F(2)),
            action: MenuAction::Edit,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "toggle watched".to_string(),
            hotkey: Some(KeyCode::F(3)),
            action: MenuAction::ToggleWatched,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "assign to series".to_string(),
            hotkey: Some(KeyCode::F(4)),
            action: MenuAction::AssignToSeries,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "Repeat action".to_string(),
            hotkey: Some(KeyCode::F(5)),
            action: MenuAction::RepeatAction,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "Clear Series Data".to_string(),
            hotkey: Some(KeyCode::F(6)),
            action: MenuAction::ClearSeriesData,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "Unwatch All".to_string(),
            hotkey: Some(KeyCode::F(7)),
            action: MenuAction::UnwatchAll,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "rescan".to_string(),
            hotkey: Some(KeyCode::Char('s')),
            action: MenuAction::Rescan,
            location: MenuLocation::ContextMenu,
        },
    ]
}

/// Check if a menu item should be available based on context
fn is_item_available(item: &MenuItem, context: &MenuContext) -> bool {
    match &item.action {
        MenuAction::Edit => {
            // Available only when selected entry is an Episode
            matches!(context.selected_entry, Some(Entry::Episode { .. }))
        }
        MenuAction::ToggleWatched => {
            // Available only when selected entry is an Episode
            matches!(context.selected_entry, Some(Entry::Episode { .. }))
        }
        MenuAction::AssignToSeries => {
            // Available only when selected entry is an Episode without a series
            matches!(context.selected_entry, Some(Entry::Episode { .. }))
                && context.episode_detail.series.is_none()
        }
        MenuAction::RepeatAction => {
            // Available only when can_repeat_action returns true
            if let Some(ref entry) = context.selected_entry {
                can_repeat_action(&context.last_action, entry, &context.episode_detail)
            } else {
                false
            }
        }
        MenuAction::Rescan => {
            // Always available
            true
        }
        MenuAction::ClearSeriesData => {
            // Available only when selected entry is an Episode with series data
            if let Some(Entry::Episode { .. }) = context.selected_entry {
                // Check if any series-related field is populated
                context.episode_detail.series.is_some()
                    || context.episode_detail.season.is_some()
                    || (!context.episode_detail.episode_number.is_empty()
                        && context.episode_detail.episode_number != "0")
            } else {
                false
            }
        }
        MenuAction::UnwatchAll => {
            // Available in all contexts
            true
        }
    }
}

/// Get all menu items available for the current context
pub fn get_available_menu_items(context: &MenuContext) -> Vec<MenuItem> {
    define_all_menu_items()
        .into_iter()
        .filter(|item| is_item_available(item, context))
        .collect()
}

/// Get only first-line menu items
pub fn get_first_line_items(context: &MenuContext) -> Vec<MenuItem> {
    get_available_menu_items(context)
        .into_iter()
        .filter(|item| item.location == MenuLocation::FirstLine)
        .collect()
}

/// Get only context menu items
pub fn get_context_menu_items(context: &MenuContext) -> Vec<MenuItem> {
    get_available_menu_items(context)
        .into_iter()
        .filter(|item| item.location == MenuLocation::ContextMenu)
        .collect()
}

/// Get FirstLinePreferred items in priority order (filtered by availability)
pub fn get_first_line_preferred_items(context: &MenuContext) -> Vec<MenuItem> {
    get_available_menu_items(context)
        .into_iter()
        .filter(|item| item.location == MenuLocation::FirstLinePreferred)
        .collect()
}

/// Calculate the display width for a menu item
/// Format: "[hotkey] label, "
/// Example: "[S] rescan, " = 12 chars
pub fn calculate_menu_helper_width(item: &MenuItem) -> usize {
    let mut width = 0;
    
    // Opening bracket
    width += 1;
    
    // Hotkey text
    if let Some(hotkey) = &item.hotkey {
        width += match hotkey {
            KeyCode::F(n) => {
                // F-keys: "F" + number (e.g., "F2" = 2 chars, "F10" = 3 chars)
                if *n < 10 {
                    2
                } else {
                    3
                }
            }
            KeyCode::Char(_) => {
                // Single character (uppercase)
                1
            }
            _ => {
                // Other key types (shouldn't happen in practice)
                1
            }
        };
    }
    
    // Closing bracket
    width += 1;
    
    // Space after bracket
    width += 1;
    
    // Label text
    width += item.label.len();
    
    // Separator ", "
    width += 2;
    
    width
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{EpisodeDetail, Season, Series};
    use crate::util::{Entry, ViewContext};

    #[test]
    fn test_first_line_preferred_respects_availability() {
        // Create a context with an episode that has no series
        let episode_detail = EpisodeDetail {
            title: "Test Episode".to_string(),
            year: "2023".to_string(),
            watched: "0".to_string(),
            length: "45".to_string(),
            series: None,
            season: None,
            episode_number: String::new(),
        };

        let context = MenuContext {
            selected_entry: Some(Entry::Episode {
                episode_id: 1,
                name: "Test".to_string(),
                location: "/test".to_string(),
            }),
            episode_detail: episode_detail.clone(),
            last_action: None,
            view_context: ViewContext::TopLevel,
        };

        // Get first line preferred items
        let items = get_first_line_preferred_items(&context);

        // Currently there are no FirstLinePreferred items defined,
        // so this should return an empty vector
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_context_aware_filtering_episode_without_series() {
        let episode_detail = EpisodeDetail {
            title: "Test Episode".to_string(),
            year: "2023".to_string(),
            watched: "0".to_string(),
            length: "45".to_string(),
            series: None,
            season: None,
            episode_number: String::new(),
        };

        let context = MenuContext {
            selected_entry: Some(Entry::Episode {
                episode_id: 1,
                name: "Test".to_string(),
                location: "/test".to_string(),
            }),
            episode_detail: episode_detail.clone(),
            last_action: None,
            view_context: ViewContext::TopLevel,
        };

        let available_items = get_available_menu_items(&context);

        // Should include: edit, toggle watched, assign to series, rescan, unwatch all
        // Should NOT include: repeat action (no last action), clear series data (no series)
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::Edit)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::ToggleWatched)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::AssignToSeries)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::Rescan)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::UnwatchAll)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::RepeatAction)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::ClearSeriesData)));
    }

    #[test]
    fn test_context_aware_filtering_episode_with_series() {
        let episode_detail = EpisodeDetail {
            title: "Test Episode".to_string(),
            year: "2023".to_string(),
            watched: "0".to_string(),
            length: "45".to_string(),
            series: Some(Series {
                id: 1,
                name: "Test Series".to_string(),
            }),
            season: Some(Season {
                id: 1,
                number: 1,
            }),
            episode_number: "1".to_string(),
        };

        let context = MenuContext {
            selected_entry: Some(Entry::Episode {
                episode_id: 1,
                name: "Test".to_string(),
                location: "/test".to_string(),
            }),
            episode_detail: episode_detail.clone(),
            last_action: None,
            view_context: ViewContext::TopLevel,
        };

        let available_items = get_available_menu_items(&context);

        // Should include: edit, toggle watched, clear series data, rescan, unwatch all
        // Should NOT include: assign to series (already has series), repeat action (no last action)
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::Edit)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::ToggleWatched)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::ClearSeriesData)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::Rescan)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::UnwatchAll)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::AssignToSeries)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::RepeatAction)));
    }

    #[test]
    fn test_context_aware_filtering_series_selected() {
        let episode_detail = EpisodeDetail {
            title: String::new(),
            year: String::new(),
            watched: String::new(),
            length: String::new(),
            series: None,
            season: None,
            episode_number: String::new(),
        };

        let context = MenuContext {
            selected_entry: Some(Entry::Series {
                series_id: 1,
                name: "Test Series".to_string(),
            }),
            episode_detail: episode_detail.clone(),
            last_action: None,
            view_context: ViewContext::TopLevel,
        };

        let available_items = get_available_menu_items(&context);

        // Should only include: rescan, unwatch all (always available)
        // Should NOT include: edit, toggle watched, assign to series, clear series data (episode-only)
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::Rescan)));
        assert!(available_items.iter().any(|i| matches!(i.action, MenuAction::UnwatchAll)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::Edit)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::ToggleWatched)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::AssignToSeries)));
        assert!(!available_items.iter().any(|i| matches!(i.action, MenuAction::ClearSeriesData)));
    }

    #[test]
    fn test_calculate_menu_helper_width() {
        // Test with F-key hotkey
        let item = MenuItem {
            label: "edit".to_string(),
            hotkey: Some(KeyCode::F(2)),
            action: MenuAction::Edit,
            location: MenuLocation::ContextMenu,
        };
        // "[F2] edit, " = 1 + 2 + 1 + 1 + 4 + 2 = 11
        assert_eq!(calculate_menu_helper_width(&item), 11);

        // Test with char hotkey
        let item = MenuItem {
            label: "rescan".to_string(),
            hotkey: Some(KeyCode::Char('s')),
            action: MenuAction::Rescan,
            location: MenuLocation::ContextMenu,
        };
        // "[S] rescan, " = 1 + 1 + 1 + 1 + 6 + 2 = 12
        assert_eq!(calculate_menu_helper_width(&item), 12);

        // Test with longer label
        let item = MenuItem {
            label: "toggle watched".to_string(),
            hotkey: Some(KeyCode::F(3)),
            action: MenuAction::ToggleWatched,
            location: MenuLocation::ContextMenu,
        };
        // "[F3] toggle watched, " = 1 + 2 + 1 + 1 + 14 + 2 = 21
        assert_eq!(calculate_menu_helper_width(&item), 21);
    }
}
