//! Deck theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub stage_background: Color,
    pub sorter_sidebar: Color,
}

impl Theme {
    pub fn presentation_studio() -> Self {
        Self {
            stage_background: Color(24, 26, 32),
            sorter_sidebar: Color(32, 35, 42),
        }
    }
}
