//! Gold-set JSONL parser.
//!
//! One file, one job: read the hand-labelled training set, drop anything
//! whose `quality != "gold"` (silver is noisy per spec), return clean
//! `GoldRow`s for the eval pipeline.
//!
//! Tolerant parse strategy: we deserialize into `serde_json::Value` rather
//! than a strict struct so upstream shape changes (extra columns, different
//! source tags) don't break the eval. Only the 4 fields named in the spec
//! are consulted: `category`, `text`, `source`, `quality`.
//!
//! One bad line is NEVER a fatal error — the whole file gets skipped only
//! if `fs::read_to_string` itself fails.

use crate::eval::GoldRow;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Load gold-quality rows from a JSONL file.
///
/// Rows with `quality != "gold"` are silently skipped. Rows missing the
/// `category` or `text` field are also skipped — we cannot evaluate a
/// classifier on an unlabelled example. Line-level JSON errors are
/// tolerated (one bad line does not abort the file).
pub fn load_gold_rows(path: &Path) -> Result<Vec<GoldRow>> {
    let body = fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    Ok(parse_jsonl_body(&body))
}

/// Pure-function variant — public inside the crate for test injection.
pub(crate) fn parse_jsonl_body(body: &str) -> Vec<GoldRow> {
    body.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(parse_one_line)
        .collect()
}

/// Parse a single JSONL line. Returns `None` if:
///   * JSON is malformed,
///   * `quality` is absent or not "gold",
///   * `category` or `text` is absent / non-string.
fn parse_one_line(raw: &str) -> Option<GoldRow> {
    let v: Value = serde_json::from_str(raw).ok()?;
    if !is_gold_quality(&v) {
        return None;
    }
    let category = v.get("category").and_then(Value::as_str)?.to_string();
    let text = v.get("text").and_then(Value::as_str)?.to_string();
    if category.is_empty() || text.is_empty() {
        return None;
    }
    Some(GoldRow { category, text })
}

/// Gold filter: spec says "skip rows with quality != gold (silver is noisy)".
/// Missing quality field → not gold, drop.
fn is_gold_quality(v: &Value) -> bool {
    v.get("quality").and_then(Value::as_str) == Some("gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_gold_drops_silver_and_broken() {
        let body = concat!(
            r#"{"category":"a","text":"hello","source":"x","quality":"gold"}"#,
            "\n",
            r#"{"category":"b","text":"world","source":"x","quality":"silver"}"#,
            "\n",
            "not valid json",
            "\n",
            r#"{"category":"c","text":"","quality":"gold"}"#,
            "\n",
            r#"{"text":"no cat","quality":"gold"}"#,
            "\n",
            r#"{"category":"d","text":"kept","quality":"gold"}"#,
            "\n",
        );
        let rows = parse_jsonl_body(body);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].category, "a");
        assert_eq!(rows[0].text, "hello");
        assert_eq!(rows[1].category, "d");
    }

    #[test]
    fn empty_body_returns_empty_vec() {
        assert!(parse_jsonl_body("").is_empty());
        assert!(parse_jsonl_body("\n\n\n").is_empty());
    }
}
