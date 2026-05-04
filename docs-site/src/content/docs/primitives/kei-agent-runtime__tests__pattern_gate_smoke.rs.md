---
title: pattern_gate_smoke.rs
path: kei-agent-runtime/tests/pattern_gate_smoke.rs
dna_hash: sha256:5948751be2a0f506
language: rust
size_loc: 252
generated: by-keidocs
---

# kei-agent-runtime/tests/pattern_gate_smoke.rs

Smoke tests for the generic `PatternGate` (Layer C convergence).

Covers DenyIfMatch, AllowIfMatch, DenyIfUnmatched (scope), bypass env,
and non-applicable tool short-circuit.

## Public API

- S4 — UTF-8 boundary safety.
- H2 — invalid static regex fails closed instead of panicking.
- H3 — AllowIfMatch + task-scope source is structurally invalid; the
- H1 — regex cache behaviour. Same pattern across many calls must stay

## Related

- parent: `kei-agent-runtime/tests`
- imports: kei_agent_runtime, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
