//! `search` verb — FTS5 match over `fts_<table>`, JOIN back to entity
//! table, ORDER BY rank.
//!
//! Requires `EntitySchema.fts_columns` to be `Some`.
//!
//! Security: user input is wrapped in an FTS5 double-quoted phrase so
//! the FTS5 query grammar (`col:term`, `NEAR/5`, boolean ops, `*`,
//! parentheses) is treated as LITERAL TEXT. This is a pure keyword
//! search — attackers cannot address unindexed columns or craft
//! pathological scan expressions. Embedded `"` chars in the user query
//! are escaped per FTS5 grammar by doubling (`"" → "`).
//!
//! Tokenization guard: a query with ZERO searchable tokens (e.g. all
//! punctuation, only whitespace once trimmed) is rejected with
//! `InvalidInput` (exit 2) BEFORE reaching SQLite. This preserves the
//! documented exit-code contract — otherwise the porter/unicode61
//! tokenizer produces an empty token stream and FTS5 emits an opaque
//! `fts5: syntax error` that would propagate as `VerbError::Sqlite`
//! (exit 1).

use crate::error::VerbError;
use crate::schema::EntitySchema;
use crate::verbs::get::row_to_json;
use rusqlite::Connection;
use serde_json::{json, Value};

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 10_000;

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("search") {
        return Err(VerbError::VerbDisabled {
            verb: "search".into(),
            schema: schema.name.into(),
        });
    }
    if schema.fts_columns.is_none() {
        return Err(VerbError::InvalidInput(format!(
            "search: schema {} has no fts_columns configured",
            schema.name
        )));
    }
    let query = input
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or_else(|| VerbError::InvalidInput("search: missing `query` string".into()))?;
    if query.trim().is_empty() {
        return Err(VerbError::InvalidInput("search: query must be non-empty".into()));
    }
    if !has_searchable_token(query) {
        return Err(VerbError::InvalidInput(
            "search: query has no searchable tokens".into(),
        ));
    }
    let limit = clamp(input.get("limit").and_then(|v| v.as_i64()));
    let safe_query = fts5_quote(query);

    let cols: Vec<String> = schema.fields.iter().map(|f| format!("t.{}", f.name)).collect();
    let sql = format!(
        "SELECT {cols_sel} FROM fts_{table} f \
         JOIN {table} t ON t.id = f.{table}_id \
         WHERE fts_{table} MATCH ?1 ORDER BY rank LIMIT ?2",
        cols_sel = cols.join(","),
        table = schema.table
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params![safe_query, limit])?;
    let mut results: Vec<Value> = Vec::new();
    while let Some(r) = rows.next()? {
        results.push(row_to_json(schema, r)?);
    }
    Ok(json!({ "results": results }))
}

/// Wrap a user-supplied string as an FTS5 literal phrase. Doubles any
/// embedded `"` per FTS5 grammar. Result is safe to bind as the MATCH
/// argument and will match rows containing all of the literal tokens
/// in order.
fn fts5_quote(raw: &str) -> String {
    let escaped = raw.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

/// True if `raw` contains at least one character the FTS5 porter /
/// unicode61 tokenizer will emit as a token (alphabetic or numeric).
/// Punctuation- and whitespace-only queries produce zero tokens and
/// would trip an opaque `fts5: syntax error` at MATCH time.
fn has_searchable_token(raw: &str) -> bool {
    raw.chars().any(|c| c.is_alphanumeric())
}

fn clamp(raw: Option<i64>) -> i64 {
    match raw {
        Some(n) if n > 0 && n <= MAX_LIMIT => n,
        _ => DEFAULT_LIMIT,
    }
}

#[cfg(test)]
mod tests {
    use super::{fts5_quote, has_searchable_token};

    #[test]
    fn quote_basic() {
        assert_eq!(fts5_quote("refactor"), "\"refactor\"");
    }

    #[test]
    fn quote_escapes_dq() {
        assert_eq!(fts5_quote("has \"quote\""), "\"has \"\"quote\"\"\"");
    }

    #[test]
    fn quote_preserves_colons_and_ops() {
        // Injection attempt: `title:evil` — quoted phrase neutralizes
        // the column-prefix operator so the result searches for the
        // literal tokens `title:evil` across the configured columns.
        assert_eq!(fts5_quote("title:evil"), "\"title:evil\"");
    }

    #[test]
    fn has_token_accepts_alpha() {
        assert!(has_searchable_token("hello"));
        assert!(has_searchable_token("  hi!  "));
    }

    #[test]
    fn has_token_accepts_digits() {
        assert!(has_searchable_token("2026"));
    }

    #[test]
    fn has_token_rejects_punct_only() {
        assert!(!has_searchable_token("!@#$"));
        assert!(!has_searchable_token("..."));
        assert!(!has_searchable_token("---"));
    }

    #[test]
    fn has_token_accepts_unicode_alpha() {
        // Porter/unicode61 tokenises Cyrillic; our gate must too.
        assert!(has_searchable_token("привет"));
    }
}
