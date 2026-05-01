---
name: site-create
description: End-to-end site pipeline — intake → design → sections → WYSIWYD mock-render loop → parallel audits → preview → deploy. Pure-click (≥8 AskUserQuestion blocks). The mock-render verify gate HARD-BLOCKS deploy of unlocked sections.
argument-hint: <optional one-line project intent>
---

# /site-create — 7-Phase Website Pipeline (index)

You convert a free-text product description into a deployed website through
seven strictly-ordered phases. Every decision is a click; only the intake
description (Phase 0) and per-section iteration edits (Phase 3) are typed.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Pipeline overview

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 0 | [phase-0-intake.md](phase-0-intake.md) | 7-question intake batch | 2× AskUserQuestion (4+3) |
| 1 | [phase-1-design.md](phase-1-design.md) | Invoke `/frontend-design`, emit `tokens.css` | 1× AskUserQuestion |
| 2 | [phase-2-sections.md](phase-2-sections.md) | Multi-select sections; variant per section | 2× AskUserQuestion |
| 3 | [phase-3-wysiwyd.md](phase-3-wysiwyd.md) | Per-section generate → mock-render → approve loop | N× (1 per section) |
| 4 | [phase-4-audit.md](phase-4-audit.md) | Parallel a11y / seo / responsive / perf | 1× (apply fixes?) |
| 5 | [phase-5-preview.md](phase-5-preview.md) | Preview deploy URL | 1× (proceed?) |
| 6 | [phase-6-deploy.md](phase-6-deploy.md) | Production deploy via `/web-deploy` | 1× (confirm) |

**Minimum AskUserQuestion count across a complete pipeline: 8+** — pure-click
contract. Only Phase 0 description and per-section iteration prompts are
free-text.

---

## WYSIWYD invariant (LOAD-BEARING)

> **Every section the user approved in the screenshot IS the file that gets
> deployed. Byte-for-byte. No "approximately like this".**

Enforced by the `mock-render` Rust primitive (`_primitives/_rust/mock-render/`):

- `mock-render lock` — freezes source SHA-256 after user-approved screenshot.
- `mock-render verify` — asserts source unchanged before any later write.
- `mock-render status` — lists sections, lock state, drift check.

**Hard block:** Phase 6 (deploy) refuses to run if any locked section shows
drift in `mock-render status`. The pipeline stops and loops the user back
to Phase 3 for that section.

The companion `hooks/site-wysiwyd-check.sh` (PostToolUse Edit|Write) gives a
stderr advisory whenever an edit touches a section file while a
`.keisei/dev-server.pid` exists — catches drift in the moment, not at
deploy time.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `DESC` | Phase 0 | User's product/project intent (1-3 sentences) |
| `STACK` | Phase 0 | Astro 6 / Next 16 / SvelteKit / static |
| `STYLE` | Phase 0 | Premium / dark-tech / editorial / brutalist archetype |
| `MOTION` | Phase 0 | none / subtle / rich / experimental |
| `DEPLOY` | Phase 0 | Cloudflare Pages / Vercel / local |
| `TOKENS` | Phase 1 | CSS custom properties file written to `src/tokens.css` |
| `SECTIONS` | Phase 2 | Ordered list `[{name, variant}]` |
| `LOCKED` | Phase 3 | Set of sections that passed user approval |
| `AUDIT` | Phase 4 | `{a11y, seo, responsive, perf}` findings |
| `PREVIEW_URL` | Phase 5 | Short-lived preview URL |
| `PROD_URL` | Phase 6 | Final deploy URL |

---

## Final report (emit after Phase 6)

```
=== /SITE-CREATE REPORT ===
Intake:         <first 80 chars of DESC>...
Stack:          <STACK>
Style:          <STYLE> / motion: <MOTION>
Sections:       <N locked / M total>
  - Nav      locked  sha256:6a48ca7...
  - Hero     locked  sha256:b37e2d1...
  - ...
WYSIWYD:        <clean | drifted:X sections — BLOCKED>
Audits:         a11y=<pass/N-findings> seo=<..> resp=<..> perf=<LCP Xs>
Preview:        <PREVIEW_URL>
Prod:           <PROD_URL or "pending user confirm">
Next action:    <verify on mobile / share URL / iterate section X>
```

---

## Rules (enforced at every phase)

- **Pure-click contract.** Only `DESC` (Phase 0) and per-section iteration
  prompts (Phase 3) are typed. Every other decision is an `AskUserQuestion`.
  Count them in the final report.
- **WYSIWYD hard block.** Phase 6 refuses to run if `mock-render status`
  shows any drift. See Phase 3.5 for the invariant algorithm.
- **NO DOWNGRADE (RULE -1).** Any phase that fails returns 2-3 constructive
  paths, never "can't be done".
- **NO HALLUCINATION (RULE 0.4).** Every section name / variant / hook
  referenced must exist on disk or in the block recipe. Phase 3 verifies
  before any lock.
- **Plan Mode First (RULE 0.5).** This skill IS the plan; each phase file
  has its own verify-criterion. No Edit/Write to project source before the
  corresponding phase's confirm click.
- **Constructor Pattern (RULE ZERO).** One file per section (Phase 3).
  Generated `sections/*.astro` (or `.tsx`) never exceeds 200 LOC — split
  into sub-sections on the fly.
- **Surgical Changes.** Never edit adjacent sections when iterating one.
  Orphan imports in the edited section are cleaned; neighbours are not
  touched.

---

## References

- [phase-0-intake.md](phase-0-intake.md) · [phase-1-design.md](phase-1-design.md) · [phase-2-sections.md](phase-2-sections.md) · [phase-3-wysiwyd.md](phase-3-wysiwyd.md) · [phase-4-audit.md](phase-4-audit.md) · [phase-5-preview.md](phase-5-preview.md) · [phase-6-deploy.md](phase-6-deploy.md)
- `skills/frontend-design/SKILL.md` — archetype philosophy (Phase 1)
- `skills/site-builder/SKILL.md` — hub-level WYSIWYD reference
- `skills/site-teardown/SKILL.md` — optional Phase 0 alt (clone a reference site)
- `skills/a11y-audit`, `skills/seo-audit`, `skills/responsive-audit`, `skills/perf-audit` — Phase 4 parallel fan-out
- `skills/web-deploy/SKILL.md` — Phase 6 deploy
- `_primitives/_rust/mock-render/` — WYSIWYD enforcer
- `_primitives/live-preview.sh` — dev-server lifecycle
- `_primitives/design-scrape.sh` — optional reference-site scrape (Phase 0 alt)
- `hooks/site-wysiwyd-check.sh` — PostToolUse drift advisory
