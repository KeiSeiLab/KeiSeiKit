//! Byte-level n-gram language firmware.
//!
//! Encodes `P(next_char | last_k_chars)` for k ∈ 1..=max_depth as a sparse
//! hashmap of `(context, next_char) → count`. Compact: ~10-50 KB for a
//! single language class. Replaces BPE/word-embeddings for likelihood
//! scoring.
//!
//! Theorem
//! backing: internal calibration-6 (Shannon entropy on space-separated
//! token streams; Phase 5 entropy curve: 3 chars → 1.91 bits, 7 chars →
//! 0.59 bits — depth-4 is the knee).
//!
//! Constructor Pattern: this file holds struct + API only. Corpus loading
//! is in `firmware_corpus.rs`, the n-gram accumulator in `firmware_ngram.rs`.

use crate::firmware_corpus::load_corpus_text;
use crate::firmware_ngram::NGramStats;
use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Default max-depth for n-gram contexts.
///
/// internal calibration entropy curve: 0 chars → 4.48 bits, 3 chars →
/// 1.91 bits (−57%), 7 chars → 0.59 bits (−87%). Depth-4 is the knee —
/// most marginal gain per KB of storage on corpora in the 10-25 MB range.
/// Beyond k=4 the sparse map size grows ~3× per depth for ~15% entropy
/// reduction. 
pub const DEFAULT_MAX_DEPTH: usize = 4;

/// Minimum context count required to retain an n-gram entry.
///
/// internal predecessor line 25: `min_count=2`. Drops hapax-legomena
/// which inflate size with no predictive value. [E1 VERIFIED: source]
pub const DEFAULT_MIN_COUNT: u32 = 2;

/// Compact byte-level n-gram firmware.
///
/// Fields are `pub` to match the spec. `ngrams` keys are UTF-8 context
/// strings (1..=max_depth chars long). Inner maps hold counts of each
/// observed next-character, not probabilities — keeps storage integer
/// and defers division to `log_likelihood`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Firmware {
    /// Stable index of chars that passed `min_count`, sorted by codepoint.
    pub alphabet: Vec<char>,
    /// `P(char)` per alphabet index. Used as fallback when context unseen.
    pub unigram: Vec<f64>,
    /// `k ∈ 1..=max_depth` for all context lengths stored.
    pub max_depth: usize,
    /// `context → (next_char → count)`. Sparse: only observed contexts.
    pub ngrams: HashMap<String, HashMap<char, u32>>,
    /// Total chars scanned during training (before `min_count` filter).
    pub total_chars: u64,
}

impl Firmware {
    /// Train a firmware from a directory of `.md` / `.txt` / `.jsonl` files.
    ///
    /// For `.jsonl`, extracts user turns only. For `.md`, drops
    /// `### Assistant` blocks. See `firmware_corpus` for the extractor.
    pub fn train_from_dir(path: &Path, max_depth: usize) -> Result<Self> {
        let text = load_corpus_text(path)
            .with_context(|| format!("load corpus from {}", path.display()))?;
        Ok(Self::train_from_text(&text, max_depth))
    }

    /// Train from an in-memory string (tests, one-shot use).
    pub fn train_from_text(text: &str, max_depth: usize) -> Self {
        let depth = max_depth.max(1);
        let mut stats = NGramStats::new(depth, DEFAULT_MIN_COUNT);
        stats.observe_text(text);
        stats.finalize()
    }

    /// Log-likelihood of `text` under this firmware.
    ///
    /// Uses max-available depth for each position, backs off to shorter
    /// contexts if unseen, finally to unigram. Unseen chars at unigram
    /// level are assigned a floor probability of `1 / (total_chars + 1)`
    /// to keep the value finite (no `-inf`).
    pub fn log_likelihood(&self, text: &str) -> f64 {
        let chars: Vec<char> = text.chars().collect();
        let mut total = 0.0_f64;
        for i in 0..chars.len() {
            total += self.log_prob_at(&chars, i);
        }
        total
    }

    /// Persist to gzipped JSON. JSON keeps the file human-grepable; gzip
    /// brings a 25 MB-trained firmware well under 50 KB (internal phase
    /// reported 2981× compression ratio).
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("mkdir {}", parent.display()))?;
            }
        }
        let file = fs::File::create(path)
            .with_context(|| format!("create {}", path.display()))?;
        let mut enc = GzEncoder::new(file, Compression::best());
        let json = serde_json::to_vec(self).context("serialize firmware")?;
        enc.write_all(&json).context("gz write")?;
        enc.finish().context("gz finish")?;
        Ok(())
    }

    /// Load from gzipped JSON produced by `save`.
    pub fn load(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("open {}", path.display()))?;
        let mut dec = GzDecoder::new(file);
        let mut buf = Vec::new();
        dec.read_to_end(&mut buf).context("gz read")?;
        let fw: Firmware = serde_json::from_slice(&buf).context("parse firmware json")?;
        Ok(fw)
    }

    /// Probability of `chars[i]` given `chars[..i]` at max available depth.
    /// Returns log P; falls back from deepest available context to unigram.
    fn log_prob_at(&self, chars: &[char], i: usize) -> f64 {
        let target = chars[i];
        let max_back = self.max_depth.min(i);
        for back in (1..=max_back).rev() {
            let ctx: String = chars[i - back..i].iter().collect();
            if let Some(p) = self.prob_in_context(&ctx, target) {
                return p.ln();
            }
        }
        self.log_prob_unigram(target)
    }

    /// Probability of `target` under context `ctx`, or None if ctx unseen.
    fn prob_in_context(&self, ctx: &str, target: char) -> Option<f64> {
        let next_map = self.ngrams.get(ctx)?;
        let total: u32 = next_map.values().sum();
        if total == 0 {
            return None;
        }
        let count = next_map.get(&target).copied().unwrap_or(0);
        // Add-one smoothing ONLY when target is absent from context's next-
        // set — keeps probability strictly positive without disturbing seen
        // transitions.
        if count == 0 {
            let alpha = self.alphabet.len().max(1) as f64;
            return Some(1.0 / (total as f64 + alpha));
        }
        Some(count as f64 / total as f64)
    }

    /// Unigram fallback with a `1/(N+1)` floor for unseen chars.
    fn log_prob_unigram(&self, target: char) -> f64 {
        if let Some(idx) = self.alphabet.iter().position(|c| *c == target) {
            let p = self.unigram[idx];
            if p > 0.0 {
                return p.ln();
            }
        }
        let floor = 1.0 / (self.total_chars as f64 + 1.0);
        floor.ln()
    }
}
