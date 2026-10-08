//! Decoupled command catalog and taxonomy for DeckCraft.
//!
//! Every `id` here is a real [`deckcraft_engine::cmd::CommandSpec`] id, so the Martensite
//! ribbon/menus dispatch the exact same verbs the egui UI, CLI, control channel and MCP use.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Slide,
    Insert,
    Format,
    Design,
    Transitions,
    SlideShow,
    View,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "New…", category: CommandCategory::File, default_shortcut: Some("Cmd+N"), secondary_shortcut: None },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Cmd+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Cmd+S"), secondary_shortcut: None },
    CommandSpec {
        id: "file.saveAs",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+S"),
        secondary_shortcut: Some("F12"),
    },
    CommandSpec { id: "file.close", label: "Close", category: CommandCategory::File, default_shortcut: Some("Cmd+W"), secondary_shortcut: None },
    CommandSpec { id: "file.export", label: "Export…", category: CommandCategory::File, default_shortcut: None, secondary_shortcut: None },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.redo",
        label: "Redo",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+Y"),
        secondary_shortcut: Some("Shift+Cmd+Z"),
    },
    CommandSpec { id: "edit.cut", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Cmd+X"), secondary_shortcut: None },
    CommandSpec { id: "edit.copy", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Cmd+C"), secondary_shortcut: None },
    CommandSpec { id: "edit.paste", label: "Paste", category: CommandCategory::Edit, default_shortcut: Some("Cmd+V"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.duplicate",
        label: "Duplicate",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.selectAll",
        label: "Select All",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+A"),
        secondary_shortcut: None,
    },
    // Slide
    CommandSpec { id: "slide.new", label: "New Slide", category: CommandCategory::Slide, default_shortcut: Some("Cmd+M"), secondary_shortcut: None },
    CommandSpec {
        id: "slide.duplicate",
        label: "Duplicate Slide",
        category: CommandCategory::Slide,
        default_shortcut: Some("Shift+Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "slide.delete", label: "Delete Slide", category: CommandCategory::Slide, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "slide.layout", label: "Layout", category: CommandCategory::Slide, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "slide.hide", label: "Hide Slide", category: CommandCategory::Slide, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "slide.next",
        label: "Next Slide",
        category: CommandCategory::Slide,
        default_shortcut: Some("PageDown"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "slide.previous",
        label: "Previous Slide",
        category: CommandCategory::Slide,
        default_shortcut: Some("PageUp"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "slide.first",
        label: "First Slide",
        category: CommandCategory::Slide,
        default_shortcut: Some("Home"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "slide.last", label: "Last Slide", category: CommandCategory::Slide, default_shortcut: Some("End"), secondary_shortcut: None },
    // Insert
    CommandSpec { id: "insert.textBox", label: "Text Box", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "shape.insert", label: "Shapes", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.picture", label: "Pictures…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.table", label: "Table", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.audio", label: "Audio…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "insert.video", label: "Video…", category: CommandCategory::Insert, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "insert.hyperlink",
        label: "Link…",
        category: CommandCategory::Insert,
        default_shortcut: Some("Cmd+K"),
        secondary_shortcut: None,
    },
    // Format
    CommandSpec { id: "format.bold", label: "Bold", category: CommandCategory::Format, default_shortcut: Some("Cmd+B"), secondary_shortcut: None },
    CommandSpec {
        id: "format.italic",
        label: "Italic",
        category: CommandCategory::Format,
        default_shortcut: Some("Cmd+I"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "format.font", label: "Font…", category: CommandCategory::Format, default_shortcut: Some("Cmd+T"), secondary_shortcut: None },
    CommandSpec { id: "format.align", label: "Align Text", category: CommandCategory::Format, default_shortcut: None, secondary_shortcut: None },
    // Design
    CommandSpec { id: "design.theme", label: "Themes", category: CommandCategory::Design, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "design.colors", label: "Colors", category: CommandCategory::Design, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "design.fonts", label: "Fonts", category: CommandCategory::Design, default_shortcut: None, secondary_shortcut: None },
    // Transitions
    CommandSpec {
        id: "transition.set",
        label: "Transition",
        category: CommandCategory::Transitions,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "transition.applyAll",
        label: "Apply To All",
        category: CommandCategory::Transitions,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // Slide Show
    CommandSpec {
        id: "show.fromStart",
        label: "From Beginning",
        category: CommandCategory::SlideShow,
        default_shortcut: Some("F5"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "show.fromCurrent",
        label: "From Current Slide",
        category: CommandCategory::SlideShow,
        default_shortcut: Some("Shift+F5"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "show.setup", label: "Set Up Show…", category: CommandCategory::SlideShow, default_shortcut: None, secondary_shortcut: None },
    // View
    CommandSpec { id: "view.slideMaster", label: "Slide Master", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "slide.notes", label: "Notes", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "window.next",
        label: "Next Window",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+F6"),
        secondary_shortcut: None,
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_registry_ids_exist_in_engine_catalog() {
        for cmd in COMMAND_REGISTRY {
            assert!(deckcraft_engine::find_command(cmd.id).is_some(), "registry id not in the engine command catalog: {}", cmd.id);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Slide).is_empty());
        assert!(!commands_by_category(CommandCategory::Insert).is_empty());
        assert!(!commands_by_category(CommandCategory::SlideShow).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}
