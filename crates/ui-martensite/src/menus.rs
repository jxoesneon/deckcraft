//! Deck menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "File", items: &["file.new", "file.open"] },
    MenuCategory { title: "Slide", items: &["slide.new", "slide.duplicate"] },
    MenuCategory { title: "Slide Show", items: &["present.start"] },
];
