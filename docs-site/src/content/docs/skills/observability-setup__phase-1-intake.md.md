---
title: phase-1-intake
path: observability-setup/phase-1-intake.md
dna_hash: sha256:c6f9eb1724b42a29
language: markdown
size_loc: 72
generated: by-keidocs
---

# observability-setup/phase-1-intake.md

## Public API

- `Phase 1 — Intake (scale / stack / log target)` — Three orthogonal questions bundled into ONE `AskUserQuestion` call. Every
- `1a — Emit AskUserQuestion (one call, three questions)` — ```json
- `1b — Store answers` — - First answer → `SCALE` ∈ {`single-host`, `small-cluster`, `prod`}
- `1c — Immediate sanity checks (emit as plain message, no clicks)` — - If `SCALE == single-host` AND `STACK == otel-vendor`: warn — vendor OTel
- `Verify-criterion` — - `SCALE`, `STACK`, `LOG_TARGET` all set to one of their enumerated values.

## Related

- parent: `observability-setup`

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
