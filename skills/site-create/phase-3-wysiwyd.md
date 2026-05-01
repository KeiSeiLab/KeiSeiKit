# Phase 3 — WYSIWYD Block-by-Block Build (CORE LOOP)

> Goal: for each section in `SECTIONS`, generate the source file, render a
> mock, get user approval, then **lock** via `mock-render` so the
> source-SHA is frozen.
>
> This phase enforces the LOAD-BEARING invariant: the screenshot the user
> approves IS the file that gets deployed. Byte-for-byte.
> **Verify criterion per section:** `site-state.json` has `locked: true` +
> matching `sha256` for that section.

---

## 3.0 — One-time setup (first section only)

Start the dev server via the `live-preview.sh` primitive:

```bash
$HOME/.claude/agents/_primitives/live-preview.sh start <project-root>
```

This writes `.keisei/dev-server.pid` — the WYSIWYD PostToolUse hook
(`hooks/site-wysiwyd-check.sh`) uses this file to decide whether to run
drift checks on subsequent Edit/Write operations.

Wait for the port to respond (max 30s poll). If it never comes up, fall
back to printing the dev-server log tail and ask the user whether to
abort or retry.

Create a helper preview route `<project-root>/src/pages/_block-preview.astro`
that takes `?block=<Name>` and renders only that section. This isolates
sections for per-section screenshots without bleed-through.

---

## 3.1 — For each section in SECTIONS: generate

Write `<project-root>/src/sections/<Name>.astro` (or `<Name>.tsx` for Next /
`<Name>.svelte` for SvelteKit):

- Props: none (sections are concrete, not generic)
- Tokens: only CSS custom properties from `src/tokens.css`
- No hardcoded hex / pixel / font values
- Copy: use Phase-0 `DESC`-derived placeholders; first section includes the
  product name from `DESC`
- Motion: match `MOTION` tier from Phase 0 (none / subtle / rich / experimental)
- File stays < 200 LOC (Constructor Pattern) — split into sub-components if
  it grows

---

## 3.2 — Render mock

```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render screenshot \
  "http://localhost:4321/_block-preview?block=<Name>" \
  --out "<project-root>/mocks/<Name>.png" \
  --viewport 1440x900
```

If Playwright is not installed, the primitive fails with a clear error —
prompt user to `npx playwright install chromium` and retry.

---

## 3.3 — Show + approve (1 AskUserQuestion per section)

Display `mocks/<Name>.png` inline. Ask:

- **Approve** — lock and move on
- **Iterate** — free-text what to change (single free-text moment inside
  the skill, allowed per Phase-3 exception)
- **Switch variant** (A / B / C) — regenerate 3.1 with different variant
- **Swap block** — pick a different block for this slot, re-loop 3.1

---

## 3.4 — Act on approval

### Approve

```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render lock \
  --project <project-root> \
  --section src/sections/<Name>.astro \
  --screenshot mocks/<Name>.png
```

This writes into `<project-root>/site-state.json`:

```json
{
  "sections": {
    "<Name>": { "path": "src/sections/<Name>.astro", "sha256": "...", "locked": true, "screenshot": "mocks/<Name>.png" }
  }
}
```

Commit: `feat(site): lock <Name> section`. Move to next section in `SECTIONS`.

### Iterate / Switch variant / Swap block

Loop back to 3.1 (with the free-text change, the new variant, or the new
block). Do NOT touch any other section's file.

---

## 3.5 — WYSIWYD verify before any cross-section edit

If a later phase (e.g. audit) would edit a locked section, you MUST first:

```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render verify \
  --project <project-root> \
  --section src/sections/<Name>.astro
```

- Exit 0: unchanged since lock — proceed.
- Exit 2: **DRIFT** — stop. Re-render, re-approve via 3.3 loop, re-lock.

This is the hard block that guarantees Phase-6 deploy never ships a section
the user did not approve.

---

## 3.6 — Verify criterion (completion of Phase 3)

All `SECTIONS[i]` must have `locked: true` in `site-state.json`.
Run `mock-render status` and confirm 0 `DRIFT` rows. Emit:

`Phase 3 done: N sections locked, 0 drift. Proceeding to audits.`

Proceed to Phase 4.
