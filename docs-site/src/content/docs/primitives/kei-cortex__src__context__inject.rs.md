---
title: inject.rs
path: kei-cortex/src/context/inject.rs
dna_hash: sha256:fea7fe40a77e3486
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-cortex/src/context/inject.rs

Compose the final system prompt from persona + discovered context +
optional matched skill.

Order (top to bottom in the rendered prompt):
1. Persona system prompt (passed in by the caller; never trimmed).
2. Nearest `CLAUDE.md` from `ctx` — labelled "Project context".
3. Nearest `AGENTS.md` from `ctx` (only if its on-disk path differs
from the CLAUDE.md path, to avoid double-injection on symlinks).
4. The loaded skill body (if any) — labelled "Active skill".

Total length cap: `MAX_TOTAL_BYTES` (50 KiB). Persona is sacred — we
never trim it. Beyond that, sections are dropped *oldest first* in
reverse-injection order: skill → AGENTS.md → CLAUDE.md, until the cap
holds. Each section that survives is included whole; we don't slice
mid-section.

## Public API

- `pub const MAX_TOTAL_BYTES` — Total byte cap for the augmented system prompt.
- `pub fn build_system_prompt` — Build the augmented system prompt.
- First entry matching `kind` (nearest-first invariant from `discover`).
- Nearest AGENTS.md whose path differs from the picked CLAUDE.md (so a
- Render each named section as a `(label, body)` pair in injection order.
- Drop trailing sections (skill → agents → claude) until total ≤ cap.
- Sum of formatted section lengths (cheap upper-bound estimate).
- Stitch sections into a single prompt with `\n\n=== <label> ===\n` headers.
- Last-resort cap when persona alone exceeds the limit. Truncates with

## Related

- parent: `kei-cortex/Cargo.toml`

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
