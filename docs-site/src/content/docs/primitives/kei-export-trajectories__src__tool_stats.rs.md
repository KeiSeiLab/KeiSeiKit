---
title: tool_stats.rs
path: kei-export-trajectories/src/tool_stats.rs
dna_hash: sha256:16ce7f7da0b6bee1
language: rust
size_loc: 127
generated: by-keidocs
---

# kei-export-trajectories/src/tool_stats.rs

Tool-statistic extraction + key-set normalization.

Constructor Pattern: pure functions over the in-memory event stream
returned by `ledger_reader`. Stats live in a `BTreeMap` so the JSONL
output is byte-stable across runs (golden-test friendly).

The Hermes spec demands that every JSONL line in an export carry the
SAME `tool_stats` key set — the union of all tools observed across all
trajectories. Per-trajectory missing tools land as `{count:0,success:0,
failure:0}` zero defaults so HuggingFace `datasets` doesn't choke on a
ragged Arrow schema.

## Public API

- One observed tool invocation, distilled out of the event stream by
- `pub fn aggregate_tool_stats` — Aggregate a flat list of `ToolEvent`s into per-tool counters. Returned
- `pub fn normalize_keys` — Walk every trajectory once to build the union key set, then walk again
- First pass — collect every tool name observed across the whole batch.
- Second pass — for each trajectory, insert zero-default entries for any

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: crate, std

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
