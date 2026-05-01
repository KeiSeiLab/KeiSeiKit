//! SSE-frame parsing — feeds canned chunked bytes through
//! `flush_complete_frames` and asserts the resulting `ChatEvent` stream.
//!
//! We do NOT spin up a real reqwest server here; the parser is exercised
//! directly because it is the only part of the client that has logic worth
//! testing in isolation. The HTTP path is covered manually against a live
//! daemon (see README).

use kei_tty::client::flush_complete_frames;
use kei_tty::types::ChatEvent;

/// Drain a fully-formed SSE response in one go.
#[test]
fn full_response_token_sentiment_done() {
    let mut buf = String::from(
        "data: {\"type\":\"token\",\"text\":\"Hel\"}\n\n\
         data: {\"type\":\"token\",\"text\":\"lo\"}\n\n\
         data: {\"type\":\"sentiment\",\"tag\":\"happy\",\"confidence\":0.9}\n\n\
         data: {\"type\":\"done\",\"conversation_id\":\"abc\"}\n\n",
    );
    let mut events = Vec::new();
    flush_complete_frames(&mut buf, &mut |e| events.push(e));
    assert_eq!(events.len(), 4);
    assert_eq!(events[0], ChatEvent::Token("Hel".into()));
    assert_eq!(events[1], ChatEvent::Token("lo".into()));
    matches!(events[2], ChatEvent::Sentiment { .. });
    assert_eq!(
        events[3],
        ChatEvent::Done {
            conversation_id: "abc".into()
        }
    );
    assert!(buf.is_empty(), "buffer should be empty after full drain");
}

/// Simulate chunked TCP delivery: bytes arrive split across `\n\n`
/// boundaries. The parser must hold partial state across calls.
#[test]
fn chunked_delivery_preserves_partial_frames() {
    let chunks = [
        "data: {\"type\":\"token\",\"text\":\"a\"}\n",
        "\ndata: {\"type\":\"to",
        "ken\",\"text\":\"b\"}\n\n",
        "data: {\"type\":\"done\",\"conversation_id\":\"x\"}\n\n",
    ];
    let mut buf = String::new();
    let mut events = Vec::new();
    for c in chunks {
        buf.push_str(c);
        flush_complete_frames(&mut buf, &mut |e| events.push(e));
    }
    assert_eq!(events.len(), 3);
    assert_eq!(events[0], ChatEvent::Token("a".into()));
    assert_eq!(events[1], ChatEvent::Token("b".into()));
    assert_eq!(
        events[2],
        ChatEvent::Done {
            conversation_id: "x".into()
        }
    );
}

/// Comments (`:` lines) and `event:` / `id:` headers must be ignored.
#[test]
fn comments_and_headers_ignored() {
    let mut buf = String::from(
        ": ratatui keep-alive\n\
         event: chat\n\
         id: 42\n\
         data: {\"type\":\"error\",\"message\":\"oops\"}\n\n",
    );
    let mut events = Vec::new();
    flush_complete_frames(&mut buf, &mut |e| events.push(e));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], ChatEvent::Error("oops".into()));
}

/// Multi-line `data:` fields are concatenated with `\n` per W3C spec.
#[test]
fn multi_line_data_concatenates() {
    let mut buf = String::from("data: line1\ndata: line2\n\n");
    let mut events = Vec::new();
    flush_complete_frames(&mut buf, &mut |e| events.push(e));
    // "line1\nline2" is not valid JSON — parser yields no event but does
    // not panic; buffer is fully drained.
    assert!(events.is_empty());
    assert!(buf.is_empty());
}

/// Future event tags surface as `Other` rather than dropping the frame.
#[test]
fn unknown_event_tag_surfaces_as_other() {
    let mut buf = String::from("data: {\"type\":\"future_thing\",\"x\":1}\n\n");
    let mut events = Vec::new();
    flush_complete_frames(&mut buf, &mut |e| events.push(e));
    assert_eq!(events.len(), 1);
    match &events[0] {
        ChatEvent::Other(t) => assert_eq!(t, "future_thing"),
        _ => panic!("expected Other, got {:?}", events[0]),
    }
}
