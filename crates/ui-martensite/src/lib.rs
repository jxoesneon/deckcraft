//! Sovereign retained-mode interface for DeckCraft built on the Martensite GUI engine.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod command_reg;
pub mod menus;
pub mod shortcuts;
pub mod slides;
pub mod theme;
pub mod widgets;

use std::sync::{Arc, Mutex};

use deckcraft_engine::Engine;

/// Application state container managing the Martensite GUI pipeline.
pub struct DeckcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: deckcraft_engine::Tool,
    /// View-model backing the slide sorter strip; synced from the engine's active
    /// presentation by [`Self::sync_deck`].
    pub deck: slides::DeckState,
    /// Slide the editor is focused on (slide-nav state).
    pub current_slide: usize,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    /// Speaker-notes pane below the slide canvas.
    pub notes_visible: bool,
    pub is_presenting: bool,
    pub is_dirty: bool,
}

impl DeckcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::dark_neutral(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: deckcraft_engine::Tool::Select,
            deck: slides::DeckState::new_widescreen(),
            current_slide: 0,
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            notes_visible: true,
            is_presenting: false,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: deckcraft_engine::Tool) {
        self.active_tool = tool;
    }

    /// PowerPoint zooms between 10 % and 400 %.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.1, 4.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    /// Rebuild the slide-strip view-model from the engine's active presentation.
    /// Returns false when there is no open presentation.
    pub fn sync_deck(&mut self) -> bool {
        let engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(state) = engine.active() else { return false };
        self.deck.slides = state
            .doc
            .slides
            .iter()
            .enumerate()
            .map(|(i, s)| slides::Slide {
                id: i as u64 + 1,
                title: if s.name.is_empty() { format!("Slide {}", i + 1) } else { s.name.clone() },
                notes: String::new(),
                hidden: s.hidden,
            })
            .collect();
        if self.current_slide >= self.deck.slides.len() {
            self.current_slide = self.deck.slides.len().saturating_sub(1);
        }
        self.is_dirty = !Arc::ptr_eq(&state.doc, &state.saved_doc);
        true
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

    pub fn first_slide(&mut self) {
        self.current_slide = 0;
    }

    pub fn last_slide(&mut self) {
        self.current_slide = self.deck.slides.len().saturating_sub(1);
    }

    /// Focus slide `index` in the editor; false when out of range.
    pub fn go_to_slide(&mut self, index: usize) -> bool {
        if index < self.deck.slides.len() {
            self.current_slide = index;
            true
        } else {
            false
        }
    }

    pub fn toggle_notes(&mut self) -> bool {
        self.notes_visible = !self.notes_visible;
        self.notes_visible
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
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = DeckcraftApp::new(engine);
        assert_eq!(app.active_tool, deckcraft_engine::Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.notes_visible);
        assert!(!app.is_presenting);
        assert!(!app.is_dirty);
        assert_eq!(app.deck.slides.len(), 1);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = DeckcraftApp::new(engine);

        app.set_zoom(2.0);
        assert_eq!(app.zoom_level, 2.0);

        app.set_zoom(0.01);
        assert_eq!(app.zoom_level, 0.1);

        app.set_zoom(100.0);
        assert_eq!(app.zoom_level, 4.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = DeckcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_deck_navigation() {
        let engine = Engine::new();
        let mut app = DeckcraftApp::new(engine);
        app.deck.add_blank_slide();
        app.deck.add_blank_slide();
        assert_eq!(app.deck.slides.len(), 3);

        app.next_slide();
        assert_eq!(app.current_slide, 1);
        app.last_slide();
        assert_eq!(app.current_slide, 2);
        app.next_slide();
        assert_eq!(app.current_slide, 2);
        app.prev_slide();
        assert_eq!(app.current_slide, 1);
        app.first_slide();
        assert_eq!(app.current_slide, 0);

        assert!(app.go_to_slide(1));
        assert_eq!(app.current_slide, 1);
        assert!(!app.go_to_slide(3));
    }

    #[test]
    fn test_toggles() {
        let engine = Engine::new();
        let mut app = DeckcraftApp::new(engine);

        assert!(app.notes_visible);
        assert!(!app.toggle_notes());
        assert!(!app.notes_visible);
        assert!(app.toggle_notes());

        assert!(!app.is_presenting);
        assert!(app.toggle_presenter_mode());
        assert!(app.is_presenting);
        assert!(!app.toggle_presenter_mode());
    }

    #[test]
    fn test_sync_deck_without_presentation() {
        let engine = Engine::new();
        let mut app = DeckcraftApp::new(engine);
        // No presentation open: nothing to sync, deck view-model untouched.
        assert!(!app.sync_deck());
        assert_eq!(app.deck.slides.len(), 1);
    }
}
