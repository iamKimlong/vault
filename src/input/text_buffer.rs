//! Text Buffer
//!
//! Reusable text editing buffer with cursor management.

use crossterm::event::{KeyCode, KeyModifiers};
use zeroize::Zeroizing;

/// Trait for text editing operations
pub trait TextEditing {
    fn content(&self) -> &str;
    fn cursor(&self) -> usize;
    fn set_cursor(&mut self, pos: usize);
    fn set_content(&mut self, content: &str);
    fn insert_char(&mut self, c: char);
    fn delete_char(&mut self);
    fn delete_char_forward(&mut self);
    fn delete_word(&mut self);
    fn clear_to_start(&mut self);
    fn clear(&mut self);
    fn cursor_left(&mut self);
    fn cursor_right(&mut self);
    fn cursor_home(&mut self);
    fn cursor_end(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

/// Handle common text input keys, returns true if key was handled
pub fn handle_text_key<T: TextEditing>(buf: &mut T, code: KeyCode, mods: KeyModifiers) -> bool {
    match (code, mods) {
        (KeyCode::Backspace, KeyModifiers::CONTROL | KeyModifiers::ALT) => buf.delete_word(),
        (KeyCode::Backspace, _) => buf.delete_char(),
        (KeyCode::Delete, _) => buf.delete_char_forward(),
        (KeyCode::Char('a'), KeyModifiers::CONTROL) => buf.cursor_home(),
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => buf.cursor_end(),
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => buf.clear_to_start(),
        (KeyCode::Left, _) => buf.cursor_left(),
        (KeyCode::Right, _) => buf.cursor_right(),
        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => buf.insert_char(c),
        _ => return false,
    }
    true
}

pub fn find_word_boundary_back(s: &str, from: usize) -> usize {
    let chars: Vec<char> = s.chars().take(from).collect();
    let mut pos = chars.len();
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    let is_punct = |c: char| !c.is_whitespace() && !is_word(c);

    // Skip trailing whitespace
    while pos > 0 && chars[pos - 1].is_whitespace() {
        pos -= 1;
    }
    if pos == 0 { return 0; }

    if is_word(chars[pos - 1]) {
        // Delete word chars only
        while pos > 0 && is_word(chars[pos - 1]) {
            pos -= 1;
        }
    } else {
        // Delete punctuation, then preceding word chars
        while pos > 0 && is_punct(chars[pos - 1]) {
            pos -= 1;
        }
        while pos > 0 && is_word(chars[pos - 1]) {
            pos -= 1;
        }
    }
    pos
}

// ============================================================================
// TextBuffer - for normal text
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct TextBuffer {
    content: String,
    cursor: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(content: impl Into<String>) -> Self {
        let content = content.into();
        let cursor = content.len();
        Self { content, cursor }
    }
}

impl TextEditing for TextBuffer {
    fn content(&self) -> &str {
        &self.content
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.content.len());
    }

    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.cursor = self.content.len();
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn clear(&mut self) {
        self.content.clear();
        self.cursor = 0;
    }

    fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor, c);
        self.cursor += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.content.remove(self.cursor);
    }

    fn delete_char_forward(&mut self) {
        if self.cursor >= self.content.len() {
            return;
        }
        self.content.remove(self.cursor);
    }

    fn delete_word(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let new_cursor = find_word_boundary_back(&self.content, self.cursor);
        self.content.drain(new_cursor..self.cursor);
        self.cursor = new_cursor;
    }

    fn clear_to_start(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.content.drain(..self.cursor);
        self.cursor = 0;
    }

    fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn cursor_right(&mut self) {
        if self.cursor < self.content.len() {
            self.cursor += 1;
        }
    }

    fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    fn cursor_end(&mut self) {
        self.cursor = self.content.len();
    }
}

// ============================================================================
// SecureTextBuffer - for sensitive data with automatic zeroization
// ============================================================================

#[derive(Debug, Clone)]
pub struct SecureTextBuffer {
    content: Zeroizing<String>,
    cursor: usize,
}

impl Default for SecureTextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureTextBuffer {
    pub fn new() -> Self {
        Self {
            content: Zeroizing::new(String::new()),
            cursor: 0,
        }
    }
}

impl TextEditing for SecureTextBuffer {
    fn content(&self) -> &str {
        &self.content
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.content.len());
    }

    fn set_content(&mut self, content: &str) {
        *self.content = content.to_string();
        self.cursor = self.content.len();
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn clear(&mut self) {
        self.content.clear();
        self.cursor = 0;
    }

    fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor, c);
        self.cursor += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.content.remove(self.cursor);
    }

    fn delete_char_forward(&mut self) {
        if self.cursor >= self.content.len() {
            return;
        }
        self.content.remove(self.cursor);
    }

    fn delete_word(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let before: String = self.content.chars().take(self.cursor).collect();
        let after: String = self.content.chars().skip(self.cursor).collect();
        let new_cursor = find_word_boundary_back(&before, self.cursor);
        *self.content = format!("{}{}", &before[..new_cursor], after);
        self.cursor = new_cursor;
    }

    fn clear_to_start(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let after: String = self.content.chars().skip(self.cursor).collect();
        *self.content = after;
        self.cursor = 0;
    }

    fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn cursor_right(&mut self) {
        if self.cursor < self.content.len() {
            self.cursor += 1;
        }
    }

    fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    fn cursor_end(&mut self) {
        self.cursor = self.content.len();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_delete() {
        let mut buf = TextBuffer::new();
        buf.insert_char('h');
        buf.insert_char('i');
        assert_eq!(buf.content(), "hi");
        assert_eq!(buf.cursor(), 2);

        buf.delete_char();
        assert_eq!(buf.content(), "h");
        assert_eq!(buf.cursor(), 1);
    }

    #[test]
    fn test_cursor_movement() {
        let mut buf = TextBuffer::with_content("hello");
        assert_eq!(buf.cursor(), 5);

        buf.cursor_home();
        assert_eq!(buf.cursor(), 0);

        buf.cursor_end();
        assert_eq!(buf.cursor(), 5);

        buf.cursor_left();
        assert_eq!(buf.cursor(), 4);

        buf.cursor_right();
        assert_eq!(buf.cursor(), 5);
    }

    #[test]
    fn test_delete_word_simple() {
        let mut buf = TextBuffer::with_content("hello world");
        buf.delete_word();
        assert_eq!(buf.content(), "hello ");
    }

    #[test]
    fn test_delete_word_with_spaces() {
        let mut buf = TextBuffer::with_content("hello   ");
        buf.delete_word();
        assert_eq!(buf.content(), "");
    }

    #[test]
    fn test_delete_word_symbol() {
        let mut buf = TextBuffer::with_content("hello!");
        buf.delete_word();
        assert_eq!(buf.content(), "");
    }

    #[test]
    fn test_clear_to_start() {
        let mut buf = TextBuffer::with_content("hello world");
        buf.set_cursor(6);
        buf.clear_to_start();
        assert_eq!(buf.content(), "world");
        assert_eq!(buf.cursor(), 0);
    }

    #[test]
    fn test_handle_text_key() {
        let mut buf = TextBuffer::new();

        assert!(handle_text_key(&mut buf, KeyCode::Char('a'), KeyModifiers::NONE));
        assert_eq!(buf.content(), "a");

        assert!(handle_text_key(&mut buf, KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(buf.content(), "");

        assert!(!handle_text_key(&mut buf, KeyCode::Enter, KeyModifiers::NONE));
    }

    #[test]
    fn test_secure_buffer_basic() {
        let mut buf = SecureTextBuffer::new();
        buf.insert_char('p');
        buf.insert_char('a');
        buf.insert_char('s');
        buf.insert_char('s');
        assert_eq!(buf.content(), "pass");
        assert_eq!(buf.cursor(), 4);
    }

    #[test]
    fn test_secure_buffer_clear() {
        let mut buf = SecureTextBuffer::new();
        buf.set_content("secret");
        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.cursor(), 0);
    }

    #[test]
    fn test_secure_buffer_delete_word() {
        let mut buf = SecureTextBuffer::new();
        buf.set_content("hello world");
        buf.delete_word();
        assert_eq!(buf.content(), "hello ");
    }
}
