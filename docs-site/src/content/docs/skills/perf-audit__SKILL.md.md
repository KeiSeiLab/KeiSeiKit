---
title: SKILL
path: perf-audit/SKILL.md
dna_hash: sha256:2bf7590c3c7ae46d
language: markdown
size_loc: 57
generated: by-keidocs
---

# perf-audit/SKILL.md

## Public API

- `Performance Audit Workflow` — ---
- `When to use` — - Auditing an API endpoint, page, or function for performance regressions (baseline → profile → fix → remeasure).
- `Step 1: Establish Baseline` — - Measure current performance:
- `Step 2: Profile` — - Identify WHERE time is spent:
- `Step 3: Identify Top 3 Bottlenecks` — - Rank by impact (% of total time)
- `Step 4: Checkpoint` — - `checkpoint: before perf-audit $target`
- `Step 5: Fix (One at a Time)` — - Fix #1 bottleneck → measure → confirm improvement
- `Step 6: Final Measurement` — - Re-run baseline measurements
- `Step 7: Commit` — - `perf: optimize $target — <summary of improvements>`

## Related

- parent: `perf-audit`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
