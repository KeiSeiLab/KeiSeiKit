# Phase 4 — Parallel Audit (a11y / seo / responsive / perf)

> Goal: run 4 audit skills in parallel against the locked site; collect
> findings grouped by severity; get user approval per fix.
> **Verify criterion:** all 4 audits ran; findings surfaced or confirmed
> zero; any applied fix passed `mock-render verify` first.

---

## 4.a — Fan-out parallel

Run the 4 audit skills concurrently (tool fan-out is allowed for audits —
they are read-only and independent):

```
/a11y-audit      scan src/
/seo-audit       <project-root>
/responsive-audit src/pages/index.astro
/perf-audit      src/
```

Each returns a findings list with severity (`critical / important / nice`)
and, where possible, a file+line + suggested patch.

Merge the 4 result streams into a single aggregated list.

---

## 4.b — Group + present findings

Group findings by severity. For each, show:

- Severity (critical / important / nice)
- Category (a11y / seo / responsive / perf)
- File + line
- Description (1 sentence)
- Proposed fix (1 sentence)
- Affected section(s) (map file path → section name)

---

## 4.c — One AskUserQuestion: apply fixes?

Three options:

- **Apply all non-layout fixes automatically** — tweak meta tags, alt
  attributes, `fetchpriority`, preload hints, other non-visual edits.
  Per-fix algorithm in 4.d below.
- **Review each fix individually** — loop per finding, ask approve/skip.
- **Skip audit fixes** — proceed to Phase 5 with findings in the report.

---

## 4.d — Per-fix algorithm (applies to any chosen fix)

For EVERY fix the skill is about to write:

1. Determine the affected section file (if any).
2. Run `mock-render verify --section <file>` first.
   - Exit 0 → proceed to step 3.
   - Exit 2 → STOP. Report drift to user; loop back to Phase 3.3 for that
     section (re-render, re-approve, re-lock) before attempting the fix.
3. If the fix is non-layout (meta tag, alt, preload, `loading="lazy"`,
   `decoding="async"`, `aria-*`) → apply directly.
4. If the fix alters layout (CSS classes that move content, new DOM nodes,
   removed sections) → do NOT apply silently. Flag it back to the user:
   > "Fix #N changes layout. Re-approval via Phase 3.3 needed. Proceed?"
5. After EVERY applied fix, re-run `mock-render lock` on the affected
   section so the frozen hash matches the new source.
6. Commit: `fix(site): <audit-category> — <short description>`.

---

## 4.e — Verify criterion

- All 4 audit skills completed.
- Findings list fully walked (either applied, individually approved/skipped,
  or deferred per 4.c choice).
- `mock-render status` shows 0 drift rows.

Emit:
`Phase 4 done: <a11y-findings> a11y / <seo> seo / <resp> responsive / <perf> perf. Proceeding to preview.`

Proceed to Phase 5.
