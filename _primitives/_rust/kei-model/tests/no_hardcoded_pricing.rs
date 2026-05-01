//! RULE 0.4 guardrail: scan `src/**/*.rs` and reject any hardcoded pricing
//! literal. Pricing belongs in `data/models.toml`, never in Rust source.
//!
//! Two heuristic patterns:
//!   1. `\d+_per_mtok` — direct mention of a numeric rate.
//!   2. A digit run of 4+ within ~20 chars of the word "pricing" — covers
//!      structured-literal smuggling.

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

fn src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn collect_rs(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).expect("read src/").flatten() {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
    out
}

#[test]
fn no_numeric_per_mtok_literal() {
    let pat = Regex::new(r"\d+_per_mtok").unwrap();
    for f in collect_rs(&src_dir()) {
        let txt = fs::read_to_string(&f).unwrap();
        assert!(
            !pat.is_match(&txt),
            "hardcoded pricing literal in {} (matches \\d+_per_mtok)",
            f.display()
        );
    }
}

#[test]
fn no_digit_run_near_pricing_word() {
    // Match "pricing" (any case) with a 4+ digit run within 30 chars on
    // either side. Anchors on the literal word, ignores test fixtures
    // (which live under tests/, not src/).
    let pat = Regex::new(r#"(?i)pricing[^"]{0,30}\d{4,}|\d{4,}[^"]{0,30}pricing"#).unwrap();
    for f in collect_rs(&src_dir()) {
        let txt = fs::read_to_string(&f).unwrap();
        if let Some(m) = pat.find(&txt) {
            panic!(
                "suspected hardcoded pricing in {} near match: {:?}",
                f.display(),
                m.as_str()
            );
        }
    }
}
