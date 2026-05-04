---
title: firmware.rs
path: frustration-matrix/src/firmware.rs
dna_hash: sha256:b6333687f4ee4a03
language: rust
size_loc: 170
generated: by-keidocs
---

# frustration-matrix/src/firmware.rs

Byte-level n-gram language firmware.

Encodes `P(next_char | last_k_chars)` for k ∈ 1..=max_depth as a sparse
hashmap of `(context, next_char) → count`. Compact: ~10-50 KB for a
single language class. Replaces BPE/word-embeddings for likelihood
scoring.

Theorem
backing: internal calibration-6 (Shannon entropy on space-separated
token streams; Phase 5 entropy curve: 3 chars → 1.91 bits, 7 chars →
0.59 bits — depth-4 is the knee).

Constructor Pattern: this file holds struct + API only. Corpus loading
is in `firmware_corpus.rs`, the n-gram accumulator in `firmware_ngram.rs`.

## Public API

- `pub const DEFAULT_MAX_DEPTH` — Default max-depth for n-gram contexts.
- `pub const DEFAULT_MIN_COUNT` — Minimum context count required to retain an n-gram entry.
- Compact byte-level n-gram firmware.
- Stable index of chars that passed `min_count`, sorted by codepoint.
- `P(char)` per alphabet index. Used as fallback when context unseen.
- `k ∈ 1..=max_depth` for all context lengths stored.
- `context → (next_char → count)`. Sparse: only observed contexts.
- Total chars scanned during training (before `min_count` filter).
- `pub fn train_from_dir` — Train a firmware from a directory of `.md` / `.txt` / `.jsonl` files.
- `pub fn train_from_text` — Train from an in-memory string (tests, one-shot use).
- `pub fn log_likelihood` — Log-likelihood of `text` under this firmware.
- `pub fn save` — Persist to gzipped JSON. JSON keeps the file human-grepable; gzip
- `pub fn load` — Load from gzipped JSON produced by `save`.
- Probability of `chars[i]` given `chars[..i]` at max available depth.
- Probability of `target` under context `ctx`, or None if ctx unseen.
- Unigram fallback with a `1/(N+1)` floor for unseen chars.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, flate2, serde, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
