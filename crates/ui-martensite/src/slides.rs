//! Slide deck view-model structures.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AspectRatio {
    Widescreen16_9,
    Standard4_3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Slide {
    pub id: u64,
    pub title: String,
    pub notes: String,
    /// Hidden slides play no part in the show (`slide.hide`).
    pub hidden: bool,
}

pub struct DeckState {
    pub aspect: AspectRatio,
    pub slides: Vec<Slide>,
}

impl DeckState {
    pub fn new_widescreen() -> Self {
        Self { aspect: AspectRatio::Widescreen16_9, slides: vec![Slide { id: 1, title: "Slide 1".to_string(), notes: String::new(), hidden: false }] }
    }

    pub fn add_blank_slide(&mut self) -> u64 {
        let next_id = self.slides.len() as u64 + 1;
        self.slides.push(Slide { id: next_id, title: format!("Slide {}", next_id), notes: String::new(), hidden: false });
        next_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_builder() {
        let mut deck = DeckState::new_widescreen();
        assert_eq!(deck.slides.len(), 1);
        assert_eq!(deck.aspect, AspectRatio::Widescreen16_9);
        let id = deck.add_blank_slide();
        assert_eq!(id, 2);
        assert_eq!(deck.slides.len(), 2);
        assert!(!deck.slides[1].hidden);
    }
}
