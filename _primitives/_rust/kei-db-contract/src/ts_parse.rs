//! TS parser cube: shallow regex extraction of `type X = { ... }` and `interface X { ... }`.

use anyhow::{Context, Result};
use regex::Regex;
use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

/// One declared field on a TS type / interface.
#[derive(Debug, Clone, Serialize)]
pub struct TsField {
    pub name: String,
    pub ts_type: String,
    pub optional: bool,
}

/// One declared `type` alias or `interface` block.
#[derive(Debug, Clone, Serialize)]
pub struct TsType {
    pub name: String,
    pub fields: Vec<TsField>,
    pub source_file: String,
}

/// Walk a directory recursively, parse every `.ts` and `.tsx` file.
pub fn parse_ts_glob(roots: &[&Path]) -> Result<Vec<TsType>> {
    let mut out = Vec::new();
    for root in roots {
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(root).sort_by_file_name() {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
            if !matches!(ext, "ts" | "tsx") {
                continue;
            }
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("read {}", path.display()))?;
            let label = path.display().to_string();
            out.extend(extract_ts_types(&text, &label));
        }
    }
    Ok(out)
}

/// Extract every type/interface block from a single TS source string.
pub fn extract_ts_types(text: &str, source: &str) -> Vec<TsType> {
    let stripped = strip_line_comments(text);
    let mut out = Vec::new();
    let header_re = Regex::new(
        r"(?m)^(?:export\s+)?(?:type|interface)\s+([A-Z][A-Za-z0-9_]*)\s*(?:=\s*)?\{",
    )
    .expect("static regex");
    for m in header_re.captures_iter(&stripped) {
        let name = m.get(1).map(|x| x.as_str().to_string()).unwrap_or_default();
        let header_end = m.get(0).map(|x| x.end()).unwrap_or(0);
        let Some(body) = capture_balanced_braces(&stripped, header_end) else {
            continue;
        };
        let fields = parse_ts_fields(&body);
        out.push(TsType {
            name,
            fields,
            source_file: source.to_string(),
        });
    }
    out
}

fn strip_line_comments(text: &str) -> String {
    text.lines()
        .map(|line| match line.find("//") {
            Some(idx) => line[..idx].to_string(),
            None => line.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn capture_balanced_braces(text: &str, start: usize) -> Option<String> {
    let bytes = text.as_bytes();
    let mut depth = 1usize;
    let mut idx = start;
    while idx < bytes.len() {
        match bytes[idx] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(text[start..idx].to_string());
                }
            }
            _ => {}
        }
        idx += 1;
    }
    None
}

fn parse_ts_fields(body: &str) -> Vec<TsField> {
    let field_re = Regex::new(
        r"(?:readonly\s+)?([A-Za-z_][A-Za-z0-9_]*)(\?)?\s*:\s*([^;,\r\n]+?)\s*[;,\r\n]",
    )
    .expect("static regex");
    let padded = format!("{};", body);
    let mut out = Vec::new();
    for c in field_re.captures_iter(&padded) {
        let name = c[1].to_string();
        let optional = c.get(2).is_some();
        let ts_type = c[3].trim().to_string();
        if ts_type.is_empty() || ts_type == "{" {
            continue;
        }
        out.push(TsField {
            name,
            ts_type,
            optional,
        });
    }
    out
}
