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
    FirstLine,    // Always visible in first header line
    ContextMenu,  // Only visible in F1 context menu
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    Edit,
    ToggleWatched,
    AssignToSeries,
    RepeatAction,
    Rescan,
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
            label: "rescan".to_string(),
            hotkey: Some(KeyCode::Char('l')),  // CTRL+L handled separately
            action: MenuAction::Rescan,
            location: MenuLocation::FirstLine,
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
