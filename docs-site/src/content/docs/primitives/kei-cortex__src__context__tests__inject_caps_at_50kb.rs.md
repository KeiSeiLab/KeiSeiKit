---
title: inject_caps_at_50kb.rs
path: kei-cortex/src/context/tests/inject_caps_at_50kb.rs
dna_hash: sha256:f12e08b1685a48b2
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-cortex/src/context/tests/inject_caps_at_50kb.rs

`build_system_prompt` caps total output at 50 KiB by dropping trailing
sections (skill -> agents -> claude) until the cap holds. Persona is
never trimmed at the section boundary.

Test feeds ~200 KiB across CLAUDE.md + AGENTS.md + skill, asserts that
the rendered output is <= 50 KiB and that trailing sections were dropped
before the leading ones.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

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
