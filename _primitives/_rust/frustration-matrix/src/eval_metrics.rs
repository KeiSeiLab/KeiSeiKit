//! Metric math — pure functions over two parallel label vectors.
//!
//! No IO, no predictors, no disk. Every function takes `&[&str]` + returns
//! numbers or HashMaps. Follows sklearn convention:
//!
//!   * precision_c  = TP_c / (TP_c + FP_c)       (0 if denominator 0)
//!   * recall_c     = TP_c / (TP_c + FN_c)       (0 if denominator 0)
//!   * f1_c         = 2 · P · R / (P + R)        (0 if denominator 0)
//!   * support_c    = number of gold rows with true=c
//!   * accuracy     = correct / total            (0 if total=0)
//!
//! Macro-F1 is computed in `eval_report`; it is the arithmetic mean of
//! per-category f1 scores over categories WITH support > 0.
//!
//! Zero-support categories: we still emit a row (precision=recall=f1=0),
//! so the report can show them — matches the spec test
//! `per_category_metrics_handle_zero_support`.

use crate::eval::{Metrics, PerCategoryMetric};
use std::collections::{BTreeSet, HashMap};

/// Compute full metrics bundle from parallel truth / prediction vectors.
///
/// Panics only on a length mismatch (that would be a programming error
/// in the eval loop, not a runtime condition we expect).
pub fn compute_metrics(truth: &[&str], pred: &[String]) -> Metrics {
    assert_eq!(
        truth.len(),
        pred.len(),
        "compute_metrics: truth/pred length mismatch ({}, {})",
        truth.len(),
        pred.len()
    );
    let accuracy = compute_accuracy(truth, pred);
    let per_category = compute_per_category(truth, pred);
    Metrics {
        accuracy,
        per_category,
    }
}

/// Overall accuracy — correct predictions over total rows.
fn compute_accuracy(truth: &[&str], pred: &[String]) -> f64 {
    if truth.is_empty() {
        return 0.0;
    }
    let correct = truth
        .iter()
        .zip(pred.iter())
        .filter(|(t, p)| **t == p.as_str())
        .count();
    correct as f64 / truth.len() as f64
}

/// One `PerCategoryMetric` per category that appears in EITHER vector.
///
/// Categories are collected from both `truth` and `pred` to ensure a
/// classifier that over-predicts `"uncategorized"` still shows up with
/// zero precision / support (instead of being silently dropped).
/// Sorted alphabetically for deterministic report order.
fn compute_per_category(truth: &[&str], pred: &[String]) -> Vec<PerCategoryMetric> {
    let cats = collect_categories(truth, pred);
    cats.into_iter()
        .map(|c| per_category_one(&c, truth, pred))
        .collect()
}

/// Sorted set of every category label seen in truth OR pred.
fn collect_categories(truth: &[&str], pred: &[String]) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for t in truth {
        set.insert((*t).to_string());
    }
    for p in pred {
        set.insert(p.clone());
    }
    set.into_iter().collect()
}

/// Compute precision / recall / f1 / support for ONE category label.
/// Division-by-zero is replaced by 0.0 per sklearn `zero_division=0`.
fn per_category_one(cat: &str, truth: &[&str], pred: &[String]) -> PerCategoryMetric {
    let counts = count_tp_fp_fn(cat, truth, pred);
    let precision = safe_div(counts.tp, counts.tp + counts.fp);
    let recall = safe_div(counts.tp, counts.tp + counts.fn_);
    let f1 = if precision + recall > 0.0 {
        2.0 * precision * recall / (precision + recall)
    } else {
        0.0
    };
    PerCategoryMetric {
        category: cat.to_string(),
        precision,
        recall,
        f1,
        support: counts.tp + counts.fn_,
    }
}

/// TP / FP / FN counts for one category under one-vs-rest framing.
struct Counts {
    tp: usize,
    fp: usize,
    fn_: usize,
}

fn count_tp_fp_fn(cat: &str, truth: &[&str], pred: &[String]) -> Counts {
    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut fn_ = 0usize;
    for (t, p) in truth.iter().zip(pred.iter()) {
        let t_is = **t == *cat;
        let p_is = p == cat;
        match (t_is, p_is) {
            (true, true) => tp += 1,
            (false, true) => fp += 1,
            (true, false) => fn_ += 1,
            (false, false) => {}
        }
    }
    Counts { tp, fp, fn_ }
}

fn safe_div(num: usize, den: usize) -> f64 {
    if den == 0 {
        0.0
    } else {
        num as f64 / den as f64
    }
}

/// Build a (true, predicted) → count confusion matrix.
///
/// Keys are `(String, String)` so the map outlives the borrow on `truth`;
/// memory cost is negligible (gold sets are O(100) rows).
pub fn build_confusion(
    truth: &[&str],
    pred: &[String],
) -> HashMap<(String, String), usize> {
    assert_eq!(truth.len(), pred.len(), "build_confusion: length mismatch");
    let mut out: HashMap<(String, String), usize> = HashMap::new();
    for (t, p) in truth.iter().zip(pred.iter()) {
        *out.entry(((*t).to_string(), p.clone())).or_insert(0) += 1;
    }
    out
}

/// Macro-F1 = arithmetic mean of per-category f1 over categories with
/// support > 0. Zero-support categories are excluded so adding unseen
/// labels to the report doesn't dilute the number.
pub fn macro_f1(m: &Metrics) -> f64 {
    let with_support: Vec<&PerCategoryMetric> = m
        .per_category
        .iter()
        .filter(|p| p.support > 0)
        .collect();
    if with_support.is_empty() {
        return 0.0;
    }
    let total: f64 = with_support.iter().map(|p| p.f1).sum();
    total / with_support.len() as f64
}
