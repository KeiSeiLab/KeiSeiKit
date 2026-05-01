//! Keyboard handler — translates [`KeyEvent`] into [`KeyOutcome`].
//!
//! Bindings:
//!   * `Enter`                 — send the input buffer (returns [`KeyOutcome::Send`])
//!   * `Shift+Enter`           — insert newline into the input buffer
//!   * `Ctrl+C`                — cancel the in-flight request
//!   * `Ctrl+D`                — exit the program
//!   * `Ctrl+L`                — clear the visible chat (history retained)
//!   * `PageUp` / `PageDown`   — scroll history by one page
//!   * `Backspace`             — delete one character from input
//!   * any printable character — append to input buffer

use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Result of mapping one keypress.
pub enum KeyOutcome {
    /// The user pressed Enter and the buffer was non-empty; payload is the
    /// drained message body. The caller is responsible for sending it.
    Send(String),
    /// User asked to exit (Ctrl+D).
    Quit,
    /// User asked to cancel the in-flight request (Ctrl+C while streaming).
    Cancel,
    /// Pure view-state edit (input buffer, scroll, clear); nothing else.
    Nothing,
}

/// Page step for PageUp/PageDown. Hard-coded — the renderer does not have
/// access to the terminal height at this layer, and 10 lines is a workable
/// default for the standard 80×24 terminal.
const PAGE_STEP: u16 = 10;

/// Top-level dispatcher.
pub fn handle_key(k: KeyEvent, app: &mut App) -> KeyOutcome {
    let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
    let shift = k.modifiers.contains(KeyModifiers::SHIFT);
    match (k.code, ctrl, shift) {
        (KeyCode::Char('d'), true, _) => KeyOutcome::Quit,
        (KeyCode::Char('c'), true, _) => handle_ctrl_c(app),
        (KeyCode::Char('l'), true, _) => clear_view(app),
        (KeyCode::PageUp, _, _) => scroll_up(app),
        (KeyCode::PageDown, _, _) => scroll_down(app),
        (KeyCode::Enter, _, true) => insert_newline(app),
        (KeyCode::Enter, _, _) => try_send(app),
        (KeyCode::Backspace, _, _) => backspace(app),
        (KeyCode::Char(c), false, _) => insert_char(app, c),
        _ => KeyOutcome::Nothing,
    }
}

fn handle_ctrl_c(app: &mut App) -> KeyOutcome {
    if app.in_flight {
        KeyOutcome::Cancel
    } else {
        KeyOutcome::Quit
    }
}

fn clear_view(app: &mut App) -> KeyOutcome {
    app.scroll = u16::MAX; // forces renderer to bottom; history retained
    app.status = format!("cleared view ({} lines retained)", app.history.len());
    KeyOutcome::Nothing
}

fn scroll_up(app: &mut App) -> KeyOutcome {
    app.scroll = app.scroll.saturating_sub(PAGE_STEP);
    KeyOutcome::Nothing
}

fn scroll_down(app: &mut App) -> KeyOutcome {
    app.scroll = app.scroll.saturating_add(PAGE_STEP);
    KeyOutcome::Nothing
}

fn insert_newline(app: &mut App) -> KeyOutcome {
    app.input.push('\n');
    KeyOutcome::Nothing
}

fn try_send(app: &mut App) -> KeyOutcome {
    if app.in_flight || app.input.trim().is_empty() {
        return KeyOutcome::Nothing;
    }
    let msg = std::mem::take(&mut app.input);
    KeyOutcome::Send(msg)
}

fn backspace(app: &mut App) -> KeyOutcome {
    app.input.pop();
    KeyOutcome::Nothing
}

fn insert_char(app: &mut App, c: char) -> KeyOutcome {
    app.input.push(c);
    KeyOutcome::Nothing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEventKind, KeyEventState, KeyModifiers};

    fn ev(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn enter_with_text_sends_and_clears_input() {
        let mut app = App::new();
        app.input = "hello".into();
        match handle_key(ev(KeyCode::Enter, KeyModifiers::NONE), &mut app) {
            KeyOutcome::Send(s) => assert_eq!(s, "hello"),
            _ => panic!("expected Send"),
        }
        assert!(app.input.is_empty());
    }

    #[test]
    fn enter_empty_input_does_nothing() {
        let mut app = App::new();
        assert!(matches!(
            handle_key(ev(KeyCode::Enter, KeyModifiers::NONE), &mut app),
            KeyOutcome::Nothing
        ));
    }

    #[test]
    fn shift_enter_inserts_newline() {
        let mut app = App::new();
        app.input = "ab".into();
        handle_key(ev(KeyCode::Enter, KeyModifiers::SHIFT), &mut app);
        assert_eq!(app.input, "ab\n");
    }

    #[test]
    fn ctrl_c_in_flight_cancels() {
        let mut app = App::new();
        app.in_flight = true;
        assert!(matches!(
            handle_key(ev(KeyCode::Char('c'), KeyModifiers::CONTROL), &mut app),
            KeyOutcome::Cancel
        ));
    }

    #[test]
    fn ctrl_d_quits() {
        let mut app = App::new();
        assert!(matches!(
            handle_key(ev(KeyCode::Char('d'), KeyModifiers::CONTROL), &mut app),
            KeyOutcome::Quit
        ));
    }
}
