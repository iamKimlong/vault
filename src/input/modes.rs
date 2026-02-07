//! Input Modes
//!
//! Modal editing state machine for vim-style interface.

use super::{TextBuffer, TextEditing};

/// Input mode enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Search,
    Confirm,
    Help,
    Logs,
    Tags,
    Export,
}

impl InputMode {
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
            Self::Command => "COMMAND",
            Self::Search => "SEARCH",
            Self::Confirm => "CONFIRM",
            Self::Help => "HELP",
            Self::Logs => "LOG",
            Self::Tags => "TAG",
            Self::Export => "EXPORT",
        }
    }

    pub fn is_text_input(&self) -> bool {
        matches!(self, Self::Command | Self::Search)
    }
}

/// Mode state with associated data
#[derive(Debug, Clone)]
pub struct ModeState {
    pub mode: InputMode,
    pub buffer: TextBuffer,
    pub pending: Option<char>,
}

impl Default for ModeState {
    fn default() -> Self {
        Self {
            mode: InputMode::Normal,
            buffer: TextBuffer::new(),
            pending: None,
        }
    }
}

impl ModeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
        self.buffer.clear();
        self.pending = None;
    }

    pub fn to_normal(&mut self) {
        self.set_mode(InputMode::Normal);
    }

    pub fn to_insert(&mut self) {
        self.mode = InputMode::Insert;
    }

    pub fn to_command(&mut self) {
        self.set_mode(InputMode::Command);
    }

    pub fn to_search(&mut self) {
        self.set_mode(InputMode::Search);
    }

    pub fn to_confirm(&mut self) {
        self.set_mode(InputMode::Confirm);
    }

    pub fn to_help(&mut self) {
        self.set_mode(InputMode::Help);
    }

    pub fn to_tags(&mut self) {
        self.mode = InputMode::Tags;
    }

    pub fn to_logs(&mut self) {
        self.mode = InputMode::Logs;
    }

    pub fn to_export(&mut self) {
        self.set_mode(InputMode::Export);
    }

    // Convenience methods that delegate to buffer
    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert_char(c);
    }

    pub fn delete_char(&mut self) {
        self.buffer.delete_char();
    }

    pub fn delete_char_forward(&mut self) {
        self.buffer.delete_char_forward();
    }

    pub fn delete_word(&mut self) {
        self.buffer.delete_word();
    }

    pub fn cursor_left(&mut self) {
        self.buffer.cursor_left();
    }

    pub fn cursor_right(&mut self) {
        self.buffer.cursor_right();
    }

    pub fn cursor_home(&mut self) {
        self.buffer.cursor_home();
    }

    pub fn cursor_end(&mut self) {
        self.buffer.cursor_end();
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn clear_to_start(&mut self) {
        self.buffer.clear_to_start();
    }

    pub fn get_buffer(&self) -> &str {
        self.buffer.content()
    }

    pub fn set_buffer(&mut self, content: &str) {
        self.buffer.set_content(content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_transitions() {
        let mut state = ModeState::new();
        assert_eq!(state.mode, InputMode::Normal);

        state.to_insert();
        assert_eq!(state.mode, InputMode::Insert);

        state.to_command();
        assert_eq!(state.mode, InputMode::Command);

        state.to_normal();
        assert_eq!(state.mode, InputMode::Normal);
    }

    #[test]
    fn test_command_mode_input() {
        let mut state = ModeState::new();
        state.to_command();
        for c in "quit".chars() {
            state.insert_char(c);
        }
        assert_eq!(state.get_buffer(), "quit");
    }

    #[test]
    fn test_cancel_returns_to_normal() {
        let mut state = ModeState::new();
        state.to_command();
        state.insert_char('x');
        state.to_normal();
        assert_eq!(state.mode, InputMode::Normal);
    }

    #[test]
    fn test_text_input() {
        let mut state = ModeState::new();
        state.to_insert();

        for c in "hello".chars() {
            state.insert_char(c);
        }

        assert_eq!(state.get_buffer(), "hello");
        assert_eq!(state.buffer.cursor(), 5);

        state.delete_char();
        assert_eq!(state.get_buffer(), "hell");
    }

    #[test]
    fn test_cursor_movement() {
        let mut state = ModeState::new();
        state.set_buffer("hello");

        state.cursor_home();
        assert_eq!(state.buffer.cursor(), 0);

        state.cursor_end();
        assert_eq!(state.buffer.cursor(), 5);

        state.cursor_left();
        assert_eq!(state.buffer.cursor(), 4);

        state.cursor_right();
        assert_eq!(state.buffer.cursor(), 5);
    }

    #[test]
    fn test_is_text_input() {
        assert!(!InputMode::Normal.is_text_input());
        assert!(!InputMode::Insert.is_text_input());
        assert!(InputMode::Command.is_text_input());
        assert!(InputMode::Search.is_text_input());
        assert!(!InputMode::Help.is_text_input());
    }
}
