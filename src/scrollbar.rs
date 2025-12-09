/// Scroll bar state and rendering for terminal UI lists
///
/// This module provides reusable scroll bar functionality for any scrollable list
/// in the terminal interface. It calculates scroll bar position and dimensions
/// based on list size and viewport, then renders the scroll bar in a specified column.

/// Represents the calculated state of a scroll bar
///
/// Contains all information needed to render a scroll bar, including visibility,
/// position, and dimensions. Use `ScrollBarState::hidden()` to create a non-visible state.
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollBarState {
    /// Whether the scroll bar should be rendered
    pub visible: bool,
    /// Starting row of the scroll bar track
    pub track_start: usize,
    /// Height of the scroll bar track in rows
    pub track_height: usize,
    /// Starting row of the scroll indicator
    pub indicator_start: usize,
    /// Height of the scroll indicator in rows (minimum 1)
    pub indicator_height: usize,
    /// Column position for rendering the scroll bar
    pub column: usize,
}

impl ScrollBarState {
    /// Creates a hidden scroll bar state
    ///
    /// Use this when the scroll bar should not be displayed (e.g., when all items
    /// fit on screen or when there are no items to display).
    pub fn hidden() -> Self {
        ScrollBarState {
            visible: false,
            track_start: 0,
            track_height: 0,
            indicator_start: 0,
            indicator_height: 0,
            column: 0,
        }
    }
}

/// Calculates scroll bar state based on list parameters
///
/// This function determines whether a scroll bar should be visible and calculates
/// its position and dimensions based on the total number of items, visible items,
/// and current scroll position.
///
/// # Arguments
///
/// * `total_items` - Total number of items in the list
/// * `visible_items` - Number of items that fit in the visible area
/// * `first_visible_index` - Index of the first visible item (0-based)
/// * `start_row` - Starting row for the scroll bar track
/// * `available_height` - Height available for the scroll bar track
/// * `column` - Column position for rendering the scroll bar
///
/// # Returns
///
/// A `ScrollBarState` with calculated positions and dimensions. If the scroll bar
/// should not be visible (total_items <= visible_items), returns a hidden state.
///
/// # Examples
///
/// ```
/// use movies::scrollbar::calculate_scrollbar_state;
///
/// // List with 100 items, showing 20 at a time, starting at item 0
/// let state = calculate_scrollbar_state(100, 20, 0, 5, 20, 80);
/// assert!(state.visible);
/// assert_eq!(state.indicator_start, 5); // At top
///
/// // List with 10 items, showing 20 at a time - no scroll bar needed
/// let state = calculate_scrollbar_state(10, 20, 0, 5, 20, 80);
/// assert!(!state.visible);
/// ```
pub fn calculate_scrollbar_state(
    total_items: usize,
    visible_items: usize,
    first_visible_index: usize,
    start_row: usize,
    available_height: usize,
    column: usize,
) -> ScrollBarState {
    // If all items fit on screen, no scroll bar needed
    if total_items <= visible_items || total_items == 0 || available_height == 0 {
        return ScrollBarState::hidden();
    }

    // Calculate indicator height proportional to visible/total ratio
    // Minimum height is 1 to ensure indicator is always visible
    let indicator_height = std::cmp::max(
        1,
        (visible_items * available_height) / total_items
    );

    // Calculate the scrollable range for the indicator
    let indicator_travel_range = available_height.saturating_sub(indicator_height);
    
    // Calculate indicator position proportionally based on scroll position
    // The indicator should move through the travel range as we scroll through the list
    let scrollable_items = total_items.saturating_sub(visible_items);
    
    let indicator_offset = if scrollable_items > 0 {
        (first_visible_index * indicator_travel_range) / scrollable_items
    } else {
        0
    };

    let indicator_start = start_row + indicator_offset;
    
    // Ensure indicator stays within track bounds
    let track_end = start_row + available_height;
    let indicator_end = indicator_start + indicator_height;
    let clamped_indicator_start = if indicator_end > track_end {
        // If indicator would extend past track end, move it back
        track_end.saturating_sub(indicator_height)
    } else {
        indicator_start
    };

    ScrollBarState {
        visible: true,
        track_start: start_row,
        track_height: available_height,
        indicator_start: clamped_indicator_start,
        indicator_height,
        column,
    }
}

/// Renders a scroll bar in the terminal at the calculated position
///
/// This function draws the scroll bar track and indicator using the characters
/// and colors specified in the theme. If the scroll bar is not visible,
/// the function returns immediately without rendering anything.
///
/// # Arguments
///
/// * `state` - The calculated scroll bar state containing position and dimensions
/// * `theme` - Theme containing scroll bar characters and colors
///
/// # Returns
///
/// * `io::Result<()>` - Ok if rendering succeeds, Err if IO operations fail
///
/// # Examples
///
/// ```no_run
/// use movies::scrollbar::{calculate_scrollbar_state, render_scrollbar};
/// use movies::theme::Theme;
///
/// let theme = Theme::default();
/// let state = calculate_scrollbar_state(100, 20, 0, 5, 20, 80);
/// render_scrollbar(&state, &theme).expect("Failed to render scroll bar");
/// ```
pub fn render_scrollbar(
    state: &ScrollBarState,
    theme: &crate::theme::Theme,
) -> std::io::Result<()> {
    use crate::terminal::print_at;
    use crate::display::{string_to_fg_color_or_default, string_to_bg_color_or_default};
    use crossterm::style::Stylize;

    // If scroll bar is not visible, return early
    if !state.visible {
        return Ok(());
    }

    // Get colors from theme
    let fg_color = string_to_fg_color_or_default(&theme.scrollbar_fg);
    let bg_color = string_to_bg_color_or_default(&theme.scrollbar_bg);

    // Draw the track characters for the full track height
    for row in 0..state.track_height {
        let absolute_row = state.track_start + row;
        let styled_track = theme.scrollbar_track_char
            .as_str()
            .with(fg_color)
            .on(bg_color);
        print_at(state.column, absolute_row, &styled_track)?;
    }

    // Draw the indicator characters at the calculated position
    // Ensure indicator stays within track bounds
    let track_end = state.track_start + state.track_height;
    for row in 0..state.indicator_height {
        let absolute_row = state.indicator_start + row;
        // Only draw if within track bounds
        if absolute_row >= state.track_start && absolute_row < track_end {
            let styled_indicator = theme.scrollbar_indicator_char
                .as_str()
                .with(fg_color)
                .on(bg_color);
            print_at(state.column, absolute_row, &styled_indicator)?;
        }
    }

    Ok(())
}
