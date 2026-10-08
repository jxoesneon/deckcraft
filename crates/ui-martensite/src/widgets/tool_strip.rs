//! Tool strip widget: single/double column layout, tool flyouts, and fill/line chips.

use deckcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Select, alternatives: &[] },
    ToolSlot { primary: Tool::TextBox, alternatives: &[] },
    ToolSlot { primary: Tool::Shape, alternatives: &[] },
    ToolSlot { primary: Tool::Image, alternatives: &[] },
    ToolSlot { primary: Tool::Table, alternatives: &[] },
    ToolSlot { primary: Tool::Ink, alternatives: &[Tool::Eraser] },
    ToolSlot { primary: Tool::Notes, alternatives: &[] },
    ToolSlot { primary: Tool::Navigator, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
    ToolSlot { primary: Tool::Present, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub double_column: bool,
    /// Shape fill chip (like a colour well; default deck accent).
    pub fill_color: [u8; 4],
    /// Shape outline chip.
    pub line_color: [u8; 4],
    /// Ink overlay during a running show (pen annotations over the slides).
    pub ink_overlay: bool,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::Select,
            double_column: false,
            fill_color: [68, 114, 196, 255],  // Default accent fill
            line_color: [255, 255, 255, 255], // Default white outline
            ink_overlay: false,
        }
    }

    pub fn toggle_column_mode(&mut self) -> bool {
        self.double_column = !self.double_column;
        self.double_column
    }

    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.fill_color, &mut self.line_color);
    }

    pub fn reset_default_colors(&mut self) {
        self.fill_color = [68, 114, 196, 255];
        self.line_color = [255, 255, 255, 255];
    }

    pub fn toggle_ink_overlay(&mut self) -> bool {
        self.ink_overlay = !self.ink_overlay;
        self.ink_overlay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Select);
        assert!(!strip.double_column);
        assert_eq!(strip.fill_color, [68, 114, 196, 255]);
        assert_eq!(strip.line_color, [255, 255, 255, 255]);

        strip.swap_colors();
        assert_eq!(strip.fill_color, [255, 255, 255, 255]);
        assert_eq!(strip.line_color, [68, 114, 196, 255]);

        strip.reset_default_colors();
        assert_eq!(strip.fill_color, [68, 114, 196, 255]);

        assert!(strip.toggle_column_mode());
        assert!(strip.double_column);
        assert!(!strip.toggle_column_mode());

        assert!(strip.toggle_ink_overlay());
        assert!(strip.ink_overlay);
    }

    #[test]
    fn test_every_tool_has_a_slot() {
        for tool in [
            Tool::Select,
            Tool::TextBox,
            Tool::Shape,
            Tool::Image,
            Tool::Table,
            Tool::Ink,
            Tool::Eraser,
            Tool::Notes,
            Tool::Navigator,
            Tool::Hand,
            Tool::Zoom,
            Tool::Present,
        ] {
            let covered = TOOL_SLOTS.iter().any(|s| s.primary == tool || s.alternatives.contains(&tool));
            assert!(covered, "no tool strip slot for {tool:?}");
        }
    }
}
