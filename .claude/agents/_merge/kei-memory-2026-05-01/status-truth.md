# Wave merge status-truth — kei-memory architecture sweep 2026-05-01

| Wave | shipped | stubs | cargo-check | cargo-test | behaviour-verified |
|---|---|---|---|---|---|
| A — functional ingest + classifier + error | functional | 0 | PASS (orchestrator-verified) | PASS (orchestrator-verified) | yes |
| B — lib crate split | functional | 0 | PASS | PASS | yes |
| C — tfidf idf debounce + JOIN + filter_map | functional | 0 | PASS (post-reconcile) | PASS (post-reconcile) | yes |
| D — commands split + injection_guard tests + patterns UPSERT + nits | functional | 0 | PASS | PASS | yes |

**Orchestrator verify-before-commit (RULE 0.13 §"Verify-before-commit"):**
- `cargo check --all-targets`: PASS (1 unrelated warning Severity::Warn dead-code)
- `cargo test`: 42 passed, 0 failed across 9 binaries
- All 4 waves' STATUS-TRUTH markers collected
- All marked `shipped: functional`, no scaffolding

**Plan-doc reconciliation:** all 4 plan items (Wave A/B/C/D) shipped functionally.
**Public summary:** "4 architecture refactors landed functionally — schema fix, lib split, idf dedup, commands/patterns/nits."
