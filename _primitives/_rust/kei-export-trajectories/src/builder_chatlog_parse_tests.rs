//! Unit tests for `builder_chatlog_parse`. Lives in a sibling file so the
//! parser module stays under the 200-LOC Constructor Pattern budget.

use super::builder_chatlog_parse::parse_chatlog_turns;
use super::sharegpt::From as ShareGptFrom;

#[test]
fn no_markers_yields_legacy_single_turn() {
    let v = parse_chatlog_turns("hello world");
    assert_eq!(v.len(), 1);
    assert!(matches!(v[0].from, ShareGptFrom::Gpt));
    assert_eq!(v[0].value, "<think>\n</think>\nhello world");
}

#[test]
fn tool_response_yields_tool_variant_with_inner_payload() {
    let s = "before<tool_response>RESP</tool_response>after";
    let v = parse_chatlog_turns(s);
    assert_eq!(v.len(), 3);
    assert!(matches!(v[0].from, ShareGptFrom::Gpt));
    assert_eq!(v[0].value, "before");
    assert!(matches!(v[1].from, ShareGptFrom::Tool));
    assert_eq!(v[1].value, "RESP");
    assert!(matches!(v[2].from, ShareGptFrom::Gpt));
    assert_eq!(v[2].value, "after");
}

#[test]
fn tool_call_block_kept_with_markers() {
    let s = "<tool_call>{\"name\":\"x\"}</tool_call>";
    let v = parse_chatlog_turns(s);
    assert_eq!(v.len(), 1);
    assert!(matches!(v[0].from, ShareGptFrom::Gpt));
    assert!(v[0].value.starts_with("<tool_call>"));
}

#[test]
fn whitespace_segments_dropped() {
    let s = "   <tool_response>X</tool_response>   ";
    let v = parse_chatlog_turns(s);
    assert_eq!(v.len(), 1);
    assert!(matches!(v[0].from, ShareGptFrom::Tool));
}

/// Realistic Hermes-style multi-tool flow:
/// gpt-think → tool_call → tool_response → gpt-think → tool_call →
/// tool_response → gpt-final. Locks in the iterative parser's ordering and
/// role discrimination across more than one tool round-trip — the previous
/// suite only exercised single-block cases.
#[test]
fn multi_tool_sequence_preserves_order_and_roles() {
    let s = "think1<tool_call>{\"name\":\"a\"}</tool_call>\
             <tool_response>RESP_A</tool_response>\
             think2<tool_call>{\"name\":\"b\"}</tool_call>\
             <tool_response>RESP_B</tool_response>final";
    let v = parse_chatlog_turns(s);

    assert_eq!(v.len(), 7, "expected 7 turns, got {}", v.len());

    let kinds: Vec<&str> = v
        .iter()
        .map(|m| match m.from {
            ShareGptFrom::Gpt => "gpt",
            ShareGptFrom::Tool => "tool",
            ShareGptFrom::Human => "human",
            ShareGptFrom::System => "system",
        })
        .collect();
    assert_eq!(
        kinds,
        vec!["gpt", "gpt", "tool", "gpt", "gpt", "tool", "gpt"]
    );

    assert_eq!(v[0].value, "think1");
    assert!(v[1].value.starts_with("<tool_call>") && v[1].value.contains("\"a\""));
    assert_eq!(v[2].value, "RESP_A");
    assert_eq!(v[3].value, "think2");
    assert!(v[4].value.starts_with("<tool_call>") && v[4].value.contains("\"b\""));
    assert_eq!(v[5].value, "RESP_B");
    assert_eq!(v[6].value, "final");
}
