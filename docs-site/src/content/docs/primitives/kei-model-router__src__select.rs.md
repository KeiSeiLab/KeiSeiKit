---
title: select.rs
path: kei-model-router/src/select.rs
dna_hash: sha256:06f8f28e9625e36d
language: rust
size_loc: 288
generated: by-keidocs
---

# kei-model-router/src/select.rs

Decision rule — the heart of the router.

m*(d̂) = argmin_{m ∈ M} { c(d̂, m) | P[q(d̂, m) ≥ q*] ≥ 1 − δ }

Implementation:
1. Compute `task_class_dna` from full DNA.
2. For each model m ∈ {Haiku, Sonnet, Opus}:
a. Pull posterior from ledger for (task_class, m).
b. If n=0 → optionally smooth via kernel from similar task_classes.
c. Compute q_lower(δ).
3. Filter to models where q_lower ≥ q*.
4. Among feasible: pick cheapest (smallest expected cost).
5. If feasible set empty → fallback.

Per RULE -1: empty feasible set → return fallback (top tier), NOT an
error. Router never refuses; it surfaces uncertainty by selecting
safer model.

Constructor Pattern: this is the orchestrating cube. SQL is delegated
to `posterior`, math to `pricing`, similarity to `kernel`.

## Public API

- Pinned override: if Some, skip routing and use this. For per-agent pins.
- Estimated input/output token counts; if None, use defaults.
- `pub const DEFAULT_TOKENS_IN` — Sensible defaults for a typical Agent spawn (~ 4k in, 1.5k out).
- Pull all (task_class_dna, model) posteriors weighted by kernel(task_class, *).

## Related

- parent: `kei-model-router/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
