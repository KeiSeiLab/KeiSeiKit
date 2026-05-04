---
title: eval_gold.rs
path: frustration-matrix/src/eval_gold.rs
dna_hash: sha256:a277c820770c3e5a
language: rust
size_loc: 96
generated: by-keidocs
---

# frustration-matrix/src/eval_gold.rs

Gold-set JSONL parser.

One file, one job: read the hand-labelled training set, drop anything
whose `quality != "gold"` (silver is noisy per spec), return clean
`GoldRow`s for the eval pipeline.

Tolerant parse strategy: we deserialize into `serde_json::Value` rather
than a strict struct so upstream shape changes (extra columns, different
source tags) don't break the eval. Only the 4 fields named in the spec
are consulted: `category`, `text`, `source`, `quality`.

One bad line is NEVER a fatal error — the whole file gets skipped only
if `fs::read_to_string` itself fails.

## Public API

- `pub fn load_gold_rows` — Load gold-quality rows from a JSONL file.
- Pure-function variant — public inside the crate for test injection.
- Parse a single JSONL line. Returns `None` if:
- Gold filter: spec says "skip rows with quality != gold (silver is noisy)".

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
