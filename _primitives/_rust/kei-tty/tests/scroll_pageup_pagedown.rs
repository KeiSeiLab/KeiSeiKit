//! Scroll-math regression tests for `App` + `ui::clamp_scroll`.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use kei_tty::app::{App, LineKind};
use kei_tty::keys::handle_key;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

#[test]
fn pageup_decreases_scroll_saturating_at_zero() {
    let mut app = App::new();
    app.scroll = 3;
    let _ = handle_key(key(KeyCode::PageUp), &mut app);
    assert_eq!(app.scroll, 0, "saturating sub by 10 from 3 should be 0");
}

#[test]
fn pagedown_increases_scroll_by_page_step() {
    let mut app = App::new();
    app.scroll = 5;
    let _ = handle_key(key(KeyCode::PageDown), &mut app);
    assert_eq!(app.scroll, 15);
}

#[test]
fn pagedown_saturates_at_u16_max() {
    let mut app = App::new();
    app.scroll = u16::MAX - 3;
    let _ = handle_key(key(KeyCode::PageDown), &mut app);
    assert_eq!(app.scroll, u16::MAX);
}

#[test]
fn ctrl_l_sets_scroll_sentinel_and_keeps_history() {
    let mut app = App::new();
    for i in 0..5 {
        app.push_line(LineKind::User, format!("line {i}"));
    }
    let outcome = handle_key(
        KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        },
        &mut app,
    );
    let _ = outcome;
    assert_eq!(app.scroll, u16::MAX);
    assert_eq!(app.history.len(), 5, "history must be retained across Ctrl+L");
}
