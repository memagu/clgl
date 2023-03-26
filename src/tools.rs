/// Clears the terminal.
pub fn clear_terminal() {
    print!("\x1b[2J");
}

/// Sets the cursors position to the top-left corner in the termnal.
pub fn reset_cursor_position() {
    print!("\x1b[H");
}