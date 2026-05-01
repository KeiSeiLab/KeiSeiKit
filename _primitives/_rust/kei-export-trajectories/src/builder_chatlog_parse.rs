//! Multi-turn chatlog parser. Splits raw chatlog on `<tool_call>` /
//! `<tool_response>` XML and emits typed `ShareGptMessage` turns.
//! Legacy fallback (no markers): single `From::Gpt` turn wrapped in
//! `<think>\n</think>\n` to keep the pre-Hermes golden fixture stable.

use crate::sharegpt::{From as ShareGptFrom, ShareGptMessage};

const TOOL_CALL_OPEN: &str = "<tool_call>";
const TOOL_CALL_CLOSE: &str = "</tool_call>";
const TOOL_RESP_OPEN: &str = "<tool_response>";
const TOOL_RESP_CLOSE: &str = "</tool_response>";

/// Parse a chatlog into ShareGPT turns.
///
/// Returns:
/// - if `text` contains NO `<tool_call>` and NO `<tool_response>`
///   markers → exactly one `From::Gpt` turn wrapping body in
///   `<think>\n</think>\n` (legacy shape, golden test stable);
/// - otherwise → an ordered sequence where:
///   - `<tool_call>...</tool_call>` block → `From::Gpt`, value =
///     entire block including the markers (Hermes XML convention);
///   - `<tool_response>...</tool_response>` block → `From::Tool`,
///     value = INNER text only (Hermes spec: tool turns carry plain
///     payload, not wrapped XML);
///   - plain text between blocks → `From::Gpt` with that text verbatim.
///   Whitespace-only segments are dropped.
pub fn parse_chatlog_turns(text: &str) -> Vec<ShareGptMessage> {
    if !has_markers(text) {
        return legacy_single_turn(text);
    }
    let segments = split_into_segments(text);
    segments
        .into_iter()
        .filter_map(segment_to_message)
        .collect()
}

fn has_markers(text: &str) -> bool {
    text.contains(TOOL_CALL_OPEN) || text.contains(TOOL_RESP_OPEN)
}

fn legacy_single_turn(body: &str) -> Vec<ShareGptMessage> {
    vec![ShareGptMessage {
        from: ShareGptFrom::Gpt,
        value: format!("<think>\n</think>\n{body}"),
    }]
}

/// One classified slice of the chatlog.
#[derive(Debug)]
enum Segment<'a> {
    Plain(&'a str),
    ToolCall(&'a str),     // entire `<tool_call>...</tool_call>`
    ToolResponse(&'a str), // INNER payload only
}

fn split_into_segments(text: &str) -> Vec<Segment<'_>> {
    let mut out: Vec<Segment<'_>> = Vec::new();
    let mut cursor = 0usize;
    while cursor < text.len() {
        cursor = step_segment(text, cursor, &mut out);
    }
    out
}

/// Advance one segment from `cursor` and return the new cursor.
/// On end-of-text returns `text.len()` so the caller's loop terminates.
fn step_segment<'a>(text: &'a str, cursor: usize, out: &mut Vec<Segment<'a>>) -> usize {
    let rest = &text[cursor..];
    match next_marker(rest) {
        None => {
            out.push(Segment::Plain(rest));
            text.len()
        }
        Some((rel_offset, kind)) => {
            if rel_offset > 0 {
                out.push(Segment::Plain(&rest[..rel_offset]));
            }
            let block_start = cursor + rel_offset;
            match kind {
                MarkerKind::Call => take_call_block(text, block_start, out),
                MarkerKind::Resp => take_resp_block(text, block_start, out),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MarkerKind {
    Call,
    Resp,
}

/// Find the earliest opening marker in `rest`. Returns `(relative_offset, kind)`.
fn next_marker(rest: &str) -> Option<(usize, MarkerKind)> {
    let c = rest.find(TOOL_CALL_OPEN);
    let r = rest.find(TOOL_RESP_OPEN);
    match (c, r) {
        (None, None) => None,
        (Some(i), None) => Some((i, MarkerKind::Call)),
        (None, Some(j)) => Some((j, MarkerKind::Resp)),
        (Some(i), Some(j)) if i < j => Some((i, MarkerKind::Call)),
        (Some(_), Some(j)) => Some((j, MarkerKind::Resp)),
    }
}

/// Consume one `<tool_call>...</tool_call>` block starting at absolute
/// `start`. Pushes a `Segment::ToolCall` carrying the full block (markers
/// included). Returns absolute cursor position after the block. If close
/// marker is missing, treats the rest of `text` as the block body.
fn take_call_block<'a>(text: &'a str, start: usize, out: &mut Vec<Segment<'a>>) -> usize {
    let after_open = start + TOOL_CALL_OPEN.len();
    let close_rel = text[after_open..].find(TOOL_CALL_CLOSE);
    match close_rel {
        Some(rel) => {
            let end = after_open + rel + TOOL_CALL_CLOSE.len();
            out.push(Segment::ToolCall(&text[start..end]));
            end
        }
        None => {
            out.push(Segment::ToolCall(&text[start..]));
            text.len()
        }
    }
}

/// Consume one `<tool_response>...</tool_response>` block starting at
/// absolute `start`. Pushes `Segment::ToolResponse` carrying the INNER
/// payload only.
fn take_resp_block<'a>(text: &'a str, start: usize, out: &mut Vec<Segment<'a>>) -> usize {
    let after_open = start + TOOL_RESP_OPEN.len();
    let close_rel = text[after_open..].find(TOOL_RESP_CLOSE);
    match close_rel {
        Some(rel) => {
            let inner = &text[after_open..after_open + rel];
            out.push(Segment::ToolResponse(inner));
            after_open + rel + TOOL_RESP_CLOSE.len()
        }
        None => {
            out.push(Segment::ToolResponse(&text[after_open..]));
            text.len()
        }
    }
}

fn segment_to_message(seg: Segment<'_>) -> Option<ShareGptMessage> {
    let (from, raw) = match seg {
        Segment::Plain(s) => (ShareGptFrom::Gpt, s.to_string()),
        Segment::ToolCall(s) => (ShareGptFrom::Gpt, s.to_string()),
        Segment::ToolResponse(s) => (ShareGptFrom::Tool, s.to_string()),
    };
    if raw.trim().is_empty() {
        return None;
    }
    Some(ShareGptMessage { from, value: raw })
}

// Unit tests live in `builder_chatlog_parse_tests.rs` (registered from
// `lib.rs`) to keep this parser file inside the 200-LOC Constructor budget.
