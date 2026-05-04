---
title: phase-2-propose
path: onboard/phase-2-propose.md
dna_hash: sha256:76a87c4f4ec713f9
language: markdown
size_loc: 154
generated: by-keidocs
---

# onboard/phase-2-propose.md

## Public API

- `Phase 2 — Propose Candidates` — Analyse `SCAN` from Phase 1 and produce `CANDIDATES` — a list of proposed
- `2a — Candidate kinds` — For each project in `PATHS`, emit up to:
- `2b — Agent proposal (per project)` — Compose a dry-run `/new-agent` input based on the scan:
- `2c — Hook proposal (per project)` — Map scan → hook suggestions:
- `2d — Primitive proposal (per project)` — Read `_primitives/MANIFEST.toml` (already on disk). Map scan → primitives:
- `2e — Output structure` — Emit a structured summary (display only — no file write):
- `Candidates for <project-path>` — ### Agent (1 proposal)
- `Verify-criterion` — - Every proposed block name exists under `_blocks/` (ls-verify before

## Related

- parent: `onboard`

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
