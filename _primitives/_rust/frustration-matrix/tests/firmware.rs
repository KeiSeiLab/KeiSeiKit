//! Firmware tests — cover training, save/load, multilingual alphabet,
//! unigram fallback, and size budget (≤50 KB at depth 4 on 1 MB corpus).
//!
//! Like `tests/integration.rs`, we link source modules via `#[path]` so
//! the binary crate doesn't need to export a library surface.

#[path = "../src/jsonl.rs"]
mod jsonl;
#[path = "../src/firmware_ngram.rs"]
mod firmware_ngram;
#[path = "../src/firmware.rs"]
mod firmware;
#[path = "../src/firmware_corpus.rs"]
mod firmware_corpus;

use firmware::Firmware;
use std::fs;
use tempfile::tempdir;

// ---------------------------------------------------------------
// 1. train_bigram_from_trivial_text
//    "abab" has seen only (a→b) and (b→a). Querying "ab" must score
//    higher than "ac" whose second-position transition is unseen.
// ---------------------------------------------------------------
#[test]
fn train_bigram_from_trivial_text() {
    let fw = Firmware::train_from_text("abababab", 1);
    let ll_ab = fw.log_likelihood("ab");
    let ll_ac = fw.log_likelihood("ac");
    assert!(
        ll_ab > ll_ac,
        "ll_ab={} should exceed ll_ac={} (seen vs unseen transition)",
        ll_ab,
        ll_ac,
    );
    assert!(ll_ac.is_finite(), "ll_ac must be finite, got {}", ll_ac);
}

// ---------------------------------------------------------------
// 2. save_load_roundtrip — alphabet + ngrams identical after a round trip.
// ---------------------------------------------------------------
#[test]
fn save_load_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let corpus = dir.path().join("corpus");
    fs::create_dir_all(&corpus).expect("mkdir corpus");
    fs::write(corpus.join("a.txt"), "the quick brown fox jumps over the lazy dog").unwrap();
    fs::write(corpus.join("b.txt"), "the rain in spain falls mainly on the plain").unwrap();
    let fw = Firmware::train_from_dir(&corpus, 3).expect("train");
    let out = dir.path().join("fw.json.gz");
    fw.save(&out).expect("save");
    let loaded = Firmware::load(&out).expect("load");
    assert_eq!(loaded.alphabet, fw.alphabet, "alphabet mismatch");
    assert_eq!(loaded.max_depth, fw.max_depth, "max_depth mismatch");
    assert_eq!(loaded.total_chars, fw.total_chars, "total_chars mismatch");
    assert_eq!(loaded.ngrams, fw.ngrams, "ngrams mismatch");
    let t = "the fox";
    assert!(
        (loaded.log_likelihood(t) - fw.log_likelihood(t)).abs() < 1e-9,
        "log_likelihood differs after roundtrip",
    );
}

// ---------------------------------------------------------------
// 3. multilingual_corpus_splits_alphabet — Cyrillic and Latin both present.
// ---------------------------------------------------------------
#[test]
fn multilingual_corpus_splits_alphabet() {
    // Repeat each char enough times to clear the min_count=2 filter.
    let text = "privet privet мир мир hello hello world world";
    let fw = Firmware::train_from_text(text, 2);
    let has_latin = fw.alphabet.iter().any(|c| matches!(*c, 'a'..='z'));
    let has_cyrillic = fw.alphabet.iter().any(|c| {
        let u = *c as u32;
        (0x0400..=0x04FF).contains(&u)
    });
    assert!(has_latin, "alphabet missing Latin: {:?}", fw.alphabet);
    assert!(has_cyrillic, "alphabet missing Cyrillic: {:?}", fw.alphabet);
}

// ---------------------------------------------------------------
// 4. unseen_context_falls_back_to_unigram — finite log-lik even when
//    context is not in the n-gram map.
// ---------------------------------------------------------------
#[test]
fn unseen_context_falls_back_to_unigram() {
    let fw = Firmware::train_from_text("the the the the", 3);
    // 'x' never seen — unigram fallback must use the floor, not -inf.
    let ll = fw.log_likelihood("xyz");
    assert!(ll.is_finite(), "log_likelihood returned non-finite: {}", ll);
    // `t` is in the alphabet but a context like "zz" is unseen;
    // back-off to unigram must give a non-zero finite value.
    let ll2 = fw.log_likelihood("zzt");
    assert!(ll2.is_finite(), "log_likelihood after backoff: {}", ll2);
}

// ---------------------------------------------------------------
// 5. depth_4_on_small_corpus_stays_under_50kb — size budget sanity.
//    Generates a 1 MB corpus of predictable prose; saves; asserts
//    file size < 50 KB. This is the internal compression-ratio target.
// ---------------------------------------------------------------
#[test]
fn depth_4_on_small_corpus_stays_under_50kb() {
    let dir = tempdir().expect("tempdir");
    let corpus = dir.path().join("corpus");
    fs::create_dir_all(&corpus).expect("mkdir");
    // 1 MB of natural-ish English — repeated sentences keep context
    // count bounded while still exercising depth-4 branching.
    let sentence = "the quick brown fox jumps over the lazy dog near the old oak tree. ";
    let mut buf = String::with_capacity(1_048_576);
    while buf.len() < 1_048_576 {
        buf.push_str(sentence);
    }
    fs::write(corpus.join("prose.txt"), &buf).unwrap();
    let fw = Firmware::train_from_dir(&corpus, 4).expect("train");
    let out = dir.path().join("fw.json.gz");
    fw.save(&out).expect("save");
    let size = fs::metadata(&out).expect("stat").len();
    assert!(
        size < 50 * 1024,
        "firmware file size {} bytes exceeds 50 KB budget",
        size,
    );
}
