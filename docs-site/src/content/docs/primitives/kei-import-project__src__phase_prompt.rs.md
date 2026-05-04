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
