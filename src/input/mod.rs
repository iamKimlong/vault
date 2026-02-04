//! Input Module
//!
//! Handles keyboard input with vim-style modal editing.

pub mod keymap;
pub mod modes;
pub mod text_buffer;

// Re-exports
pub use modes::InputMode;
pub use text_buffer::{handle_text_key, SecureTextBuffer, TextBuffer, TextEditing};
