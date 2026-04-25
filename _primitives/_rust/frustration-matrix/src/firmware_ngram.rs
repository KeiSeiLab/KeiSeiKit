//! N-gram statistics accumulator — a pure cube.
//!
//! Single-pass scan over a UTF-8 string: for every position `i`, observe
//! contexts of every length `k ∈ 1..=max_depth` ending at `i-1` paired
//! with the char at `i`. Final step filters hapax-legomena (`min_count`)
//! and builds the alphabet + unigram vector on alphabet indices.
//!
//! Constructor Pattern: no IO, no dependencies on `Firmware`. Produces
//! owned `HashMap`s that `Firmware::finalize` moves into the struct.

use crate::firmware::Firmware;
use std::collections::HashMap;

/// Mutable accumulator for one training pass.
pub struct NGramStats {
    max_depth: usize,
    min_count: u32,
    total_chars: u64,
    unigram_counts: HashMap<char, u64>,
    ngram_counts: HashMap<String, HashMap<char, u32>>,
}

impl NGramStats {
    pub fn new(max_depth: usize, min_count: u32) -> Self {
        Self {
            max_depth,
            min_count,
            total_chars: 0,
            unigram_counts: HashMap::new(),
            ngram_counts: HashMap::new(),
        }
    }

    /// Consume a chunk of UTF-8 text. Character-boundary-safe: we iterate
    /// over `chars()` and rebuild context strings via `collect::<String>()`,
    /// never `&text[i..j]` byte slices (see markdown.rs note on `×`).
    pub fn observe_text(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        for i in 0..chars.len() {
            self.count_unigram(chars[i]);
            self.count_ngrams_at(&chars, i);
        }
    }

    fn count_unigram(&mut self, ch: char) {
        *self.unigram_counts.entry(ch).or_insert(0) += 1;
        self.total_chars += 1;
    }

    /// For position `i`, record every context of length `k ∈ 1..=max_depth`
    /// ending at `i-1` with next-char `chars[i]`. Skipped at `i=0`.
    fn count_ngrams_at(&mut self, chars: &[char], i: usize) {
        if i == 0 {
            return;
        }
        let max_back = self.max_depth.min(i);
        for back in 1..=max_back {
            let ctx: String = chars[i - back..i].iter().collect();
            let nxt = chars[i];
            self.ngram_counts
                .entry(ctx)
                .or_insert_with(HashMap::new)
                .entry(nxt)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    /// Build the final `Firmware`. Applies `min_count` filter on each
    /// `(context, next_char)` pair, drops newly-empty contexts, then
    /// derives alphabet + unigram vector from the filtered unigram map.
    pub fn finalize(self) -> Firmware {
        let alphabet = build_alphabet(&self.unigram_counts, self.min_count);
        let unigram = build_unigram(&alphabet, &self.unigram_counts, self.total_chars);
        let ngrams = filter_ngrams(self.ngram_counts, self.min_count);
        Firmware {
            alphabet,
            unigram,
            max_depth: self.max_depth,
            ngrams,
            total_chars: self.total_chars,
        }
    }
}

/// Alphabet = chars with `count >= min_count`, sorted by codepoint.
/// Deterministic across runs — critical for save/load round-trip tests.
fn build_alphabet(counts: &HashMap<char, u64>, min_count: u32) -> Vec<char> {
    let mut v: Vec<char> = counts
        .iter()
        .filter(|(_, c)| **c >= min_count as u64)
        .map(|(ch, _)| *ch)
        .collect();
    v.sort_unstable();
    v
}

/// Unigram vector aligned to alphabet order. `P(ch) = count / total`.
fn build_unigram(
    alphabet: &[char],
    counts: &HashMap<char, u64>,
    total: u64,
) -> Vec<f64> {
    if total == 0 {
        return vec![0.0; alphabet.len()];
    }
    alphabet
        .iter()
        .map(|ch| counts.get(ch).copied().unwrap_or(0) as f64 / total as f64)
        .collect()
}

/// Drop n-grams below `min_count`. Contexts that become empty after the
/// filter are removed entirely.
fn filter_ngrams(
    raw: HashMap<String, HashMap<char, u32>>,
    min_count: u32,
) -> HashMap<String, HashMap<char, u32>> {
    let mut out = HashMap::with_capacity(raw.len());
    for (ctx, nexts) in raw {
        let kept: HashMap<char, u32> = nexts
            .into_iter()
            .filter(|(_, c)| *c >= min_count)
            .collect();
        if !kept.is_empty() {
            out.insert(ctx, kept);
        }
    }
    out
}
