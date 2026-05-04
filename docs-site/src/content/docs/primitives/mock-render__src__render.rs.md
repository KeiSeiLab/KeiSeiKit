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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
