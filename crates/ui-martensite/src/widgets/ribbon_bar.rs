//! Ribbon bar widget adapting dynamically to the active tool and selection.

use deckcraft_engine::Tool;

use crate::widgets::scrubby_input::ScrubbyInputWidget;

pub struct RibbonBarWidget {
    pub active_tool: Tool,
    /// Font group: point size of the selected text (PowerPoint: 1–4000 pt).
    pub font_size: ScrubbyInputWidget,
    /// Paragraph group: line spacing multiplier.
    pub line_spacing: ScrubbyInputWidget,
    /// Paragraph group: list indent level.
    pub indent_level: ScrubbyInputWidget,
    /// Shape Format group: shape rotation in degrees.
    pub rotation: ScrubbyInputWidget,
    /// View group toggles.
    pub gridlines: bool,
    pub guides: bool,
}

impl RibbonBarWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            font_size: ScrubbyInputWidget::new("Size", 18.0, 1.0, 4000.0, "pt"),
            line_spacing: ScrubbyInputWidget::new("Spacing", 1.0, 0.5, 4.0, "×"),
            indent_level: ScrubbyInputWidget::new("Indent", 0.0, 0.0, 9.0, "lvl"),
            rotation: ScrubbyInputWidget::new("Rotation", 0.0, -360.0, 360.0, "°"),
            gridlines: false,
            guides: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn toggle_gridlines(&mut self) -> bool {
        self.gridlines = !self.gridlines;
        self.gridlines
    }

    pub fn toggle_guides(&mut self) -> bool {
        self.guides = !self.guides;
        self.guides
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ribbon_bar_defaults() {
        let mut bar = RibbonBarWidget::new();
        assert_eq!(bar.active_tool, Tool::Select);
        assert_eq!(bar.font_size.value, 18.0);
        assert_eq!(bar.line_spacing.value, 1.0);
        assert!(!bar.gridlines);

        bar.set_tool(Tool::TextBox);
        assert_eq!(bar.active_tool, Tool::TextBox);

        assert!(bar.toggle_gridlines());
        assert!(bar.gridlines);
        assert!(bar.toggle_guides());
        assert!(bar.guides);
        assert!(!bar.toggle_guides());
    }
}
