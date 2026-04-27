//! Output row — one hit per (category, chatlog file, line_no).
//!
//! Constructor Pattern: one struct, two serializers. CSV is emitted by hand
//! (no `csv` crate in the dependency list); JSONL uses `serde_json`.
//!
//! Fields are public and stable — this is the wire format the `report`
//! sub-command reads back from disk.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub category: String,
    pub chatlog_file: String,
    pub line_no: usize,
    pub timestamp: String, // ISO-ish string or mtime seconds
    pub quote: String,
    pub weight: f64,
}

/// CSV header — kept as a const so tests + report agree.
pub const CSV_HEADER: &str = "category,chatlog_file,line_no,timestamp,quote,weight";

/// CSV-escape per RFC 4180 + single-line enforcement. We replace newlines
/// with spaces BEFORE quote-wrapping so parse_csv's line-split assumption
/// holds. The original multi-line text is lossy-reduced for the CSV export;
/// use JSONL output if full fidelity matters.
fn csv_escape(s: &str) -> String {
    let singleline: String = s
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    let needs_quote = singleline.contains(',') || singleline.contains('"');
    let mut body = singleline.replace('"', "\"\"");
    if needs_quote {
        body.insert(0, '"');
        body.push('"');
    }
    body
}

/// Serialize one row to a single CSV line (no trailing newline).
pub fn to_csv(r: &Row) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_escape(&r.category),
        csv_escape(&r.chatlog_file),
        r.line_no,
        csv_escape(&r.timestamp),
        csv_escape(&r.quote),
        r.weight
    )
}

/// Serialize one row to JSONL (ends with newline inside `to_string`).
pub fn to_jsonl(r: &Row) -> Result<String> {
    serde_json::to_string(r).context("serialize row as JSON")
}

/// Parse a CSV body (header + rows) back into `Vec<Row>`.
/// Minimal RFC 4180 subset — no multi-line quoted fields (our quotes
/// never contain newlines because we stripped them at capture).
pub fn parse_csv(body: &str) -> Result<Vec<Row>> {
    let mut lines = body.lines();
    let Some(hdr) = lines.next() else {
        return Ok(Vec::new());
    };
    if hdr.trim() != CSV_HEADER {
        anyhow::bail!("csv header mismatch; got {hdr:?}, expected {CSV_HEADER:?}");
    }
    lines.enumerate().map(parse_row).collect()
}

fn parse_row((idx, line): (usize, &str)) -> Result<Row> {
    let fields = split_csv_line(line);
    if fields.len() != 6 {
        anyhow::bail!("csv line {} has {} fields, expected 6", idx + 2, fields.len());
    }
    Ok(Row {
        category: fields[0].clone(),
        chatlog_file: fields[1].clone(),
        line_no: fields[2].parse().context("line_no")?,
        timestamp: fields[3].clone(),
        quote: fields[4].clone(),
        weight: fields[5].parse().context("weight")?,
    })
}

/// Split a CSV line with RFC-4180 quote handling (single-line only).
fn split_csv_line(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_quote = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, in_quote) {
            ('"', true) if chars.peek() == Some(&'"') => {
                buf.push('"');
                chars.next();
            }
            ('"', _) => in_quote = !in_quote,
            (',', false) => {
                out.push(std::mem::take(&mut buf));
            }
            (ch, _) => buf.push(ch),
        }
    }
    out.push(buf);
    out
}
