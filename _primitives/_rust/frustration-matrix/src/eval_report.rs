//! Eval report — CSV writer + human-readable stdout summary.
//!
//! Constructor Pattern: IO-only. All math is already done in
//! `eval_metrics`; this cube just serializes.
//!
//! CSV schema (one row per model per category):
//!   `model,category,precision,recall,f1,support`
//!
//! Stdout format matches the layout in the task spec — fixed-width
//! columns so `grep` / `awk` still work on the summary.

use crate::eval::{EvalReport, Metrics, PerCategoryMetric};
use crate::eval_metrics::macro_f1;
use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Path;

/// CSV header — kept as a const so tests + readers agree.
pub const CSV_HEADER: &str = "model,category,precision,recall,f1,support";

const REGEX_LABEL: &str = "regex";
const FIRMWARE_LABEL: &str = "firmware";

/// Write the full report to a CSV file.
///
/// Parent directory is created if missing. Categories are emitted in
/// alphabetical order for a given model, which matches the ordering
/// produced by `compute_metrics`.
pub fn write_csv(path: &Path, report: &EvalReport) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("mkdir {}", parent.display()))?;
        }
    }
    let mut file = fs::File::create(path)
        .with_context(|| format!("create {}", path.display()))?;
    writeln!(file, "{CSV_HEADER}")?;
    write_model_rows(&mut file, REGEX_LABEL, &report.regex_metrics)?;
    write_model_rows(&mut file, FIRMWARE_LABEL, &report.firmware_metrics)?;
    file.flush().context("flush csv")?;
    Ok(())
}

fn write_model_rows(
    file: &mut fs::File,
    model: &str,
    metrics: &Metrics,
) -> Result<()> {
    for m in &metrics.per_category {
        writeln!(
            file,
            "{},{},{:.6},{:.6},{:.6},{}",
            model, m.category, m.precision, m.recall, m.f1, m.support
        )?;
    }
    Ok(())
}

/// Print a human-readable summary. Mirrors the task-spec layout.
pub fn print_summary(report: &EvalReport) {
    println!("=== EVAL: regex vs firmware ===");
    println!("Gold rows (quality=gold): {}", report.total_gold_rows);
    println!();
    print_overall_line(report);
    println!();
    println!("Per-category:");
    print_category_header();
    for cat in shared_category_set(report) {
        print_category_row(&cat, report);
    }
}

/// Two-line overall block: accuracy + macro-f1 for both models.
fn print_overall_line(report: &EvalReport) {
    let r_acc = report.regex_metrics.accuracy;
    let f_acc = report.firmware_metrics.accuracy;
    let r_mf1 = macro_f1(&report.regex_metrics);
    let f_mf1 = macro_f1(&report.firmware_metrics);
    println!("            regex      firmware");
    println!("accuracy    {:>6.2}     {:>6.2}", r_acc, f_acc);
    println!("macro-f1    {:>6.2}     {:>6.2}", r_mf1, f_mf1);
}

fn print_category_header() {
    println!(
        "{:<22} {:<17} {:<17} {:<17}",
        "category",
        "precision(r->fw)",
        "recall(r->fw)",
        "f1(r->fw)"
    );
}

fn print_category_row(cat: &str, report: &EvalReport) {
    let r = find_cat(&report.regex_metrics.per_category, cat);
    let f = find_cat(&report.firmware_metrics.per_category, cat);
    println!(
        "{:<22} {:<17} {:<17} {:<17}",
        clip(cat, 22),
        fmt_arrow(r.map(|m| m.precision), f.map(|m| m.precision)),
        fmt_arrow(r.map(|m| m.recall), f.map(|m| m.recall)),
        fmt_arrow(r.map(|m| m.f1), f.map(|m| m.f1)),
    );
}

/// Union of categories seen in either model's report, alphabetical.
fn shared_category_set(report: &EvalReport) -> Vec<String> {
    let mut s: BTreeSet<String> = BTreeSet::new();
    for m in &report.regex_metrics.per_category {
        s.insert(m.category.clone());
    }
    for m in &report.firmware_metrics.per_category {
        s.insert(m.category.clone());
    }
    s.into_iter().collect()
}

fn find_cat<'a>(
    pc: &'a [PerCategoryMetric],
    cat: &str,
) -> Option<&'a PerCategoryMetric> {
    pc.iter().find(|m| m.category == cat)
}

fn fmt_arrow(lhs: Option<f64>, rhs: Option<f64>) -> String {
    let l = lhs.unwrap_or(0.0);
    let r = rhs.unwrap_or(0.0);
    format!("{:.2}->{:.2}", l, r)
}

fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let cut: String = s.chars().take(n.saturating_sub(1)).collect();
    format!("{cut}-")
}
