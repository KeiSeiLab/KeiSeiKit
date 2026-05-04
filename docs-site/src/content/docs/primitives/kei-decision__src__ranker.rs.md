---
title: ranker.rs
path: kei-decision/src/ranker.rs
dna_hash: sha256:8f8c19894a38fd5c
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-decision/src/ranker.rs

Topological sort + score-based ranker.

Inputs: a `Vec<RawAction>` plus a parallel `Vec<ActionKind>` from the
classifier. Output: `Vec<RankedAction>` ordered so that:
1. All deps of an action come before it.
2. Within deps-equivalent groups, higher score wins.

Score = severity_weight × (1 / max(effort_hours, 0.5)) × deps_factor
severity_weight: HIGH=10, MEDIUM=5, LOW=2 (default 5)
effort_hours:   parsed from "1-2h" / "30min" / "2-3h" / "1-2d" etc.
deps_factor:    1.0 for no deps, 0.5 per upstream dep (penalises chains)

## Public API

- `pub fn rank_actions` — Topo-sort by deps, then score-rank within levels.
- Deterministic topo order: Kahn's algorithm with score-then-id tie-break.
- Highest score sorts to the END of the ready queue (since we `pop`).
- Parse "1-2h" / "30min" / "2-3h" / "1-2d" / "4-6h" → midpoint in hours.

## Related

- parent: `kei-decision/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
