//! `quality::constructor-pattern` — walks the run dir, asserts every `.rs`
//! file ≤ 200 LOC and every top-level `fn` ≤ 30 LOC.

use crate::capability::*;
use std::path::Path;
use walkdir::WalkDir;

pub struct ConstructorPattern;

const FILE_LOC_LIMIT: usize = 200;
const FN_LOC_LIMIT: usize = 30;

impl Capability for ConstructorPattern {
    fn name(&self) -> &'static str {
        "quality::constructor-pattern"
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let root = ctx.run_dir();
        let mut violations: Vec<String> = Vec::new();
        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
            .filter(|e| !is_ignored(e.path()))
        {
            check_file(entry.path(), &mut violations);
        }
        if violations.is_empty() {
            VerifyResult::Pass
        } else {
            VerifyResult::Fail {
                reason: format!("{} constructor-pattern violation(s)", violations.len()),
                detail: Some(violations.join("\n")),
            }
        }
    }
}

fn is_ignored(p: &Path) -> bool {
    p.components()
        .any(|c| matches!(c.as_os_str().to_str(), Some("target") | Some(".git")))
}

fn check_file(path: &Path, out: &mut Vec<String>) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return,
    };
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() > FILE_LOC_LIMIT {
        out.push(format!(
            "{}: {} LOC > {}",
            path.display(),
            lines.len(),
            FILE_LOC_LIMIT
        ));
    }
    for (name, n) in scan_fn_lengths(&lines) {
        if n > FN_LOC_LIMIT {
            out.push(format!("{} fn `{name}`: {n} LOC > {FN_LOC_LIMIT}", path.display()));
        }
    }
}

/// Extract `(fn_name, line_count)` for top-level `fn` definitions by tracking
/// brace depth. Best-effort — approximate for nested fns but adequate here.
fn scan_fn_lengths(lines: &[&str]) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let mut cur: Option<(String, usize, i32)> = None;
    for line in lines {
        if cur.is_none() {
            if let Some(name) = parse_fn_name(line) {
                let opens = line.matches('{').count() as i32 - line.matches('}').count() as i32;
                if opens > 0 {
                    cur = Some((name, 1, opens));
                    continue;
                }
            }
        } else if let Some((name, count, d)) = cur.as_mut() {
            *count += 1;
            *d += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            if *d <= 0 {
                out.push((name.clone(), *count));
                cur = None;
            }
        }
    }
    out
}

fn parse_fn_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
    let rest = rest.strip_prefix("async ").unwrap_or(rest);
    let rest = rest.strip_prefix("const ").unwrap_or(rest);
    let rest = rest.strip_prefix("unsafe ").unwrap_or(rest);
    let rest = rest.strip_prefix("fn ")?;
    let end = rest.find(['(', '<', ' ']).unwrap_or(rest.len());
    Some(rest[..end].to_string())
}
