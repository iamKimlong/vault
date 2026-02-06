//! Layout helpers and common rendering utilities

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Widget, Block, BorderType, Borders, Paragraph},
};

/// Percentage based layout
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let content_area = Rect::new(r.x, r.y, r.width, r.height.saturating_sub(2));
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(content_area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Fixed sized layout
pub fn centered_rect_fixed(width: u16, height: u16, r: Rect, unlocked: bool) -> Rect {
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let available_height = if unlocked { r.height.saturating_sub(2) } else { r.height };
    let remainder = (available_height.saturating_sub(height)) % 2;
    let adjusted_height = height + remainder;
    let y = r.y + (available_height.saturating_sub(adjusted_height)) / 2;
    Rect::new(x, y, width.min(r.width), adjusted_height.min(r.height))
}

pub fn create_popup_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color))
        .style(Style::default().bg(Color::Black))
}

pub fn render_empty_message(area: Rect, buf: &mut Buffer, msg: &str) {
    Paragraph::new(msg)
        .style(Style::default().fg(Color::DarkGray))
        .render(area, buf);
}

pub fn render_separator_line(buf: &mut Buffer, x: u16, y: u16, width: u16) {
    for px in x..x + width {
        buf.set_string(px, y, "─", Style::default().fg(Color::DarkGray));
    }
}

pub fn render_footer(buf: &mut Buffer, popup: Rect, text: &str) {
    let y = popup.y + popup.height - 1;
    let x = popup.x + (popup.width.saturating_sub(text.len() as u16)) / 2;
    buf.set_string(x, y, text, Style::default().fg(Color::DarkGray));
}

pub fn highlight_row(buf: &mut Buffer, x: u16, y: u16, width: u16) {
    for px in x..x + width {
        if let Some(cell) = buf.cell_mut((px, y)) {
            cell.set_bg(Color::DarkGray);
        }
    }
}

pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    format!("{}…", &s[..max_len.saturating_sub(1)])
}

pub fn render_text_at_virtual_x(
    buf: &mut Buffer,
    base_x: u16,
    y: u16,
    view_width: u16,
    h_offset: usize,
    virtual_x: u16,
    text: &str,
    style: Style,
) {
    let h_off = h_offset as u16;
    let text_len = text.chars().count() as u16;

    if virtual_x + text_len <= h_off {
        return;
    }
    if virtual_x >= h_off + view_width {
        return;
    }

    let screen_x = if virtual_x >= h_off { base_x + virtual_x - h_off } else { base_x };
    let skip_chars = if virtual_x < h_off { (h_off - virtual_x) as usize } else { 0 };
    let available = (base_x + view_width).saturating_sub(screen_x) as usize;

    let visible_text: String = text.chars().skip(skip_chars).take(available).collect();
    buf.set_string(screen_x, y, &visible_text, style);
}
