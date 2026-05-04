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
