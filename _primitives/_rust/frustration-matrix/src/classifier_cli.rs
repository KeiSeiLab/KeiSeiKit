//! CLI glue for the `classify` subcommand.
//!
//! Constructor Pattern: main.rs stays dispatch-only; this cube owns the
//! print-a-ranking layer. Pure function of (dir, message, min_len,
//! threshold). Splits load + classify + print into three tiny helpers.

use crate::classifier::Classifier;
use anyhow::Result;
use std::path::Path;

/// Entry point called from `main::dispatch`. Load bundle, classify, print.
pub fn run(
    dir: &Path,
    message: &str,
    min_len: usize,
    threshold: f64,
) -> Result<()> {
    let cls = Classifier::load_from_dir(dir)?;
    let res = cls.classify(message, min_len, threshold);
    print_header();
    for s in &res.scores {
        print_row(&s.category, s.log_ratio, s.normalized);
    }
    print_best(res.best_category.as_deref());
    Ok(())
}

fn print_header() {
    println!("{:<28} {:>12} {:>14}", "CATEGORY", "LOG_RATIO", "NORMALIZED");
}

fn print_row(category: &str, log_ratio: f64, normalized: f64) {
    println!("{:<28} {:>12.4} {:>14.6}", category, log_ratio, normalized);
}

fn print_best(best: Option<&str>) {
    match best {
        Some(c) => println!("\nbest: {c}"),
        None => println!("\nbest: (none — msg too short or no category above threshold)"),
    }
}
