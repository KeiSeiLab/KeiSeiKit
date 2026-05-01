//! TUI state machine — pure data, no I/O.
//!
//! Owns the chat history, the current input buffer, scroll position, and
//! the "in-flight" flag (true while we are draining an SSE stream).
//! The actual event loop lives in [`crate::runner`] which `tokio::select!`s
//! over keyboard events and a channel of [`ChatEvent`]s shovelled in by
//! the daemon client task.

use crate::types::ChatEvent;

/// Maximum number of message lines retained in history (older ones are
/// dropped to keep memory bounded for long sessions).
pub const HISTORY_CAP: usize = 4096;

/// One persisted history entry. `kind` drives the colour in [`crate::ui`].
#[derive(Debug, Clone)]
pub struct Line {
    pub kind: LineKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    User,
    Assistant,
    ToolUse,
    ToolResult,
    Error,
    Sentiment,
    System,
}

/// All UI state. Cheap to clone on a per-frame basis; cloning is avoided
/// in the hot loop by passing `&App` to the renderer.
#[derive(Debug, Default)]
pub struct App {
    pub history: Vec<Line>,
    pub input: String,
    pub scroll: u16,
    pub in_flight: bool,
    pub current_streaming: String,
    pub conversation_id: Option<String>,
    pub status: String,
    pub should_quit: bool,
    pub cancel_requested: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            status: "ready — type a message and press Enter".into(),
            ..Self::default()
        }
    }

    /// Append a new history line, evicting the oldest if cap exceeded.
    pub fn push_line(&mut self, kind: LineKind, text: impl Into<String>) {
        self.history.push(Line { kind, text: text.into() });
        if self.history.len() > HISTORY_CAP {
            self.history.drain(..self.history.len() - HISTORY_CAP);
        }
    }

    /// Apply a parsed [`ChatEvent`] to the state machine.
    pub fn apply_event(&mut self, ev: ChatEvent) {
        match ev {
            ChatEvent::Token(t) => self.current_streaming.push_str(&t),
            ChatEvent::Sentiment { tag, confidence } => self.apply_sentiment(tag, confidence),
            ChatEvent::ToolUseStart { name, id } => {
                self.push_line(LineKind::ToolUse, format!("[tool_use: {name} #{id}]"));
            }
            ChatEvent::ToolResult { id, output } => {
                self.push_line(LineKind::ToolResult, format!("[tool_result #{id}] {output}"));
            }
            ChatEvent::Error(msg) => self.apply_error(msg),
            ChatEvent::Done { conversation_id } => self.apply_done(conversation_id),
            ChatEvent::Other(tag) => {
                self.push_line(LineKind::System, format!("[unknown event: {tag}]"));
            }
        }
    }

    fn apply_sentiment(&mut self, tag: String, confidence: f32) {
        self.push_line(
            LineKind::Sentiment,
            format!("[sentiment: {tag} ({:.0}%)]", confidence * 100.0),
        );
    }

    fn apply_error(&mut self, msg: String) {
        self.flush_streaming();
        self.push_line(LineKind::Error, format!("ERROR: {msg}"));
    }

    fn apply_done(&mut self, conversation_id: String) {
        self.flush_streaming();
        self.conversation_id = Some(conversation_id);
        self.in_flight = false;
        self.status = "ready".into();
    }

    /// Move the in-progress streaming buffer into history (called on Done /
    /// Error so the assistant turn becomes a final history line).
    fn flush_streaming(&mut self) {
        if !self.current_streaming.is_empty() {
            let text = std::mem::take(&mut self.current_streaming);
            self.push_line(LineKind::Assistant, text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_token_accumulates_streaming() {
        let mut app = App::new();
        app.apply_event(ChatEvent::Token("Hel".into()));
        app.apply_event(ChatEvent::Token("lo".into()));
        assert_eq!(app.current_streaming, "Hello");
        assert!(app.history.iter().all(|l| l.kind != LineKind::Assistant));
    }

    #[test]
    fn apply_done_flushes_streaming_into_history() {
        let mut app = App::new();
        app.in_flight = true;
        app.apply_event(ChatEvent::Token("Hi".into()));
        app.apply_event(ChatEvent::Done { conversation_id: "c1".into() });
        assert_eq!(app.current_streaming, "");
        assert!(!app.in_flight);
        assert_eq!(app.conversation_id.as_deref(), Some("c1"));
        let last = app.history.last().unwrap();
        assert_eq!(last.kind, LineKind::Assistant);
        assert_eq!(last.text, "Hi");
    }

    #[test]
    fn apply_error_flushes_and_logs_error() {
        let mut app = App::new();
        app.apply_event(ChatEvent::Token("partial".into()));
        app.apply_event(ChatEvent::Error("boom".into()));
        assert_eq!(app.history[0].kind, LineKind::Assistant);
        assert_eq!(app.history[1].kind, LineKind::Error);
    }

    #[test]
    fn history_cap_evicts_oldest() {
        let mut app = App::new();
        for i in 0..(HISTORY_CAP + 5) {
            app.push_line(LineKind::System, format!("line {i}"));
        }
        assert_eq!(app.history.len(), HISTORY_CAP);
        assert_eq!(app.history[0].text, "line 5");
    }
}
