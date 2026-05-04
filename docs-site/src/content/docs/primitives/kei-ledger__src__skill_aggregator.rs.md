---
title: skill_aggregator.rs
path: kei-ledger/src/skill_aggregator.rs
dna_hash: sha256:b35e959f2af2f297
language: rust
size_loc: 179
generated: by-keidocs
---

# kei-ledger/src/skill_aggregator.rs

Phase D nightly aggregation cube for `skill_invocations`.

Constructor Pattern: one cube = aggregate-read surface. The write side
lives in `skill_metrics.rs`. This file stays at ≤200 LOC.

Decision rules (thresholds per task spec):
- Validated  : total ≥ 10 AND success_rate ≥ 0.90
- Archive    : total ≥ 10 AND success_rate < 0.30
- Reextract  : total ≥ 10 AND success_rate ∈ [0.30, 0.70)
- Insufficient: total < 10

Times: unix-seconds (consistent with rest of ledger).

## Public API

- Recommendation tier for a skill based on aggregated metrics.
- ≥10 invocations AND success_rate ≥ 0.90 → mark stable.
- ≥10 invocations AND success_rate < 0.30 → archive.
- ≥10 invocations AND success_rate ∈ [0.30, 0.70) → re-derive from corpus.
- < 10 invocations → wait for more data.
- Aggregated per-skill metrics for Phase D decision-making.
- Success rate in [0.0, 1.0]. `0.0` when total_invocations == 0.
- Median duration (p50) in milliseconds; 0 when no duration data.
- 95th-percentile duration in milliseconds; 0 when no duration data.
- Unix-second timestamp of the most-recent invocation.
- Derive the recommendation tier from counts and rate.
- Compute p50 and p95 for a single skill via a percentile sub-query.
- Pure fn: index-based p50/p95 from a sorted slice.
- `pub fn aggregate_skills` — Aggregate all skills from `skill_invocations`.

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: rusqlite, serde

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
