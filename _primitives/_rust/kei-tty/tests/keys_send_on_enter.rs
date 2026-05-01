//! KeyEvent → `App` state-transition tests.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use kei_tty::app::App;
use kei_tty::keys::{handle_key, KeyOutcome};

fn key(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: mods,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

#[test]
fn typing_and_enter_round_trip_sends_message() {
    let mut app = App::new();
    for c in "hello".chars() {
        let _ = handle_key(key(KeyCode::Char(c), KeyModifiers::NONE), &mut app);
    }
    assert_eq!(app.input, "hello");
    match handle_key(key(KeyCode::Enter, KeyModifiers::NONE), &mut app) {
        KeyOutcome::Send(msg) => assert_eq!(msg, "hello"),
        other => panic!("expected Send, got {:?}", std::mem::discriminant(&other)),
    }
    assert!(app.input.is_empty(), "input must be drained on send");
}

#[test]
fn shift_enter_inserts_newline_and_does_not_send() {
    let mut app = App::new();
    app.input = "line one".into();
    let outcome = handle_key(key(KeyCode::Enter, KeyModifiers::SHIFT), &mut app);
    assert!(matches!(outcome, KeyOutcome::Nothing));
    assert_eq!(app.input, "line one\n");
}

#[test]
fn enter_while_in_flight_does_not_double_send() {
    let mut app = App::new();
    app.in_flight = true;
    app.input = "hi".into();
    let outcome = handle_key(key(KeyCode::Enter, KeyModifiers::NONE), &mut app);
    assert!(matches!(outcome, KeyOutcome::Nothing));
    assert_eq!(app.input, "hi", "input must NOT be drained when in_flight");
}

#[test]
fn ctrl_c_idle_quits() {
    let mut app = App::new();
    app.in_flight = false;
    let outcome = handle_key(key(KeyCode::Char('c'), KeyModifiers::CONTROL), &mut app);
    assert!(matches!(outcome, KeyOutcome::Quit));
}

#[test]
fn ctrl_c_streaming_cancels() {
    let mut app = App::new();
    app.in_flight = true;
    let outcome = handle_key(key(KeyCode::Char('c'), KeyModifiers::CONTROL), &mut app);
    assert!(matches!(outcome, KeyOutcome::Cancel));
}

#[test]
fn backspace_pops_one_char() {
    let mut app = App::new();
    app.input = "abc".into();
    let _ = handle_key(key(KeyCode::Backspace, KeyModifiers::NONE), &mut app);
    assert_eq!(app.input, "ab");
}

#[test]
fn whitespace_only_input_does_not_send() {
    let mut app = App::new();
    app.input = "   \t  ".into();
    let outcome = handle_key(key(KeyCode::Enter, KeyModifiers::NONE), &mut app);
    assert!(matches!(outcome, KeyOutcome::Nothing));
}
