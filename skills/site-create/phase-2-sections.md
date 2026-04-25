# Phase 2 — Section Selection

> Goal: user picks which sections the site contains, in what order, and which
> variant per section. 2 AskUserQuestion calls.
> **Verify criterion:** `SECTIONS = [{name, variant}, ...]` populated and
> non-empty.

---

## 2.a — Default section list per archetype

From Phase-0 `TYPE`:

| TYPE | Default sections (in order) |
|---|---|
| SaaS landing | Nav, Hero, LogoBar, Features, Testimonials, Pricing, FAQ, CTA, Footer |
| Multi-page marketing | Nav, Hero, Features, CTA, Footer + routes /about, /pricing, /contact |
| Portfolio | Nav, Hero, CaseGrid, About, Contact, Footer |
| Docs site | NavSidebar, Content, TOC, Footer-minimal |

---

## 2.b — First AskUserQuestion: pick sections (multi-select)

Send an `AskUserQuestion` with `multiSelect: true`. Pre-check the defaults
from 2.a; user can add or remove freely. The label for each option carries
a one-line description.

Include under "available" the full set from the block library (approx):

```
Nav, NavSidebar, Hero, Hero-split, Hero-centered, LogoBar, Features,
Features-bento, Features-alternating, Pricing, Pricing-simple, Pricing-tiered,
Testimonials, Testimonials-grid, Testimonials-carousel, CTA, CTA-split,
FAQ, FAQ-accordion, CaseGrid, Contact, Contact-form, Footer, Footer-minimal
```

Store the user's selection (ordered) as `SECTIONS` (names only for now).

---

## 2.c — Second AskUserQuestion: variant per section

For sections that have multiple variants (e.g. Hero-split vs Hero-centered),
send a SECOND `AskUserQuestion` with 3-5 questions (batched — UI max is 4;
use two calls if >4 sections have variants).

Each question: "Variant for <section>?" with A / B / C options. Default
pre-selected is usually the most conservative variant.

Store into `SECTIONS` as `[{name, variant}, ...]`.

---

## 2.d — Verify criterion

- `SECTIONS` is a non-empty ordered list
- Every `{name}` maps to a known block recipe (if a block library is
  installed) OR is one of the default archetype sections
- Every variant is `A`, `B`, or `C`

If any section lacks a known recipe, fall back to a plain skeleton (tokens
only, no fancy variant).

Emit a confirmation line:
`Sections locked: N × {name/variant} — starting WYSIWYD loop`.

Proceed to Phase 3.
