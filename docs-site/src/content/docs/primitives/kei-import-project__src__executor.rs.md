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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
