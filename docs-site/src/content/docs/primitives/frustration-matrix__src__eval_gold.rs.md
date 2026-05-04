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
