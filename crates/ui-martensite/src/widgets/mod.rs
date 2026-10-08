//! Martensite widget suite for DeckCraft.

pub mod dock_panel;
pub mod ribbon_bar;
pub mod scrubby_input;
pub mod slide_canvas;
pub mod slide_strip;
pub mod timing_split;
pub mod tool_strip;

pub use dock_panel::DockPanelGroup;
pub use ribbon_bar::RibbonBarWidget;
pub use scrubby_input::ScrubbyInputWidget;
pub use slide_canvas::SlideCanvasWidget;
pub use slide_strip::{SlideItemDef, SlideStripWidget};
pub use timing_split::TransitionTimingWidget;
pub use tool_strip::ToolStripWidget;
