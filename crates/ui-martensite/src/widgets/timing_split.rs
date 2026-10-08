//! Transition timing widget: split handles trimming a slide transition's
//! advance window and duration envelope (percentages of the slide's timeline).

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitRange {
    /// Low edge: where the range starts, split into two feathered handles (0–100 %).
    pub lo_start: u8,
    pub lo_end: u8,
    /// High edge: where the range ends, split into two feathered handles (0–100 %).
    pub hi_start: u8,
    pub hi_end: u8,
}

impl SplitRange {
    pub const fn default_full() -> Self {
        Self { lo_start: 0, lo_end: 0, hi_start: 100, hi_end: 100 }
    }

    pub fn is_split_lo(&self) -> bool {
        self.lo_start != self.lo_end
    }

    pub fn is_split_hi(&self) -> bool {
        self.hi_start != self.hi_end
    }

    pub fn set_lo_split(&mut self, start: u8, end: u8) {
        let s = start.min(end);
        let e = start.max(end);
        self.lo_start = s;
        self.lo_end = e.min(self.hi_start);
    }

    pub fn set_hi_split(&mut self, start: u8, end: u8) {
        let s = start.min(end);
        let e = start.max(end);
        self.hi_start = s.max(self.lo_end);
        self.hi_end = e;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransitionTimingWidget {
    /// "Advance Slide" window: on click vs. automatic, feathered at the edges.
    pub advance: SplitRange,
    /// Duration envelope of the running transition.
    pub duration: SplitRange,
}

impl TransitionTimingWidget {
    pub fn new() -> Self {
        Self { advance: SplitRange::default_full(), duration: SplitRange::default_full() }
    }

    pub fn reset(&mut self) {
        self.advance = SplitRange::default_full();
        self.duration = SplitRange::default_full();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_timing_split_thresholds() {
        let mut widget = TransitionTimingWidget::new();
        assert_eq!(widget.advance.lo_start, 0);
        assert_eq!(widget.advance.hi_end, 100);
        assert!(!widget.advance.is_split_lo());

        // Split the advance window's low edge
        widget.advance.set_lo_split(10, 25);
        assert_eq!(widget.advance.lo_start, 10);
        assert_eq!(widget.advance.lo_end, 25);
        assert!(widget.advance.is_split_lo());

        // Split the duration envelope's high edge
        widget.duration.set_hi_split(80, 95);
        assert_eq!(widget.duration.hi_start, 80);
        assert_eq!(widget.duration.hi_end, 95);
        assert!(widget.duration.is_split_hi());

        widget.reset();
        assert_eq!(widget.advance, SplitRange::default_full());
        assert_eq!(widget.duration, SplitRange::default_full());
    }
}
