use std::io::{self, Write};

/// Clears the terminal.
pub fn clear_terminal() {
    io::stdout().write_all(b"\x1b[2J").unwrap();
}

/// Sets the cursors position to the top-left corner in the terminal.
pub fn reset_cursor_position() {
    io::stdout().write_all(b"\x1b[H").unwrap();
}
