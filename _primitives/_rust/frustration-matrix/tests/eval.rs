//! Integration tests for the `eval` subcommand.
//!
//! Constructor Pattern: each test = one scenario, ≤ 30 LOC body. Shared
//! fixtures + helpers live in `tests/eval_helpers/mod.rs` — subdirectory
//! so Cargo does not compile them as a separate test binary.
//!
//! We load source modules via `#[path = "../src/X.rs"]` (matches existing
//! `tests/integration.rs`). The `CategoryPredictor` trait lets each test
//! wire a `MockPredictor` — Z1/Z2 need not be complete.

#[path = "../src/categories.rs"]
mod categories;
#[path = "../src/classifier.rs"]
mod classifier;
#[path = "../src/eval.rs"]
mod eval;
#[path = "../src/eval_gold.rs"]
mod eval_gold;
#[path = "../src/eval_metrics.rs"]
mod eval_metrics;
#[path = "../src/eval_predict.rs"]
mod eval_predict;
#[path = "../src/eval_report.rs"]
mod eval_report;
#[path = "../src/firmware.rs"]
mod firmware;
#[path = "../src/firmware_corpus.rs"]
mod firmware_corpus;
#[path = "../src/firmware_ngram.rs"]
mod firmware_ngram;
#[path = "../src/jsonl.rs"]
mod jsonl;
mod eval_helpers;

use std::fs;

use eval::run_with_predictors;
use eval_helpers::{
    compare_model_rows, confusion_key, gold_row, make_gold_set, parse_csv_body, MockPredictor,
};
use eval_metrics::{build_confusion, compute_metrics};
use eval_report::write_csv;
use tempfile::tempdir;

// ---------- 1. eval_from_tiny_gold_set ----------
#[test]
fn eval_from_tiny_gold_set() {
    let gold = make_gold_set();
    let regex_stub = MockPredictor::from_pairs(&[
        ("cf1", "conservative-framing"),
        ("cf2", "uncategorized"),
        ("ps1", "paradigm-slippage"),
        ("ps2", "conservative-framing"),
        ("rs1", "repeat-signal"),
        ("rs2", "repeat-signal"),
    ]);
    let firmware_stub = MockPredictor::perfect_on(&gold);
    let report = run_with_predictors(&gold, &regex_stub, &firmware_stub);
    assert_eq!(report.total_gold_rows, 6);
    assert!((report.regex_metrics.accuracy - 4.0 / 6.0).abs() < 1e-9);
    assert!((report.firmware_metrics.accuracy - 1.0).abs() < 1e-9);
    for m in &report.firmware_metrics.per_category {
        if m.support > 0 {
            assert!((m.f1 - 1.0).abs() < 1e-9);
        }
    }
    let cf_miss = confusion_key("conservative-framing", "uncategorized");
    let ps_to_cf = confusion_key("paradigm-slippage", "conservative-framing");
    assert_eq!(report.confusion_regex.get(&cf_miss), Some(&1));
    assert_eq!(report.confusion_regex.get(&ps_to_cf), Some(&1));
}

// ---------- 2. per_category_metrics_handle_zero_support ----------
#[test]
fn per_category_metrics_handle_zero_support() {
    let truth = ["a", "a", "a"];
    let pred: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let metrics = compute_metrics(&truth, &pred);
    let b = metrics.per_category.iter().find(|m| m.category == "b").unwrap();
    assert_eq!(b.support, 0);
    assert!(b.precision.abs() < 1e-9);
    assert!(b.recall.abs() < 1e-9);
    assert!(b.f1.abs() < 1e-9);
    let a = metrics.per_category.iter().find(|m| m.category == "a").unwrap();
    assert_eq!(a.support, 3);
    assert!((a.precision - 1.0).abs() < 1e-9);
    assert!((a.recall - 1.0 / 3.0).abs() < 1e-9);
    let expected_f1 = 2.0 * 1.0 * (1.0 / 3.0) / (1.0 + 1.0 / 3.0);
    assert!((a.f1 - expected_f1).abs() < 1e-9);
}

// ---------- 3. confusion_matrix_correct_counts ----------
#[test]
fn confusion_matrix_correct_counts() {
    let truth = [
        "conservative-framing",
        "conservative-framing",
        "conservative-framing",
        "uncategorized",
    ];
    let pred: Vec<String> = vec![
        "conservative-framing".into(),
        "conservative-framing".into(),
        "paradigm-slippage".into(),
        "uncategorized".into(),
    ];
    let conf = build_confusion(&truth, &pred);
    let cf_cf = confusion_key("conservative-framing", "conservative-framing");
    let cf_ps = confusion_key("conservative-framing", "paradigm-slippage");
    let un_un = confusion_key("uncategorized", "uncategorized");
    assert_eq!(conf.get(&cf_cf), Some(&2));
    assert_eq!(conf.get(&cf_ps), Some(&1));
    assert_eq!(conf.get(&un_un), Some(&1));
    assert_eq!(conf.values().sum::<usize>(), 4);
}

// ---------- 4. output_csv_roundtrip ----------
#[test]
fn output_csv_roundtrip() {
    let gold = vec![
        gold_row("x1", "a"),
        gold_row("x2", "a"),
        gold_row("x3", "b"),
    ];
    let stub = MockPredictor::from_pairs(&[("x1", "a"), ("x2", "b"), ("x3", "b")]);
    let report = run_with_predictors(&gold, &stub, &stub);
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("report.csv");
    write_csv(&path, &report).expect("write csv");
    let body = fs::read_to_string(&path).expect("read csv");
    assert!(body.starts_with("model,category,precision,recall,f1,support\n"));
    let parsed = parse_csv_body(&body);
    compare_model_rows(&parsed, "regex", &report.regex_metrics.per_category);
    compare_model_rows(&parsed, "firmware", &report.firmware_metrics.per_category);
}
