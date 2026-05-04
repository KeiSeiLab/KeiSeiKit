---
title: SKILL
path: site-builder/SKILL.md
dna_hash: sha256:75ce859f561de4f7
language: markdown
size_loc: 334
generated: by-keidocs
---

# site-builder/SKILL.md

## Public API

- `/site-builder — WYSIWYD website builder` — ---
- `When to use` — Triggers: `/site-builder`, "create website", "build a site", "landing page", "portfolio site", "SaaS site", "docs site".
- `Output contract` — Produces a complete website project as local code:
- `Prerequisites` — - Node 20+ with `npx` available
- `Phase 0 — Intake via AskUserQuestion` — Send questions in AskUserQuestion calls (max 4 per call; use 2 calls if more).
- `Phase 1 — Section selection` — After Phase 0, pick SECTIONS based on site type.
- `Phase 2 — Foundation` — Create project scaffold (Astro example):
- `Phase 3 — WYSIWYD block-by-block build (THE CORE LOOP)` — For EACH section in the approved list:
- `Exit 0: file unchanged since lock, OK to proceed` — 
- `Exit 2: DRIFT — re-render + re-approve before continuing` — ```
- `Phase 4 — Audit (parallel)` — After all sections locked, run 4 audits in parallel:
- `Phase 5 — Preview` — Spin up a preview URL before production deploy:
- `Phase 6 — Deploy` — Only after user explicitly confirms preview:
- `State file — site-state.json` — Maintained by `mock-render lock/verify`. Shape:
- `Handoffs (sub-skills called)` — | Sub-skill | When |
- `Forbidden` — - Generating a section file without immediately rendering a screenshot of it
- `Anti-patterns (AI slop guards)` — Enforced at generation time — block the section and regenerate if detected:
- `Output report format` — ```
- `References` — - `$HOME/.claude/agents/_primitives/_rust/mock-render/` — WYSIWYD enforcer (Rust)

## Related

- parent: `site-builder`

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
