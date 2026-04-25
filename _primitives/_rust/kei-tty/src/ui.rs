//! Ratatui frame rendering. Pure read-only function over `&App`.
//!
//! Layout (vertical splits):
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │ chat history (Paragraph, scrollable)        │  ~70%
//! ├─────────────────────────────────────────────┤
//! │ input bar (multi-line)                      │  ~25%
//! ├─────────────────────────────────────────────┤
//! │ status line                                 │  fixed 1 row
//! └─────────────────────────────────────────────┘
//! ```
//! Tool-call boxes are rendered inline inside the chat history (yellow for
//! `tool_use`, green for `tool_result`).

use crate::app::{App, Line as AppLine, LineKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

/// Top-level entry — draws the whole UI for one frame.
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(1),
        ])
        .split(f.area());
    draw_history(f, chunks[0], app);
    draw_input(f, chunks[1], app);
    draw_status(f, chunks[2], app);
}

/// Render the rolling chat history including the in-progress streaming
/// assistant turn (if any) as the last visible line.
fn draw_history(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = app.history.iter().map(render_line).collect();
    if !app.current_streaming.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("● ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(app.current_streaming.clone(), Style::default().fg(Color::White)),
        ]));
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" chat ")
        .style(Style::default().fg(Color::DarkGray));
    let scroll = clamp_scroll(app.scroll, lines.len(), area.height);
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(para, area);
}

/// Render the user's input bar.
fn draw_input(f: &mut Frame, area: Rect, app: &App) {
    let title = if app.in_flight { " input (streaming…) " } else { " input " };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(app.input.as_str())
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));
    f.render_widget(para, area);
}

/// Render the status line (bottom one row).
fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let cid = app.conversation_id.as_deref().unwrap_or("(no conversation yet)");
    let text = format!(
        "{}  |  conv: {}  |  Ctrl+D quit  Ctrl+C cancel  PgUp/PgDn scroll",
        app.status, cid
    );
    let para = Paragraph::new(text).style(Style::default().fg(Color::Gray));
    f.render_widget(para, area);
}

/// Map a history [`AppLine`] to a styled ratatui [`Line`].
fn render_line(l: &AppLine) -> Line<'static> {
    let (prefix, colour) = style_for(l.kind);
    Line::from(vec![
        Span::styled(prefix, Style::default().fg(colour).add_modifier(Modifier::BOLD)),
        Span::styled(l.text.clone(), Style::default().fg(colour)),
    ])
}

/// Per-`LineKind` (prefix, foreground colour). Hard-coded palette — no theme
/// system on purpose.
fn style_for(kind: LineKind) -> (&'static str, Color) {
    match kind {
        LineKind::User => ("> ", Color::Cyan),
        LineKind::Assistant => ("● ", Color::White),
        LineKind::ToolUse => ("⚙ ", Color::Yellow),
        LineKind::ToolResult => ("✓ ", Color::Green),
        LineKind::Error => ("✗ ", Color::Red),
        LineKind::Sentiment => ("~ ", Color::Magenta),
        LineKind::System => ("· ", Color::DarkGray),
    }
}

/// Clamp `requested` so the renderer never tries to scroll past the bottom.
/// `u16::MAX` is the sentinel used by Ctrl+L to mean "stick to bottom"; we
/// resolve it here to the last full page so freshly streamed text stays in
/// view.
pub(crate) fn clamp_scroll(requested: u16, line_count: usize, area_height: u16) -> u16 {
    let viewport = area_height.saturating_sub(2); // borders
    let max_scroll = (line_count as u16).saturating_sub(viewport);
    if requested == u16::MAX {
        max_scroll
    } else {
        requested.min(max_scroll)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_scroll_caps_at_max() {
        assert_eq!(clamp_scroll(100, 12, 10), 4); // viewport 8, content 12 → max 4
    }

    #[test]
    fn clamp_scroll_sentinel_resolves_to_max() {
        assert_eq!(clamp_scroll(u16::MAX, 12, 10), 4);
    }

    #[test]
    fn clamp_scroll_short_history_zero() {
        assert_eq!(clamp_scroll(5, 3, 10), 0);
    }

    #[test]
    fn style_for_distinct_per_kind() {
        let kinds = [
            LineKind::User,
            LineKind::Assistant,
            LineKind::ToolUse,
            LineKind::ToolResult,
            LineKind::Error,
            LineKind::Sentiment,
            LineKind::System,
        ];
        let colours: Vec<_> = kinds.iter().map(|k| style_for(*k).1).collect();
        // No two kinds share the same colour.
        for (i, c1) in colours.iter().enumerate() {
            for c2 in &colours[i + 1..] {
                assert_ne!(c1, c2);
            }
        }
    }
}
