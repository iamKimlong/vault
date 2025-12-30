//! Input field widget

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct InputField<'a> {
    label: &'a str,
    value: &'a str,
    cursor: usize,
    masked: bool,
}

impl<'a> InputField<'a> {
    pub fn new(label: &'a str, value: &'a str, cursor: usize) -> Self {
        Self { label, value, cursor, masked: false }
    }

    pub fn masked(mut self) -> Self {
        self.masked = true;
        self
    }
}

impl Widget for InputField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.x, area.y, self.label, Style::default().fg(Color::Cyan));

        let input_y = area.y + 1;
        render_input_background(buf, area.x, input_y, area.width);
        render_input_value(buf, area.x, input_y, self.value, self.masked);
        render_input_cursor(buf, area.x, input_y, area.width, self.cursor);
    }
}

fn render_input_background(buf: &mut Buffer, x: u16, y: u16, width: u16) {
    for px in x..x + width {
        if let Some(cell) = buf.cell_mut((px, y)) {
            cell.set_bg(Color::DarkGray);
        }
    }
}

fn render_input_value(buf: &mut Buffer, x: u16, y: u16, value: &str, masked: bool) {
    let display = if masked { "*".repeat(value.len()) } else { value.to_string() };
    buf.set_string(x, y, &display, Style::default().fg(Color::White));
}

fn render_input_cursor(buf: &mut Buffer, x: u16, y: u16, width: u16, cursor: usize) {
    let cursor_x = x + cursor as u16;
    if cursor_x >= x + width {
        return;
    }
    if let Some(cell) = buf.cell_mut((cursor_x, y)) {
        cell.set_style(Style::default().bg(Color::White).fg(Color::Black));
    }
}
