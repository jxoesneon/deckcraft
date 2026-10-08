//! Comprehensive integration test suite for DeckCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! slide navigation, ribbon inputs, transition timing, and the slide canvas.

use deckcraft_engine::{Engine, Tool};
use deckcraft_ui_martensite::{
    DeckcraftApp,
    command_reg::find_command,
    menus::generate_main_menu,
    shortcuts::{SlideNav, nav_key},
    theme::CraftTheme,
    widgets::{DockPanelGroup, RibbonBarWidget, SlideCanvasWidget, SlideItemDef, SlideStripWidget, TransitionTimingWidget},
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = DeckcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.notes_visible);
    assert!(!app.is_presenting);

    // 2. Keystroke Workflow: switch to TextBox, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("t", app.active_tool);
    assert_eq!(new_tool, Some(Tool::TextBox));
    app.set_tool(Tool::TextBox);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores TextBox
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::TextBox));
    app.set_tool(Tool::TextBox);

    // 3. Slide Navigation: build a deck, walk it with nav keys
    app.deck.add_blank_slide();
    app.deck.add_blank_slide();
    assert_eq!(nav_key("PageDown"), Some(SlideNav::Next));
    app.next_slide();
    app.next_slide();
    assert_eq!(app.current_slide, 2);
    assert_eq!(nav_key("Home"), Some(SlideNav::First));
    app.first_slide();
    assert_eq!(app.current_slide, 0);

    // 4. Ribbon Bar Interaction for Active TextBox
    let mut ribbon = RibbonBarWidget::new();
    ribbon.set_tool(Tool::TextBox);
    ribbon.font_size.on_pointer_down(0.0);
    ribbon.font_size.on_pointer_move(6.0, false, false);
    ribbon.font_size.on_pointer_up();
    assert_eq!(ribbon.font_size.value, 24.0); // 18 + 6

    // 5. Slide Strip & Selection Updates
    let mut strip = SlideStripWidget::new();
    for id in 1..=3u64 {
        strip.slides.push(SlideItemDef {
            id,
            title: format!("Slide {id}"),
            hidden: false,
            has_notes: id == 1,
            has_transition: id == 2,
            section: None,
        });
    }
    strip.select_slide(2);
    assert_eq!(strip.selected_slide_id, Some(2));
    strip.toggle_hidden(2);
    assert!(strip.slides[1].hidden);

    // 6. Transition Timing Split Feathering
    let mut timing = TransitionTimingWidget::new();
    timing.advance.set_lo_split(10, 40);
    timing.duration.set_hi_split(70, 90);
    assert!(timing.advance.is_split_lo());
    assert!(timing.duration.is_split_hi());

    // 7. Slide Canvas Zoom at Cursor Anchor (the anchored point stays fixed)
    let mut canvas = SlideCanvasWidget::new(960, 540);
    canvas.zoom_at(1.5, [480.0, 270.0]);
    assert!((canvas.zoom - 1.5).abs() < 1e-4);
    let slide_pt = canvas.screen_to_slide([480.0, 270.0]);
    assert_eq!(slide_pt, [480.0, 270.0]);

    // 8. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Slides", "Notes", "Comments"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 9. Menu Generation Consistency (registry ids resolve against the engine)
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {cmd_id}");
                assert!(deckcraft_engine::find_command(cmd_id).is_some(), "Menu id not in the engine catalog: {cmd_id}");
            }
        }
    }

    // 10. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let studio = CraftTheme::presentation_studio();
    assert_ne!(theme.surface_app_bg, studio.surface_app_bg);
}
