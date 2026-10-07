//! Sovereign retained-mode presentation UI for DeckCraft built on Martensite.

pub mod command_reg;
pub mod menus;
pub mod slides;
pub mod theme;

pub struct DeckcraftApp {
    pub deck: slides::DeckState,
    pub current_slide: usize,
    pub is_presenting: bool,
}

impl DeckcraftApp {
    pub fn new() -> Self {
        Self {
            deck: slides::DeckState::new_widescreen(),
            current_slide: 0,
            is_presenting: false,
        }
    }

    pub fn next_slide(&mut self) {
        if self.current_slide + 1 < self.deck.slides.len() {
            self.current_slide += 1;
        }
    }

    pub fn prev_slide(&mut self) {
        if self.current_slide > 0 {
            self.current_slide -= 1;
        }
    }

    pub fn toggle_presenter_mode(&mut self) -> bool {
        self.is_presenting = !self.is_presenting;
        self.is_presenting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_navigation() {
        let mut app = DeckcraftApp::new();
        app.deck.add_blank_slide();
        assert_eq!(app.deck.slides.len(), 2);

        app.next_slide();
        assert_eq!(app.current_slide, 1);

        app.prev_slide();
        assert_eq!(app.current_slide, 0);

        assert!(app.toggle_presenter_mode());
        assert!(app.is_presenting);
    }
}
