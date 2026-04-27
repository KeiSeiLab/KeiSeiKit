//! Classifier-driven row emission for the scan loop.
//!
//! Constructor Pattern: one function, one responsibility — given one
//! extracted user-line `Hit` and a loaded `Classifier`, emit exactly one
//! `Row` (the top-scoring category, or `"uncategorized"` if the classifier
//! returned `None`). The regex path lives in `scan::apply_categories`;
//! this file is the firmware-path mirror.

use crate::classifier::{Classifier, THRESHOLD};
use crate::hit::Hit;
use crate::row::Row;

/// Default weight for classifier-emitted rows. Firmware ratios don't yet
/// have a per-category severity dial; flat 1.0 until eval calibrates.
pub const CLASSIFIER_WEIGHT: f64 = 1.0;

/// Fallback category label when the classifier declines (too short or
/// below threshold). Mirrors the string used in eval_predict.
pub const UNCATEGORIZED: &str = "uncategorized";

/// Build one `Row` from `hit` by asking `cls` for its top category.
pub fn build_row(
    hit: &Hit,
    cls: &Classifier,
    fallback_ts: &str,
    min_len: usize,
) -> Row {
    let res = cls.classify(&hit.text, min_len, THRESHOLD);
    let category = res
        .best_category
        .unwrap_or_else(|| UNCATEGORIZED.to_string());
    Row {
        category,
        chatlog_file: hit.file.clone(),
        line_no: hit.line_no,
        timestamp: hit
            .timestamp
            .clone()
            .unwrap_or_else(|| fallback_ts.to_string()),
        quote: hit.text.clone(),
        weight: CLASSIFIER_WEIGHT,
    }
}
