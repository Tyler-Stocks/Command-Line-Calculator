use clearscreen::ClearScreen;

/// Wrapper around
///
/// clearscreen::ClearScreen::default.clear().expect("Failed to clear the terminal")
pub fn clear_console() {
    ClearScreen::default()
                .clear()
                .expect("Failed to clear the terminal.")
}
