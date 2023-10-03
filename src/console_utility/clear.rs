pub fn clear_console() {
    std::process::Command::new("cls").status().unwrap();
}