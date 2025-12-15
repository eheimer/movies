use super::{Cell, Component};
use super::metadata_display::MetadataDisplay;
use super::episode_editor::EpisodeEditor;
use crate::dto::EpisodeDetail;
use crate::episode_field::EpisodeField;
use crate::theme::Theme;
use crate::util::Mode;
use std::collections::HashSet;

/// Container component that switches between sub-components based on application mode
pub struct DetailPanel {
    mode: Mode,
    episode_details: EpisodeDetail,
    edit_field: EpisodeField,
    edit_cursor_pos: usize,
    season_number: Option<usize>,
    dirty_fields: HashSet<EpisodeField>,
    entry_location: String,
}

impl DetailPanel {
    /// Create a new DetailPanel component
    pub fn new(
        mode: Mode,
        episode_details: EpisodeDetail,
        edit_field: EpisodeField,
        edit_cursor_pos: usize,
        season_number: Option<usize>,
        dirty_fields: HashSet<EpisodeField>,
        entry_location: String,
    ) -> Self {
        Self {
            mode,
            episode_details,
            edit_field,
            edit_cursor_pos,
            season_number,
            dirty_fields,
            entry_location,
        }
    }
}

impl Component for DetailPanel {
    /// Renders the DetailPanel by delegating to the appropriate sub-component based on mode
    fn render(&self, width: usize, height: usize, theme: &Theme, is_selected: bool) -> Vec<Vec<Cell>> {
        match self.mode {
            Mode::Browse => {
                let metadata_display = MetadataDisplay::new(
                    self.episode_details.clone(),
                    self.season_number,
                    self.entry_location.clone(),
                );
                metadata_display.render(width, height, theme, is_selected)
            }
            Mode::Edit => {
                let episode_editor = EpisodeEditor::new(
                    self.episode_details.clone(),
                    self.edit_field,
                    self.edit_cursor_pos,
                    self.season_number,
                    self.dirty_fields.clone(),
                    self.entry_location.clone(),
                );
                episode_editor.render(width, height, theme, is_selected)
            }
            _ => {
                // For other modes, default to MetadataDisplay
                let metadata_display = MetadataDisplay::new(
                    self.episode_details.clone(),
                    self.season_number,
                    self.entry_location.clone(),
                );
                metadata_display.render(width, height, theme, is_selected)
            }
        }
    }
}

