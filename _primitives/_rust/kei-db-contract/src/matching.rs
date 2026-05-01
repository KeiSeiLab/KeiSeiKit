//! Pair SQL tables with TS types using name heuristics:
//! `users` ≈ `User`, `magic_tokens` ≈ `MagicToken`, `auth_sessions` ≈ `AuthSession`.

use crate::sql_parse::SqlTable;
use crate::ts_parse::TsType;

/// One paired (or orphan) result from the matching step.
#[derive(Debug, Clone)]
pub struct Pair<'a> {
    pub table: Option<&'a SqlTable>,
    pub ts_type: Option<&'a TsType>,
}

/// Pair every SQL table with at most one TS type and vice versa.
pub fn pair_tables_with_types<'a>(
    tables: &'a [SqlTable],
    ts_types: &'a [TsType],
) -> Vec<Pair<'a>> {
    let mut consumed_ts: Vec<bool> = vec![false; ts_types.len()];
    let mut pairs: Vec<Pair<'a>> = Vec::new();
    for tbl in tables {
        pairs.push(claim_one_table(tbl, ts_types, &mut consumed_ts));
    }
    for (i, ts) in ts_types.iter().enumerate() {
        if !consumed_ts[i] {
            pairs.push(Pair {
                table: None,
                ts_type: Some(ts),
            });
        }
    }
    pairs
}

fn claim_one_table<'a>(
    tbl: &'a SqlTable,
    ts_types: &'a [TsType],
    consumed_ts: &mut [bool],
) -> Pair<'a> {
    let canonical = canonicalize_table(&tbl.name);
    for (i, ts) in ts_types.iter().enumerate() {
        if consumed_ts[i] {
            continue;
        }
        if canonicalize_type(&ts.name) == canonical {
            consumed_ts[i] = true;
            return Pair {
                table: Some(tbl),
                ts_type: Some(&ts_types[i]),
            };
        }
    }
    Pair {
        table: Some(tbl),
        ts_type: None,
    }
}

/// Strip plural -s, normalize separators, lowercase. `magic_tokens` → `magictoken`.
pub fn canonicalize_table(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    let no_seps = cleaned.replace(['_', '-'], "");
    let lower = no_seps.to_ascii_lowercase();
    strip_trailing_s(&lower)
}

/// Lowercase + strip trailing -s. `MagicToken` → `magictoken`.
pub fn canonicalize_type(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    strip_trailing_s(&lower)
}

fn strip_trailing_s(s: &str) -> String {
    if s.ends_with("ies") && s.len() > 3 {
        let mut out = s[..s.len() - 3].to_string();
        out.push('y');
        return out;
    }
    if s.ends_with('s') && s.len() > 1 {
        return s[..s.len() - 1].to_string();
    }
    s.to_string()
}
