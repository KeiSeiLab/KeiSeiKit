---
title: patcher.rs
path: kei-skills/src/patcher.rs
dna_hash: sha256:60473d777f1c0f5f
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-skills/src/patcher.rs

Fuzzy find-replace on SKILL.md body.

Hermes' `fuzzy_match.py` ranks candidate windows by similarity and
picks the best match above a threshold. We reuse the `similar` crate
(workspace dep) — `TextDiff::ratio` gives a `[0.0, 1.0]` score akin
to Python's `difflib.SequenceMatcher.ratio` (Hermes baseline).

Atomic write: serialize to a sibling `.tmp` file, fsync, rename.

## Public API

- `pub const FUZZY_THRESHOLD` — Minimum similarity for a fuzzy match. 0.85 mirrors agentskills floor.
- `pub fn patch_skill` — Apply a find-replace to the skill's body. In-memory; persist via
- Exact-match path. Returns `Ok(Some(_))` on hit, `Ok(None)` on miss
- Slide a line-aligned window of `old`'s line-count over `body`; pick
- Score every line-aligned window of length `span`; keep those above
- Apply selection rules: when `replace_all`, take every hit; else take
- Translate (start_line, end_line) tuples into byte offsets and stitch.
- Convert line-index regions into byte-index regions over the original body.
- `pub fn write_atomic` — Persist a patched skill atomically: write `<path>.tmp`, fsync, rename.

## Related

- parent: `kei-skills/Cargo.toml`
- imports: crate, similar, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
