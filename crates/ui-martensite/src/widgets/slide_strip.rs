//! Slide strip widget: the thumbnail rail on the left (and the Slide Sorter grid).

#[derive(Clone, Debug, PartialEq)]
pub struct SlideItemDef {
    pub id: u64,
    pub title: String,
    pub hidden: bool,
    pub has_notes: bool,
    pub has_transition: bool,
    pub section: Option<String>,
}

pub struct SlideStripWidget {
    pub slides: Vec<SlideItemDef>,
    pub selected_slide_id: Option<u64>,
    /// Thumbnail scale factor for the strip (Slide Sorter zooms it too).
    pub thumbnail_zoom: f32,
    /// Transition badge rendered under each thumbnail ("star" indicator text).
    pub transition_badges: bool,
}

impl SlideStripWidget {
    pub fn new() -> Self {
        Self { slides: Vec::new(), selected_slide_id: None, thumbnail_zoom: 1.0, transition_badges: true }
    }

    pub fn select_slide(&mut self, id: u64) {
        if self.slides.iter().any(|s| s.id == id) {
            self.selected_slide_id = Some(id);
        }
    }

    /// `slide.hide`: hidden slides dim in the strip and skip in the show.
    pub fn toggle_hidden(&mut self, id: u64) {
        if let Some(item) = self.slides.iter_mut().find(|s| s.id == id) {
            item.hidden = !item.hidden;
        }
    }

    /// `slide.move`: drag a thumbnail to a new position (clamped); returns where it went.
    pub fn move_slide(&mut self, from: usize, to: usize) -> Option<usize> {
        if from >= self.slides.len() {
            return None;
        }
        let slide = self.slides.remove(from);
        let to = to.min(self.slides.len());
        self.slides.insert(to, slide);
        Some(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: u64) -> SlideItemDef {
        SlideItemDef { id, title: format!("Slide {id}"), hidden: false, has_notes: false, has_transition: false, section: None }
    }

    #[test]
    fn test_slide_strip_mutation() {
        let mut strip = SlideStripWidget::new();
        strip.slides.push(item(1));
        strip.slides.push(item(2));
        strip.slides.push(item(3));

        strip.select_slide(2);
        assert_eq!(strip.selected_slide_id, Some(2));
        strip.select_slide(99);
        assert_eq!(strip.selected_slide_id, Some(2)); // Unknown id keeps selection

        strip.toggle_hidden(2);
        assert!(strip.slides[1].hidden);
        strip.toggle_hidden(2);
        assert!(!strip.slides[1].hidden);

        assert_eq!(strip.move_slide(0, 2), Some(2));
        assert_eq!(strip.slides.iter().map(|s| s.id).collect::<Vec<_>>(), vec![2, 3, 1]);
        assert_eq!(strip.move_slide(9, 0), None);
    }
}
