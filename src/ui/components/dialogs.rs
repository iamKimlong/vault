//! Dialog popups (confirm, message, password)

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use super::input_field::InputField;
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

pub struct MessagePopup<'a> {
    title: &'a str,
    message: &'a str,
    style: Style,
}

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
        let height = if self.error.is_some() { 7 } else { 6 };
        let popup_area = centered_rect_fixed(40, height, area, false);
        Clear.render(popup_area, buf);

        let block = create_popup_block(self.title, Color::Magenta);
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        buf.set_string(inner.x, inner.y, self.prompt, Style::default().fg(Color::White));

        let input_rect = Rect::new(inner.x, inner.y + 1, inner.width, 2);
        InputField::new("", self.value, self.cursor)
            .masked()
            .style(Style::default().fg(Color::Yellow))
            .render(input_rect, buf);

        if let Some(err) = self.error {
            buf.set_string(inner.x, inner.y + 3, err, Style::default().fg(Color::Red));
        }
    }
}
