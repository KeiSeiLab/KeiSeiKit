//! Eval — compare regex-based (v1) vs firmware-based (v2) classification on
//! a hand-labelled gold set. Reports per-category precision / recall / f1,
//! overall accuracy and macro-f1, plus two confusion matrices.
//!
//! This module consumes APIs from firmware.rs (Z1) and classifier.rs (Z2).
//! If those modules have different method names at orchestrator-merge time,
//! update the call sites here — the eval LOGIC is independent of the
//! internal firmware representation.
//!
//! Constructor Pattern: this file holds only the public types + the
//! `evaluate` orchestrator. Helpers live in sibling cubes:
//!
//!   * `eval_gold`    — parse labelled JSONL, filter quality=gold
//!   * `eval_predict` — `CategoryPredictor` trait + regex / firmware impls
//!   * `eval_metrics` — pure precision / recall / f1 math
//!   * `eval_report`  — CSV write + stdout summary
//!
//! Purity: every mathematical step in eval_metrics is a pure function of
//! two integer vectors (true + predicted). Predictors are behind a trait
//! so tests can inject `MockClassifier` without Z1/Z2 on disk.

use crate::categories::compile_all;
use crate::classifier::Classifier;
use crate::eval_gold::load_gold_rows;
use crate::eval_metrics::{build_confusion, compute_metrics};
use crate::eval_predict::{
    predict_all, CategoryPredictor, FirmwarePredictor, RegexPredictor,
};
use crate::eval_report::{print_summary, write_csv};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

/// CLI input bundle — from `main.rs` eval subcommand.
pub struct EvalInput {
    pub gold_jsonl: PathBuf,
    pub model_dir: PathBuf,
    pub output_csv: PathBuf,
}

/// Full report produced by `evaluate`.
pub struct EvalReport {
    pub total_gold_rows: usize,
    pub regex_metrics: Metrics,
    pub firmware_metrics: Metrics,
    pub confusion_regex: HashMap<(String, String), usize>,
    pub confusion_firmware: HashMap<(String, String), usize>,
}

/// Overall + per-category metrics for one classifier.
pub struct Metrics {
    pub accuracy: f64,
    pub per_category: Vec<PerCategoryMetric>,
}

/// Per-category precision / recall / f1 / support, sklearn convention.
pub struct PerCategoryMetric {
    pub category: String,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub support: usize,
}

/// Run the full eval pipeline: load gold, run both classifiers, compute
/// metrics, write CSV, print summary.
///
/// This is the ONLY function main.rs calls; all heavy lifting is delegated
/// to sibling cubes. Kept under 30 LOC per Constructor Pattern rule.
pub fn evaluate(input: &EvalInput) -> Result<EvalReport> {
    let gold = load_gold_rows(&input.gold_jsonl)
        .with_context(|| format!("load gold {}", input.gold_jsonl.display()))?;
    let regex_pred = RegexPredictor::new(compile_all());
    let classifier = Classifier::load_from_dir(&input.model_dir)
        .with_context(|| format!("load classifier {}", input.model_dir.display()))?;
    let fw_pred = FirmwarePredictor::new(classifier);
    let report = run_with_predictors(&gold, &regex_pred, &fw_pred);
    write_csv(&input.output_csv, &report)
        .with_context(|| format!("write csv {}", input.output_csv.display()))?;
    print_summary(&report);
    Ok(report)
}

/// Core eval loop over gold rows + two predictors — the pure-function
/// version used by tests. Does not touch disk.
///
/// Exposed `pub(crate)` so integration tests can wire MockClassifier
/// implementations without needing Firmware files on disk.
pub(crate) fn run_with_predictors(
    gold: &[GoldRow],
    regex_pred: &dyn CategoryPredictor,
    firmware_pred: &dyn CategoryPredictor,
) -> EvalReport {
    let regex_preds = predict_all(regex_pred, gold);
    let firmware_preds = predict_all(firmware_pred, gold);
    let truth: Vec<&str> = gold.iter().map(|g| g.category.as_str()).collect();
    let confusion_regex = build_confusion(&truth, &regex_preds);
    let confusion_firmware = build_confusion(&truth, &firmware_preds);
    let regex_metrics = compute_metrics(&truth, &regex_preds);
    let firmware_metrics = compute_metrics(&truth, &firmware_preds);
    EvalReport {
        total_gold_rows: gold.len(),
        regex_metrics,
        firmware_metrics,
        confusion_regex,
        confusion_firmware,
    }
}

/// One parsed gold row — shared input type for predictors + metrics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldRow {
    pub category: String,
    pub text: String,
}
