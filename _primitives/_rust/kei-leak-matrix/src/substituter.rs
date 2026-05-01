//! Substituter — applies substitute-severity rules to a string.
//!
//! Used by `kei-leak-matrix substitute --scope <s>` and by upstream hooks
//! (e.g. sync-public.sh) before any block check is run.

use crate::matrix::{Matrix, Scope, Severity};
use anyhow::Result;
use std::io::{Read, Write};

/// Apply every substitute-severity rule whose scope matches `requested`.
/// Rules without a `substitute_with` field are skipped (defensive).
pub fn substitute(matrix: &Matrix, content: &str, requested: Scope) -> String {
    let mut out = content.to_string();
    for rule in &matrix.rules {
        if !matches!(rule.severity, Severity::Substitute) { continue; }
        if !rule.matches_scope(requested) { continue; }
        let Some(ref repl) = rule.substitute_with else { continue };
        out = rule.regex.replace_all(&out, repl.as_str()).into_owned();
    }
    out
}

/// Handler: read stdin, write substituted content to stdout. Exit 0.
pub fn cmd_substitute(matrix: &Matrix, scope: Scope) -> Result<i32> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let out = substitute(matrix, &buf, scope);
    std::io::stdout().write_all(out.as_bytes())?;
    Ok(0)
}
