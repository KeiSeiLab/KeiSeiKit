//! Inline unit tests for `anthropic_invoker.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays under
//! the 200-LOC ceiling now that v0.40 added `parse_usage` + three
//! token-usage assertions.

use super::*;
use crate::tool::types::ToolResult;

#[test]
fn parse_text_only_turn() {
    let raw = json!({"stop_reason":"end_turn","content":[{"type":"text","text":"hello"}]});
    let turn = parse_turn(&raw).unwrap();
    assert_eq!(turn.stop_reason, "end_turn");
    assert!(matches!(&turn.content[0], ContentBlock::Text(t) if t == "hello"));
}

#[test]
fn parse_tool_use_turn() {
    let raw = json!({"stop_reason":"tool_use","content":[
        {"type":"text","text":"x"},
        {"type":"tool_use","id":"tu_1","name":"read","input":{"path":"/x"}}]});
    let turn = parse_turn(&raw).unwrap();
    assert_eq!(turn.stop_reason, "tool_use");
    assert!(matches!(&turn.content[1], ContentBlock::ToolUse(c) if c.name == "read"));
}

#[test]
fn unknown_blocks_are_filtered() {
    let raw = json!({"stop_reason":"end_turn","content":[
        {"type":"text","text":"hi"}, {"type":"future_kind"}]});
    assert_eq!(parse_turn(&raw).unwrap().content.len(), 1);
}

#[test]
fn render_tool_result_message() {
    let msg = ConversationMessage::Tool(vec![ToolResult::ok("tu_1", "data")]);
    let out = render_one(&msg);
    assert_eq!(out["role"], "user");
    assert_eq!(out["content"][0]["tool_use_id"], "tu_1");
}

#[test]
fn render_assistant_blocks_round_trip() {
    let blocks = vec![
        ContentBlock::Text("a".into()),
        ContentBlock::ToolUse(ToolCall {
            id: "tu".into(),
            name: "read".into(),
            input: json!({"p": 1}),
        }),
    ];
    let v = render_assistant_blocks(&blocks);
    assert_eq!(v[1]["type"], "tool_use");
    assert_eq!(v[1]["name"], "read");
}

#[test]
fn parse_usage_extracts_token_counts() {
    let raw = json!({
        "stop_reason": "end_turn",
        "content": [{"type": "text", "text": "hi"}],
        "usage": {"input_tokens": 1234, "output_tokens": 567},
    });
    let turn = parse_turn(&raw).unwrap();
    let usage = turn.usage.expect("usage present");
    assert_eq!(usage.input_tokens, 1234);
    assert_eq!(usage.output_tokens, 567);
}

#[test]
fn parse_usage_absent_yields_none() {
    let raw = json!({
        "stop_reason": "end_turn",
        "content": [{"type": "text", "text": "hi"}],
    });
    assert!(parse_turn(&raw).unwrap().usage.is_none());
}

#[test]
fn parse_usage_zero_zero_yields_none() {
    // Both zero is treated as absence — keeps spurious zero-token rows
    // out of the `top_provider_model` query.
    let raw = json!({
        "stop_reason": "end_turn",
        "content": [{"type": "text", "text": "hi"}],
        "usage": {"input_tokens": 0, "output_tokens": 0},
    });
    assert!(parse_turn(&raw).unwrap().usage.is_none());
}
