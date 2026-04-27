//! Firmware-based likelihood-ratio classifier.
//!
//! One cube, one responsibility: given per-category firmwares + a neutral
//! baseline, assign a message to the category with the highest
//! length-normalized log-likelihood ratio
//! `log P(msg|cat) − log P(msg|neutral)` / chars(msg).
//!
//! ASSUMPTION: `firmware.rs` is supplied by a parallel Z1 agent (RULE 0.13).
//! Expected API: `Firmware::{train_from_dir, train_from_text,
//! log_likelihood, save, load}`. We adapt call sites here; never edit Z1.
//!
//! Ratio removes base-rate language entropy (~1 bit/char). Length-normalize
//! so long messages don't trivially outscore short ones.
//!
//! Defaults (see constants below): MIN_LEN = 20 — DERIVED from internal
//! n-gram entropy (` §4 l.54:
//! "7-9 chars of context = almost full predictability"; max_depth = 8 → 2
//! full prediction windows = 16 → 20 with safety margin). THRESHOLD = 0.0
//! — any net-positive ratio counts; permissive default for tuning later.

use crate::firmware::Firmware;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Default minimum message length (chars) below which classification is
/// skipped. See module docs §Defaults for derivation.
pub const MIN_LEN: usize = 20;

/// Default normalized log-ratio threshold. See module docs §Defaults.
pub const THRESHOLD: f64 = 0.0;

/// Stem used for the baseline firmware file inside the model directory.
const NEUTRAL_STEM: &str = "neutral";

/// File extension expected for firmware files on disk.
const FIRMWARE_EXT: &str = "fw";

/// Bundle of trained firmwares: per-category + neutral baseline.
#[derive(Debug)]
pub struct Classifier {
    /// Category name (file stem) → trained firmware.
    pub categories: HashMap<String, Firmware>,
    /// Baseline firmware; ratios are reported against this.
    pub neutral: Firmware,
}

/// Result of classifying a single message. `scores` is always populated
/// (sorted desc by `normalized`) for diagnostics.
pub struct ClassificationResult {
    pub best_category: Option<String>,
    pub scores: Vec<CategoryScore>,
}

/// Per-category score for one input. `log_ratio = ll(cat) − ll(neutral)`;
/// `normalized = log_ratio / chars(msg)` is the length-fair ranking key.
pub struct CategoryScore {
    pub category: String,
    pub log_ratio: f64,
    pub normalized: f64,
}

impl Classifier {
    /// Load bundle from directory. `<dir>/neutral.fw` REQUIRED; every
    /// other `<stem>.fw` becomes a category keyed by `<stem>`.
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let files = list_firmware_files(dir)?;
        let (neutral_path, category_paths) = partition_neutral(files)?;
        let neutral =
            Firmware::load(&neutral_path).context("load neutral firmware")?;
        let categories = load_category_map(&category_paths)?;
        Ok(Classifier { categories, neutral })
    }

    /// Classify one message. `best_category = None` when msg is shorter
    /// than `min_len` chars or no category's `normalized` clears `threshold`.
    pub fn classify(
        &self,
        msg: &str,
        min_len: usize,
        threshold: f64,
    ) -> ClassificationResult {
        let char_len = msg.chars().count();
        if char_len < min_len {
            return ClassificationResult {
                best_category: None,
                scores: Vec::new(),
            };
        }
        let scores = self.score_all(msg, char_len);
        let best_category = pick_best(&scores, threshold);
        ClassificationResult { best_category, scores }
    }

    /// Compute + sort scores for every loaded category. Split out to keep
    /// `classify` small.
    fn score_all(&self, msg: &str, char_len: usize) -> Vec<CategoryScore> {
        let neutral_ll = self.neutral.log_likelihood(msg);
        let len_f = char_len.max(1) as f64;
        let mut scores: Vec<CategoryScore> = self
            .categories
            .iter()
            .map(|(cat, fw)| {
                let log_ratio = fw.log_likelihood(msg) - neutral_ll;
                CategoryScore {
                    category: cat.clone(),
                    log_ratio,
                    normalized: log_ratio / len_f,
                }
            })
            .collect();
        scores.sort_by(|a, b| {
            b.normalized
                .partial_cmp(&a.normalized)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scores
    }
}

/// Pick the winning category if its normalized score clears `threshold`.
fn pick_best(scores: &[CategoryScore], threshold: f64) -> Option<String> {
    scores
        .first()
        .filter(|s| s.normalized > threshold)
        .map(|s| s.category.clone())
}

/// List `*.fw` files under `dir`. Sorted for deterministic test output.
fn list_firmware_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("read_dir {}", dir.display()))?;
    let mut out = Vec::new();
    for entry in entries {
        let e = entry.context("read dir entry")?;
        let p = e.path();
        if p.is_file() && p.extension() == Some(OsStr::new(FIRMWARE_EXT)) {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

/// Split the list into `(neutral_path, category_paths)`; err if no neutral.
fn partition_neutral(
    files: Vec<std::path::PathBuf>,
) -> Result<(std::path::PathBuf, Vec<std::path::PathBuf>)> {
    let mut neutral: Option<std::path::PathBuf> = None;
    let mut categories = Vec::new();
    for p in files {
        if file_stem(&p) == Some(NEUTRAL_STEM) {
            neutral = Some(p);
        } else {
            categories.push(p);
        }
    }
    let neutral = neutral.ok_or_else(|| {
        anyhow!("model dir missing required {NEUTRAL_STEM}.{FIRMWARE_EXT}")
    })?;
    Ok((neutral, categories))
}

/// Build `HashMap<stem, Firmware>` from the category path list.
fn load_category_map(
    paths: &[std::path::PathBuf],
) -> Result<HashMap<String, Firmware>> {
    let mut out = HashMap::new();
    for p in paths {
        let stem = file_stem(p)
            .ok_or_else(|| anyhow!("firmware file has no stem: {}", p.display()))?
            .to_string();
        let fw = Firmware::load(p)
            .with_context(|| format!("load firmware {}", p.display()))?;
        out.insert(stem, fw);
    }
    Ok(out)
}

/// UTF-8 file stem as `&str`, or `None` for non-UTF-8 / missing stems.
fn file_stem(p: &Path) -> Option<&str> {
    p.file_stem().and_then(|s| s.to_str())
}
