# Phase 1 — Design (tokens + typography)

> Goal: produce `src/tokens.css` and a typography choice aligned with the
> Phase-0 `STYLE` archetype. 1 AskUserQuestion.
> **Verify criterion:** `src/tokens.css` written, passes `cssparser` (or a
> quick `curl`+ file inspection), fonts declared.

---

## 1.a — Invoke /frontend-design

Delegate to the `frontend-design` skill with the Phase-0 archetype:

```
/frontend-design archetype=<STYLE-derived> differentiator=<one-line from DESC>
```

Map `STYLE` → archetype:

| Phase-0 STYLE | frontend-design archetype |
|---|---|
| Premium minimalist | minimal |
| Dark / moody tech | retro-futuristic OR swiss (dark skin) |
| Editorial / long-form | editorial |
| Brutalist / anti-design | brutalist |

The sub-skill produces design tokens (color + type + spacing) in OKLCH form.

---

## 1.b — Brand asset wiring

Depending on `BRAND` from Phase 0:

- **I'll provide** — ask free-text once for the logo path + 2-3 hex colors.
  Convert hex to OKLCH before writing into tokens.
- **Generate with AI** — fan out to an optional AI-asset generator of
  your choice (skill-agnostic; the generator is not part of this
  pipeline's required deps). Save to `public/brand/logo.svg` (or .png).
- **Minimal** — emit a text-only logo placeholder; no image asset.

---

## 1.c — Write `src/tokens.css`

Shape:

```css
:root {
  /* Color (OKLCH — one --brand-hue controls the whole palette) */
  --brand-hue: <from frontend-design>;
  --color-bg:      oklch(<L> <C> var(--brand-hue));
  --color-fg:      oklch(<L> <C> var(--brand-hue));
  --color-accent:  oklch(<L> <C> calc(var(--brand-hue) + 30));
  --color-muted:   oklch(<L> <C> var(--brand-hue));
  --color-border:  oklch(<L> <C> var(--brand-hue));

  /* Type */
  --font-display: "<display>", serif;
  --font-body:    "<body>", sans-serif;

  /* Space + radius */
  --space-section: clamp(4rem, 8vw, 8rem);
  --radius-card:   0.75rem;
}

@media (prefers-color-scheme: dark) {
  :root { /* dark-mode overrides */ }
}
```

If `STACK = Astro 6` or `Next.js 16`, also emit `src/styles/tokens.css` and
import it from the root layout.

---

## 1.d — One AskUserQuestion: confirm direction

Send a single `AskUserQuestion` with the rendered token preview
(swatch block + font-pair line) and 3 options:

- Looks good — proceed to Phase 2
- Adjust palette — loop back to 1.a with a "more muted" / "more saturated" hint
- Swap typography — loop back to 1.a with a "different fonts" hint

---

## 1.e — Checkpoint commit

```
checkpoint: phase-1 design tokens + typography
```

Proceed to Phase 2.
