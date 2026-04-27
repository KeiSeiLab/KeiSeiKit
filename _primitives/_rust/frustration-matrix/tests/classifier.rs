//! Integration tests for the likelihood-ratio classifier.
//!
//! Uses `#[path]` to pull the modules under test directly from src/,
//! matching the pattern used in `tests/integration.rs` (no library
//! crate surface).
//!
//! Test fixtures are built via `Firmware::train_from_text` (Z1's
//! in-memory trainer) so we don't need disk I/O for most cases. The
//! two `load_from_dir*` tests DO hit disk via `tempfile`.

#[path = "../src/jsonl.rs"]
mod jsonl;
#[path = "../src/firmware_ngram.rs"]
mod firmware_ngram;
#[path = "../src/firmware_corpus.rs"]
mod firmware_corpus;
#[path = "../src/firmware.rs"]
mod firmware;
#[path = "../src/classifier.rs"]
mod classifier;

use classifier::{Classifier, MIN_LEN, THRESHOLD};
use firmware::Firmware;
use std::collections::HashMap;
use tempfile::tempdir;

// ---------------------------------------------------------------
// 1. load_from_dir_requires_neutral — dir without neutral.fw → Err.
// ---------------------------------------------------------------
#[test]
fn load_from_dir_requires_neutral() {
    let dir = tempdir().expect("tempdir");
    // Write one category fw but NO neutral.fw.
    let fw = Firmware::train_from_text("hello world hello world", 3);
    fw.save(&dir.path().join("alpha.fw"))
        .expect("save alpha.fw");
    let err = Classifier::load_from_dir(dir.path())
        .expect_err("expected load without neutral.fw to fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("neutral"),
        "error should mention neutral.fw, got: {msg}"
    );
}

// ---------------------------------------------------------------
// 2. load_from_dir_accepts_categories — dir with 3 .fw (2 cat + neutral)
//    → loads 2 categories.
// ---------------------------------------------------------------
#[test]
fn load_from_dir_accepts_categories() {
    let dir = tempdir().expect("tempdir");
    let neutral =
        Firmware::train_from_text("lorem ipsum dolor sit amet consectetur", 3);
    neutral
        .save(&dir.path().join("neutral.fw"))
        .expect("save neutral.fw");
    let alpha = Firmware::train_from_text("alpha alpha alpha beta gamma", 3);
    alpha
        .save(&dir.path().join("alpha.fw"))
        .expect("save alpha.fw");
    let bravo = Firmware::train_from_text("bravo bravo bravo charlie delta", 3);
    bravo
        .save(&dir.path().join("bravo.fw"))
        .expect("save bravo.fw");

    let cls =
        Classifier::load_from_dir(dir.path()).expect("classifier load");
    assert_eq!(
        cls.categories.len(),
        2,
        "expected 2 categories, got {}",
        cls.categories.len()
    );
    assert!(cls.categories.contains_key("alpha"));
    assert!(cls.categories.contains_key("bravo"));
}

// ---------------------------------------------------------------
// 3. classify_short_message_returns_none — msg < min_len → None.
// ---------------------------------------------------------------
#[test]
fn classify_short_message_returns_none() {
    let cls = make_classifier_in_memory();
    let short = "hi"; // 2 chars, well below MIN_LEN=20
    let res = cls.classify(short, MIN_LEN, THRESHOLD);
    assert!(
        res.best_category.is_none(),
        "short message should not classify"
    );
    assert!(
        res.scores.is_empty(),
        "short-message result should have empty scores, got {}",
        res.scores.len()
    );
}

// ---------------------------------------------------------------
// 4. classify_picks_highest_ratio — build two category firmwares with
//    known training texts; classify an alpha-biased message; assert alpha
//    wins.
// ---------------------------------------------------------------
#[test]
fn classify_picks_highest_ratio() {
    let cls = make_classifier_in_memory();
    // Message strongly biased toward the "alpha" training domain.
    let msg = "alpha alpha alpha alpha alpha";
    let res = cls.classify(msg, MIN_LEN, f64::NEG_INFINITY);
    // Don't assert a specific threshold; we use f64::NEG_INFINITY so the
    // top category ALWAYS wins. The point of this test is ranking.
    assert_eq!(
        res.best_category.as_deref(),
        Some("alpha"),
        "alpha should win on alpha-biased msg. scores: {:?}",
        scores_debug(&res.scores)
    );
}

// ---------------------------------------------------------------
// 5. scores_descending_by_normalized — every consecutive pair of scores
//    must have score[i].normalized >= score[i+1].normalized.
// ---------------------------------------------------------------
#[test]
fn scores_descending_by_normalized() {
    let cls = make_classifier_in_memory();
    let msg = "bravo bravo bravo bravo bravo";
    let res = cls.classify(msg, MIN_LEN, f64::NEG_INFINITY);
    assert!(res.scores.len() >= 2, "need ≥2 categories to rank");
    for pair in res.scores.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        assert!(
            a.normalized >= b.normalized,
            "scores not descending: {} ({:.4}) before {} ({:.4})",
            a.category,
            a.normalized,
            b.category,
            b.normalized
        );
    }
}

// ---------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------

/// Build an in-memory `Classifier` with two opinionated categories
/// ("alpha" / "bravo") and a mixed-domain neutral baseline. Shared
/// between tests 3-5 to keep the fixture small and intent-clear.
fn make_classifier_in_memory() -> Classifier {
    let alpha_text =
        "alpha alpha alpha alpha alpha alpha alpha alpha alpha alpha";
    let bravo_text =
        "bravo bravo bravo bravo bravo bravo bravo bravo bravo bravo";
    let neutral_text = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor";

    let mut categories: HashMap<String, Firmware> = HashMap::new();
    categories.insert("alpha".to_string(), Firmware::train_from_text(alpha_text, 3));
    categories.insert("bravo".to_string(), Firmware::train_from_text(bravo_text, 3));
    let neutral = Firmware::train_from_text(neutral_text, 3);
    Classifier { categories, neutral }
}

/// Debug-render scores without impl Debug requirement on CategoryScore.
fn scores_debug(scores: &[classifier::CategoryScore]) -> Vec<(String, f64, f64)> {
    scores
        .iter()
        .map(|s| (s.category.clone(), s.log_ratio, s.normalized))
        .collect()
}
