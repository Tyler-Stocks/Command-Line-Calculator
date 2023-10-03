use clearscreen::ClearScreen;

pub fn clear_console() {
    ClearScreen::default()
                .clear()
                .expect("Failed to clear the terminal.")
}
