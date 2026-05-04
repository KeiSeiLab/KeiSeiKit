---
title: executor.rs
path: kei-import-project/src/executor.rs
dna_hash: sha256:766ebf788e8e0e28
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-import-project/src/executor.rs

executor — build per-phase execution plan; optionally pre-register in kei-ledger.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
Does NOT spawn agents — callers feed each prompt to Agent({...}) externally.

## Public API

- Lifecycle status of a phase in the execution plan.
- Tracking record for one phase in the execution plan.
- `pub struct ExecutorPlan` — Output of `build_executor_plan`: parallel lists (records[i] ↔ prompts[i]).
- `pub fn build_executor_plan` — Build a per-phase execution plan without invoking agents.
- `pub fn prereg_phases` — Pre-register each phase as a 'queued' row in kei-ledger.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, sha2, std

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
