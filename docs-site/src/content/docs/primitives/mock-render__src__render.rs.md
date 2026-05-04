---
title: render.rs
path: mock-render/src/render.rs
dna_hash: sha256:9eb206908fe4061e
language: rust
size_loc: 49
generated: by-keidocs
---

# mock-render/src/render.rs

Playwright subprocess wrapper (RULE 0.2 exception 6 — JS-only binding).
Calls `npx playwright screenshot` with clear error messages.

Requires Node + `npx`. Playwright browsers installable via:
npx playwright install chromium

## Public API

- `pub fn screenshot` — Render a URL (typically http://localhost:<port>/<page>) or a file:// URL

## Related

- parent: `mock-render/Cargo.toml`
- imports: std

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
