//! Keystroke state machine providing 100% PowerPoint keyboard ergonomics.

use deckcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

/// Slide navigation driven by keyboard focus (PowerPoint: PageUp/PageDown/Home/End
/// walk the slide strip and the running show).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlideNav {
    Next,
    Previous,
    First,
    Last,
}

pub fn nav_key(key: &str) -> Option<SlideNav> {
    match key {
        "PageDown" => Some(SlideNav::Next),
        "PageUp" => Some(SlideNav::Previous),
        "Home" => Some(SlideNav::First),
        "End" => Some(SlideNav::Last),
        _ => None,
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Single-key tool switches when no text field owns the keystroke.
            "v" | "V" | "Escape" => Some(Tool::Select),
            "t" | "T" => Some(Tool::TextBox),
            "s" | "S" => Some(Tool::Shape),
            "i" | "I" => Some(Tool::Image),
            "g" | "G" => Some(Tool::Table),
            "p" | "P" => Some(Tool::Ink),
            "e" | "E" => Some(Tool::Eraser),
            "n" | "N" => Some(Tool::Notes),
            "h" | "H" => Some(Tool::Hand),
            "F5" => Some(Tool::Present),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Ink), Some(Tool::Select));
        assert_eq!(k.on_key_down("T", Tool::Select), Some(Tool::TextBox));
        assert_eq!(k.on_key_down("s", Tool::TextBox), Some(Tool::Shape));
        assert_eq!(k.on_key_down("i", Tool::Shape), Some(Tool::Image));
        assert_eq!(k.on_key_down("g", Tool::Image), Some(Tool::Table));
        assert_eq!(k.on_key_down("p", Tool::Table), Some(Tool::Ink));
        assert_eq!(k.on_key_down("e", Tool::Ink), Some(Tool::Eraser));
        assert_eq!(k.on_key_down("n", Tool::Eraser), Some(Tool::Notes));
        assert_eq!(k.on_key_down("h", Tool::Notes), Some(Tool::Hand));
        assert_eq!(k.on_key_down("F5", Tool::Hand), Some(Tool::Present));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::TextBox;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Shape;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }

    #[test]
    fn test_slide_nav_keys() {
        assert_eq!(nav_key("PageDown"), Some(SlideNav::Next));
        assert_eq!(nav_key("PageUp"), Some(SlideNav::Previous));
        assert_eq!(nav_key("Home"), Some(SlideNav::First));
        assert_eq!(nav_key("End"), Some(SlideNav::Last));
        assert_eq!(nav_key("x"), None);
    }
}
