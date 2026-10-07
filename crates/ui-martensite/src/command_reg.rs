//! Deck command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "slide.new", label: "New Slide", shortcut: Some("Cmd+M") },
    Command { id: "slide.duplicate", label: "Duplicate Slide", shortcut: Some("Cmd+D") },
    Command { id: "present.start", label: "Start Presentation", shortcut: Some("F5") },
];
