//! Presentation integration tests.

use deckcraft_ui_martensite::DeckcraftApp;

#[test]
fn test_presentation_workflow() {
    let mut app = DeckcraftApp::new();
    assert_eq!(app.deck.slides.len(), 1);

    app.deck.add_blank_slide();
    assert_eq!(app.deck.slides.len(), 2);

    app.next_slide();
    assert_eq!(app.current_slide, 1);
}
