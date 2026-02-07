//! Dialog popups (confirm, message, password)

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use super::layout::{centered_rect_fixed, create_popup_block};

pub struct ConfirmDialog<'a> {
    title: &'a str,
    message: &'a str,
}

impl<'a> ConfirmDialog<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self { title, message }
    }
}

impl Widget for ConfirmDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect_fixed(50, 6, area, true);
        Clear.render(popup_area, buf);

        let block = create_popup_block(self.title, Color::Yellow);
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        Paragraph::new(self.message)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .render(Rect::new(inner.x, inner.y, inner.width, 2), buf);

        render_confirm_hint(buf, inner.x, inner.y + 3, inner.width);
    }
}

fn render_confirm_hint(buf: &mut Buffer, x: u16, y: u16, _width: u16) {
    let hint = Line::from(vec![
        Span::styled("[y]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" Yes  "),
        Span::styled("[n]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" No"),
    ]);
    buf.set_line(x, y, &hint, 20);
}

#[allow(dead_code)]
pub struct MessagePopup<'a> {
    title: &'a str,
    message: &'a str,
    style: Style,
}

#[allow(dead_code)]
impl<'a> MessagePopup<'a> {
    pub fn info(title: &'a str, message: &'a str) -> Self {
        Self { title, message, style: Style::default().fg(Color::Magenta) }
    }

    pub fn error(title: &'a str, message: &'a str) -> Self {
        Self { title, message, style: Style::default().fg(Color::Red) }
    }

    pub fn success(title: &'a str, message: &'a str) -> Self {
        Self { title, message, style: Style::default().fg(Color::Green) }
    }
}

impl Widget for MessagePopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect_fixed(60, 5, area, true);
        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(self.style)
            .style(Style::default().bg(Color::Black));
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        Paragraph::new(self.message)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

pub struct PasswordDialog<'a> {
    title: &'a str,
    prompt: &'a str,
    value: &'a str,
    cursor: usize,
    error: Option<&'a str>,
}

impl<'a> PasswordDialog<'a> {
    pub fn new(title: &'a str, prompt: &'a str, value: &'a str, cursor: usize) -> Self {
        Self { title, prompt, value, cursor, error: None }
    }

    pub fn error(mut self, err: &'a str) -> Self {
        self.error = Some(err);
        self
    }
}

impl Widget for PasswordDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dialog_width = 40;
        let height = 6;
        let popup_area = centered_rect_fixed(dialog_width, height, area, false);
        Clear.render(popup_area, buf);

        let block = create_popup_block(self.title, Color::Magenta);
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        buf.set_string(inner.x, inner.y, self.prompt, Style::default().fg(Color::White));

        let field_width = inner.width as usize;
        let masked: String = "•".repeat(self.value.chars().count());
        let scroll = if self.cursor >= field_width.saturating_sub(1) {
            self.cursor.saturating_sub(field_width.saturating_sub(2))
        } else {
            0
        };
        let visible: String = masked.chars().skip(scroll).take(field_width).collect();
        let adjusted_cursor = self.cursor.saturating_sub(scroll);

        let value_y = inner.y + 2;
        fill_password_background(buf, inner.x, value_y, inner.width);
        buf.set_string(inner.x, value_y, &visible, Style::default().fg(Color::Yellow).bg(Color::DarkGray));

        render_password_cursor(buf, inner.x + adjusted_cursor as u16, value_y, inner.x + inner.width);

        if let Some(err) = self.error {
            buf.set_string(inner.x, inner.y + 3, err, Style::default().fg(Color::Red));
        }
    }
}

fn fill_password_background(buf: &mut Buffer, x: u16, y: u16, width: u16) {
    for cx in x..x + width {
        if let Some(cell) = buf.cell_mut((cx, y)) {
            cell.set_style(Style::default().bg(Color::DarkGray));
        }
    }
}

fn render_password_cursor(buf: &mut Buffer, cursor_x: u16, y: u16, max_x: u16) {
    if cursor_x >= max_x {
        return;
    }
    if let Some(cell) = buf.cell_mut((cursor_x, y)) {
        cell.set_style(Style::default().bg(Color::White).fg(Color::Black));
    }
}
