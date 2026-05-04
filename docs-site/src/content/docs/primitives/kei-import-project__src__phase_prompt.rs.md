---
title: phase_prompt.rs
path: kei-import-project/src/phase_prompt.rs
dna_hash: sha256:c1fcec65727534b5
language: rust
size_loc: 178
generated: by-keidocs
---

# kei-import-project/src/phase_prompt.rs

phase_prompt — generates agent-prompt JSON per migration phase.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- The fully-rendered prompt spec for one migration phase.
- `pub fn build_phase_prompt` — Build a `PhasePrompt` from a `ParsedPhase`.
- `pub fn render_json` — Render all prompts as a JSON array string.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
