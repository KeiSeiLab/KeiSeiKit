//! CLI command bodies for artifact CRUD (emit / get / list / chain).
//!
//! Constructor Pattern: one file for the read/write-artifact commands,
//! kept separate from main.rs so the binary file stays <200 LOC.
//! Each public `cmd_*` fn < 30 LOC.

use kei_artifact::artifact::{chain, emit, get, list, ArtifactFilter};
use kei_artifact::Store;
use std::path::Path;

pub fn cmd_emit(
    store: &Store,
    schema: &str,
    from: &str,
    content_path: &Path,
    meta: &[String],
    parent: Option<&str>,
) -> anyhow::Result<()> {
    let bytes = std::fs::read(content_path)?;
    let meta_json = if meta.is_empty() { None } else { Some(encode_meta(meta)?) };
    let id = emit(store, schema, from, &bytes, meta_json.as_deref(), parent)?;
    println!("{id}");
    Ok(())
}

fn encode_meta(kvs: &[String]) -> anyhow::Result<String> {
    let mut obj = serde_json::Map::new();
    for kv in kvs {
        let (k, v) = kv
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("--meta expects key=value: {kv}"))?;
        obj.insert(k.to_string(), serde_json::Value::String(v.to_string()));
    }
    Ok(serde_json::Value::Object(obj).to_string())
}

pub fn cmd_get(store: &Store, id: &str, format: &str) -> anyhow::Result<()> {
    let a = get(store, id)?.ok_or_else(|| anyhow::anyhow!("artifact not found: {id}"))?;
    match format {
        "json" => print_json(&a)?,
        "md" | "typed" => print_typed(&a)?,
        other => return Err(anyhow::anyhow!("unknown format '{other}'")),
    }
    Ok(())
}

fn print_json(a: &kei_artifact::Artifact) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(a)?);
    Ok(())
}

fn print_typed(a: &kei_artifact::Artifact) -> anyhow::Result<()> {
    let text = std::str::from_utf8(&a.content).unwrap_or("<binary>");
    println!("# artifact {} (schema={}, from={})", a.id, a.schema_name, a.source_agent);
    println!("{text}");
    Ok(())
}

pub fn cmd_list(
    store: &Store,
    schema: Option<&str>,
    from: Option<&str>,
    since: Option<&str>,
) -> anyhow::Result<()> {
    let filter = ArtifactFilter {
        schema_name: schema.map(str::to_string),
        source_agent: from.map(str::to_string),
        since: since.and_then(parse_since),
    };
    for a in list(store, &filter)? {
        println!("{}\t{}\t{}\t{}", a.id, a.schema_name, a.source_agent, a.created_at);
    }
    Ok(())
}

fn parse_since(s: &str) -> Option<i64> {
    // Accept raw epoch ints, or "1d" / "2h" / "30m" shorthands.
    if let Ok(n) = s.parse::<i64>() {
        return Some(n);
    }
    let (num, unit) = s.split_at(s.find(|c: char| !c.is_ascii_digit())?);
    let n: i64 = num.parse().ok()?;
    let secs = match unit {
        "s" => n,
        "m" => n * 60,
        "h" => n * 3600,
        "d" => n * 86400,
        _ => return None,
    };
    Some(chrono::Utc::now().timestamp() - secs)
}

pub fn cmd_chain(store: &Store, id: &str) -> anyhow::Result<()> {
    for a in chain(store, id)? {
        println!("{}\t{}\t{}", a.id, a.schema_name, a.source_agent);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_meta_single_pair() {
        let s = encode_meta(&["role=architect".to_string()]).unwrap();
        assert!(s.contains("\"role\""));
        assert!(s.contains("\"architect\""));
    }

    #[test]
    fn encode_meta_rejects_missing_equals() {
        let err = encode_meta(&["no-equals".to_string()]).unwrap_err();
        assert!(format!("{err}").contains("no-equals"));
    }

    #[test]
    fn parse_since_accepts_raw_epoch() {
        assert_eq!(parse_since("1700000000"), Some(1700000000));
    }

    #[test]
    fn parse_since_rejects_bad_unit() {
        assert!(parse_since("5y").is_none());
    }
}
