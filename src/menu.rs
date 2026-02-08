use crossterm::event::KeyCode;
use crate::dto::EpisodeDetail;
use crate::util::{can_repeat_action, Entry, LastAction, Mode};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub hotkey: Option<KeyCode>,
    pub action: MenuAction,
    pub location: MenuLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuLocation {
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
    Delete,
    SearchOnline,
}

pub struct MenuContext {
    pub selected_entry: Option<Entry>,
    pub episode_detail: EpisodeDetail,
    pub mode: crate::util::Mode,
    pub last_action: Option<LastAction>,
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
            label: "Search Online".to_string(),
            hotkey: Some(KeyCode::F(8)),
            action: MenuAction::SearchOnline,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "rescan".to_string(),
            hotkey: Some(KeyCode::Char('s')),
            action: MenuAction::Rescan,
            location: MenuLocation::ContextMenu,
        },
        MenuItem {
            label: "Delete".to_string(),
            hotkey: None,
            action: MenuAction::Delete,
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
        MenuAction::Delete => {
            // Available only when selected entry is an Episode
            matches!(context.selected_entry, Some(Entry::Episode { .. }))
        }
        MenuAction::SearchOnline => {
            // Available only in Browse mode
            matches!(context.mode, Mode::Browse)
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
