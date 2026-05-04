---
title: lib.rs
path: kei-model-router/src/lib.rs
dna_hash: sha256:f8b0be49646ad26b
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-model-router/src/lib.rs

kei-model-router — model selection for Claude Code Agent spawns.

Concern: given an incoming Agent invocation (subagent_type, prompt,
task-class DNA), pick the cheapest model in {Haiku 4.5, Sonnet 4.6,
Opus 4.7} that meets the empirical quality bar for similar past
invocations. Reads from `kei-ledger` posterior, writes back outcomes.

Constructor Pattern: each cube under 200 LOC, each function under 30.
Cubes assembled here:

- `pricing` — verified per-MTok constants (RULE 0.4, 2026-04-30)
- `dna_class` — task-class DNA extraction (strip nonce/body suffixes)
- `complexity` — τ-estimator (regex+length+role heuristics)
- `posterior` — Beta posterior from ledger rows per (task-class, model)
- `kernel` — substrate similarity for unseen task classes
- `select` — decision rule: argmin cost s.t. P[q ≥ q*] ≥ 1−δ
- `escalate` — retry-ladder bookkeeping

Distinct from `kei-router` (which handles NL→tool dispatch and
generic LLM provider abstraction). This crate's only job is selecting
WHICH Claude tier to spawn an Agent on.

## Related

- parent: `kei-model-router/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
