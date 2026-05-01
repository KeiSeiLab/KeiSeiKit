# Phase 0 — Intake

> Goal: convert free-text product intent into 6 locked decisions that drive
> the rest of the pipeline. 2 AskUserQuestion calls (4 + 3 questions).
> **Verify criterion:** `DESC`, `STACK`, `STYLE`, `MOTION`, `DEPLOY`, `BRAND`,
> `FORM` all set.

---

## 0.a — Description (only free-text in the skill aside from Phase-3 iteration)

Prompt the user for ONE paragraph describing the product/project.
Capture 1-3 sentences into `DESC`. If the invocation already came with an
argument (`/site-create <text>`), skip this and use it.

---

## 0.b — First AskUserQuestion (4 questions)

Send exactly 4 questions in one `AskUserQuestion` call (the UI cap is 4):

1. **Site archetype?** (single-select, stored as `TYPE`)
   - SaaS landing (one page)
   - Multi-page marketing (/ + /about + /pricing + /contact + /blog)
   - Portfolio / personal
   - Docs site

2. **Framework?** (single-select, stored as `STACK`)
   - Astro 6 (recommended for content/marketing)
   - Next.js 16 (recommended for SaaS / app-like)
   - SvelteKit (Runes, compiles small)
   - Static HTML (single index.html)

3. **Visual archetype?** (single-select, stored as `STYLE`)
   - Premium minimalist (Apple / Linear / Anthropic)
   - Dark / moody tech (Vercel / Raycast)
   - Editorial / long-form
   - Brutalist / anti-design

4. **Motion tier?** (single-select, stored as `MOTION`)
   - None (instant, print-like)
   - Subtle (fade-up, stagger — 2026 conversion default)
   - Rich (scroll-linked reveals, micro-interactions)
   - Experimental (3D / shaders / pin-scrub; Awwwards-tier only)

---

## 0.c — Second AskUserQuestion (3 questions)

Send exactly 3 questions in a second `AskUserQuestion` call:

1. **Deploy target?** (stored as `DEPLOY`)
   - Cloudflare Pages (recommended)
   - Vercel (best Next integration)
   - Local only (skip deploy for now)

2. **Brand assets?** (stored as `BRAND`)
   - I'll provide (logo path + colors next)
   - Generate with AI (skill will fan out to an external image generator)
   - Minimal (text logo + neutral palette)

3. **Include a contact form?** (stored as `FORM`)
   - Yes (wire via `/form-builder`)
   - No

---

## 0.d — Branch: reference-site clone?

If `STYLE` needs guidance, offer an OPTIONAL detour (1 extra
AskUserQuestion, skip if the user answered "I know the style already"):

- Clone a reference — invoke `/site-teardown <url>` first, feed extracted
  tokens into Phase 1.
- Start fresh — proceed directly to Phase 1.

---

## 0.e — Verify criterion

All of `DESC`, `TYPE`, `STACK`, `STYLE`, `MOTION`, `DEPLOY`, `BRAND`, `FORM`
must be populated. If any is missing, loop back to the relevant question.

Emit a single-line confirmation: `Intake locked: <TYPE> / <STACK> / <STYLE> / <MOTION> / deploy:<DEPLOY>`. Proceed to Phase 1.
