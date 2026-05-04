---
title: classifier.rs
path: frustration-matrix/src/classifier.rs
dna_hash: sha256:e04d90730cca8654
language: rust
size_loc: 185
generated: by-keidocs
---

# frustration-matrix/src/classifier.rs

Firmware-based likelihood-ratio classifier.

One cube, one responsibility: given per-category firmwares + a neutral
baseline, assign a message to the category with the highest
length-normalized log-likelihood ratio
`log P(msg|cat) − log P(msg|neutral)` / chars(msg).

ASSUMPTION: `firmware.rs` is supplied by a parallel Z1 agent (RULE 0.13).
Expected API: `Firmware::{train_from_dir, train_from_text,
log_likelihood, save, load}`. We adapt call sites here; never edit Z1.

Ratio removes base-rate language entropy (~1 bit/char). Length-normalize
so long messages don't trivially outscore short ones.

Defaults (see constants below): MIN_LEN = 20 — DERIVED from internal
n-gram entropy (` §4 l.54:
"7-9 chars of context = almost full predictability"; max_depth = 8 → 2
full prediction windows = 16 → 20 with safety margin). THRESHOLD = 0.0
— any net-positive ratio counts; permissive default for tuning later.

## Public API

- `pub const MIN_LEN` — Default minimum message length (chars) below which classification is
- `pub const THRESHOLD` — Default normalized log-ratio threshold. See module docs §Defaults.
- Stem used for the baseline firmware file inside the model directory.
- File extension expected for firmware files on disk.
- Bundle of trained firmwares: per-category + neutral baseline.
- Category name (file stem) → trained firmware.
- Baseline firmware; ratios are reported against this.
- `pub struct ClassificationResult` — Result of classifying a single message. `scores` is always populated
- `pub struct CategoryScore` — Per-category score for one input. `log_ratio = ll(cat) − ll(neutral)`;
- `pub fn load_from_dir` — Load bundle from directory. `<dir>/neutral.fw` REQUIRED; every
- `pub fn classify` — Classify one message. `best_category = None` when msg is shorter
- Compute + sort scores for every loaded category. Split out to keep
- Pick the winning category if its normalized score clears `threshold`.
- List `*.fw` files under `dir`. Sorted for deterministic test output.
- Split the list into `(neutral_path, category_paths)`; err if no neutral.
- Build `HashMap<stem, Firmware>` from the category path list.
- UTF-8 file stem as `&str`, or `None` for non-UTF-8 / missing stems.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
