//! Category predictors — trait + two real implementations.
//!
//! The `CategoryPredictor` trait isolates the eval loop from concrete
//! classifier internals so tests can inject lightweight mocks (see
//! `tests/eval.rs`). Two real impls live here:
//!
//!   * `RegexPredictor`     — v1: walk compiled category table, first
//!                             matching regex wins, else "uncategorized".
//!   * `FirmwarePredictor`  — v2: delegate to `Classifier::classify`
//!                             with the permissive `min_len=0, threshold=-inf`
//!                             settings mandated by the spec (we want the
//!                             top category even for very short inputs so
//!                             the eval never returns None for length).
//!
//! Constructor Pattern: one file, one responsibility (turn text → label).
//! All stateless functions except for the two thin predictor structs,
//! which hold their pre-compiled categories / loaded classifier.

use crate::categories::CompiledCategory;
use crate::classifier::Classifier;
use crate::eval::GoldRow;

/// Shared label for anything a classifier cannot place.
pub const UNCATEGORIZED: &str = "uncategorized";

/// Category-classification strategy — trait to allow test stubs.
pub trait CategoryPredictor {
    /// Return the predicted category label for `text`. Must be a total
    /// function: fall back to `UNCATEGORIZED` rather than panic.
    fn predict(&self, text: &str) -> String;
}

/// Regex-based predictor (v1). Walks categories in seed order and picks
/// the id of the first category whose regex list matches. If no category
/// matches, returns `"uncategorized"`.
pub struct RegexPredictor {
    categories: Vec<CompiledCategory>,
}

impl RegexPredictor {
    /// Wrap a pre-compiled category table. Take ownership so the predictor
    /// can be moved into the eval loop without lifetime contortions.
    pub fn new(categories: Vec<CompiledCategory>) -> Self {
        Self { categories }
    }
}

impl CategoryPredictor for RegexPredictor {
    fn predict(&self, text: &str) -> String {
        for c in &self.categories {
            if c.patterns.iter().any(|p| p.is_match(text)) {
                return c.id.to_string();
            }
        }
        UNCATEGORIZED.to_string()
    }
}

/// Firmware-based predictor (v2). Delegates to the loaded `Classifier`.
///
/// We pass `min_len=0` to bypass the length gate (we want a prediction
/// for every row in the gold set, not a skip for short ones), and
/// `threshold=f64::NEG_INFINITY` so the top scorer is always chosen.
/// These relaxations are specific to *eval*; the production `scan`
/// path keeps the production defaults.
pub struct FirmwarePredictor {
    classifier: Classifier,
}

impl FirmwarePredictor {
    pub fn new(classifier: Classifier) -> Self {
        Self { classifier }
    }
}

impl CategoryPredictor for FirmwarePredictor {
    fn predict(&self, text: &str) -> String {
        let res = self.classifier.classify(text, 0, f64::NEG_INFINITY);
        res.best_category.unwrap_or_else(|| UNCATEGORIZED.to_string())
    }
}

/// Run `predictor.predict` over every gold row, preserving order.
///
/// Kept free-standing so tests can share the same loop across
/// `MockClassifier` impls without re-implementing the iteration.
pub fn predict_all<P: CategoryPredictor + ?Sized>(
    predictor: &P,
    gold: &[GoldRow],
) -> Vec<String> {
    gold.iter().map(|g| predictor.predict(&g.text)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::categories::compile_all;

    #[test]
    fn regex_predictor_matches_repeat_signal() {
        let p = RegexPredictor::new(compile_all());
        assert_eq!(p.predict("я же уже просил, опять?"), "repeat-signal");
    }

    #[test]
    fn regex_predictor_uncategorized_on_no_match() {
        let p = RegexPredictor::new(compile_all());
        assert_eq!(
            p.predict("neutral text with no trigger words"),
            UNCATEGORIZED
        );
    }

    #[test]
    fn predict_all_preserves_order() {
        let p = RegexPredictor::new(compile_all());
        let gold = vec![
            GoldRow {
                category: "a".into(),
                text: "опять".into(),
            },
            GoldRow {
                category: "b".into(),
                text: "nothing matches".into(),
            },
        ];
        let preds = predict_all(&p, &gold);
        assert_eq!(preds, vec!["repeat-signal", UNCATEGORIZED]);
    }
}
