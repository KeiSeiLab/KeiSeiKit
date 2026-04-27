//! Shared fixtures for `tests/eval.rs`.
//!
//! Lives in a subdirectory so Cargo doesn't compile it as its own test
//! binary. The parent test (`tests/eval.rs`) declares this with:
//!
//! ```ignore
//! #[path = "eval_helpers/mod.rs"]
//! mod eval_helpers;
//! ```
//!
//! The `#[path]` form is used because `super::*` from here needs to resolve
//! to the eval-types already wired up in the test root via `#[path]`
//! includes — keeping the wire-up chain confined to the test root.

use super::eval::{GoldRow, PerCategoryMetric};
use super::eval_predict::CategoryPredictor;
use std::collections::HashMap;

/// Text-to-category lookup predictor — used in every eval test.
pub struct MockPredictor {
    table: HashMap<String, String>,
}

impl MockPredictor {
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let mut table = HashMap::new();
        for (text, cat) in pairs {
            table.insert((*text).to_string(), (*cat).to_string());
        }
        Self { table }
    }

    /// Mock that predicts each gold row's true category — a "perfect"
    /// classifier. Used as the firmware stub in the tiny-gold-set test.
    pub fn perfect_on(gold: &[GoldRow]) -> Self {
        let table = gold
            .iter()
            .map(|g| (g.text.clone(), g.category.clone()))
            .collect();
        Self { table }
    }
}

impl CategoryPredictor for MockPredictor {
    fn predict(&self, text: &str) -> String {
        self.table
            .get(text)
            .cloned()
            .unwrap_or_else(|| "uncategorized".to_string())
    }
}

/// Shortcut for building `GoldRow` instances.
pub fn gold_row(text: &str, cat: &str) -> GoldRow {
    GoldRow {
        category: cat.to_string(),
        text: text.to_string(),
    }
}

/// 6-row fixture used by `eval_from_tiny_gold_set`:
/// 2 × conservative-framing, 2 × paradigm-slippage, 2 × repeat-signal.
pub fn make_gold_set() -> Vec<GoldRow> {
    vec![
        gold_row("cf1", "conservative-framing"),
        gold_row("cf2", "conservative-framing"),
        gold_row("ps1", "paradigm-slippage"),
        gold_row("ps2", "paradigm-slippage"),
        gold_row("rs1", "repeat-signal"),
        gold_row("rs2", "repeat-signal"),
    ]
}

/// Parsed CSV row: (model, category, precision, recall, f1, support).
pub type ParsedRow = (String, String, f64, f64, f64, usize);

/// Parse the CSV produced by `write_csv`. Minimal RFC-4180 subset:
/// every value is numeric or a bare identifier (no commas, no quotes).
pub fn parse_csv_body(body: &str) -> Vec<ParsedRow> {
    body.lines()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .map(parse_one_row)
        .collect()
}

fn parse_one_row(line: &str) -> ParsedRow {
    let f: Vec<&str> = line.split(',').collect();
    assert_eq!(f.len(), 6, "unexpected csv row: {line:?}");
    (
        f[0].to_string(),
        f[1].to_string(),
        f[2].parse().expect("precision"),
        f[3].parse().expect("recall"),
        f[4].parse().expect("f1"),
        f[5].parse().expect("support"),
    )
}

/// Assert that for every expected `PerCategoryMetric`, the CSV contains
/// a row with matching numbers under `model`.
pub fn compare_model_rows(
    parsed: &[ParsedRow],
    model: &str,
    expected: &[PerCategoryMetric],
) {
    for exp in expected {
        let row = find_row(parsed, model, &exp.category);
        assert!((row.2 - exp.precision).abs() < 1e-6, "precision {}", exp.category);
        assert!((row.3 - exp.recall).abs() < 1e-6, "recall {}", exp.category);
        assert!((row.4 - exp.f1).abs() < 1e-6, "f1 {}", exp.category);
        assert_eq!(row.5, exp.support, "support {}", exp.category);
    }
}

fn find_row<'a>(parsed: &'a [ParsedRow], model: &str, cat: &str) -> &'a ParsedRow {
    parsed
        .iter()
        .find(|r| r.0 == model && r.1 == cat)
        .unwrap_or_else(|| panic!("csv missing {model}/{cat}"))
}

/// (truth, predicted) key for a `HashMap<(String,String), usize>` hit.
pub fn confusion_key(truth: &str, pred: &str) -> (String, String) {
    (truth.to_string(), pred.to_string())
}
