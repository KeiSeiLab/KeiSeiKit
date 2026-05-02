---
name: site-builder
description: Build a website from block recipes via interactive wizard. Asks stack/type/style/sections via AskUserQuestion, generates one section at a time, enforces WYSIWYD (what you see in the mock is byte-identical to what gets deployed) via the mock-render primitive.
---

# /site-builder — WYSIWYD website builder

> **[DEPRECATED — v0.17+]** This skill is a subset of `/site-create`. Prefer
> `/site-create` for new work. `/site-builder` remains for backwards
> compatibility. `removed-after: v2.0 (~2026-09-01)`

> **Core promise:** every section you approve in the preview IS the file that gets deployed. No "approximately like this". Byte-for-byte.

## When to use

Triggers: `/site-builder`, "create website", "build a site", "landing page", "portfolio site", "SaaS site", "docs site".

## Output contract

Produces a complete website project as local code:

```
<project-root>/
├── src/
│   ├── pages/index.astro          (or app/page.tsx for Next)
│   ├── sections/
│   │   ├── Nav.astro              — one file per section (Constructor Pattern)
│   │   ├── Hero.astro
│   │   ├── Features.astro
│   │   ├── Pricing.astro
│   │   └── ...
│   ├── layouts/Base.astro
│   └── tokens.css                  — CSS custom properties
├── public/
│   └── <brand assets>
├── astro.config.mjs                (or next.config.js)
├── package.json
├── site-state.json                 — mock-render lock file (WYSIWYD)
└── mocks/
    ├── Hero.png                    — locked screenshots per section
    ├── Features.png
    └── ...
```

Every `sections/*.astro` is independently regeneratable — editing one never touches the others.

## Prerequisites

- Node 20+ with `npx` available
- `mock-render` Rust primitive installed (built by `install.sh` into `$HOME/.claude/agents/_primitives/_rust/target/release/mock-render`)
- Playwright installed (`npx playwright install chromium` — the skill will prompt if missing)

## Phase 0 — Intake via AskUserQuestion

Send questions in AskUserQuestion calls (max 4 per call; use 2 calls if more).

### Call 1 — 4 questions

- **Site archetype?** SaaS landing / Multi-page marketing / Portfolio / Docs site
- **Framework?** Astro 6 (marketing default) / Next.js 16 (SaaS/app) / Static HTML
- **Visual archetype?** Premium minimalist / Dark moody tech / Editorial long-form / Brutalist anti-design
- **Motion tier?** None / Subtle (Motion LazyMotion) / Rich (GSAP ScrollTrigger + Motion) / Experimental (3D + shaders)

### Call 2 — 3 follow-up questions

- **Deploy target?** Cloudflare Pages (recommended) / Vercel / Local only
- **Brand assets?** User provides / Generate with AI / Minimal (text logo + neutral palette)
- **Include a contact form?** Yes (wire via /form-builder) / No

## Phase 1 — Section selection

After Phase 0, pick SECTIONS based on site type.

Defaults per archetype:

| Archetype | Default sections |
|---|---|
| SaaS landing | Nav, Hero, LogoBar, Features, Testimonials, Pricing, FAQ, CTA, Footer |
| Multi-page marketing | Nav, Hero, Features, CTA, Footer — plus routes `/about`, `/pricing`, `/contact` |
| Portfolio | Nav, Hero, Features (case grid), Testimonials, Contact, Footer |
| Docs site | NavSidebar, content layout, Footer-minimal |

Ask via AskUserQuestion: "Pick sections to include" with multi-select — show the defaults checked, user can add/remove.

For each section selected, ask: "Variant?" (A/B/C).

## Phase 2 — Foundation

Create project scaffold (Astro example):

```bash
npm create astro@latest <project-root> -- --template minimal --typescript strict --no-install --no-git
cd <project-root>
npm install
npm install motion @radix-ui/react-tabs lucide-astro  # per block deps
```

Write `src/tokens.css` using answers from Phase 0:

```css
:root {
  --color-bg: ...;
  --color-fg: ...;
  --color-accent: ...;
  --color-border: ...;
  --radius-card: 0.75rem;
  --space-section: clamp(4rem, 8vw, 8rem);
  --font-display: ...;
  --font-body: ...;
}

@media (prefers-color-scheme: dark) { :root { ... } }
```

If user chose "brand assets: I'll provide", ask free-text for logo path + 2-3 hex colors.

Commit checkpoint: `checkpoint: scaffold + tokens`.

## Phase 3 — WYSIWYD block-by-block build (THE CORE LOOP)

For EACH section in the approved list:

### 3.1 Generate the section file

Write `<project-root>/src/sections/<Name>.astro`:
- Copy from Phase 0 brand answers (or placeholder with user's domain words)
- Tokens from `src/tokens.css` (no hardcoded colors)
- Motion hooks matching Phase 0 motion tier
- Brand assets if provided

**Anti-patterns to enforce:**
- No "AI-powered X" headlines
- No centered gradient + default Inter + purple/blue palette combo
- No 5+ CTAs / tiers / features on one block

### 3.2 Render mock

Start dev server (once per session):
```bash
npm run dev &  # Astro on :4321 or Next on :3000
```

Wait for port to respond:
```bash
for i in {1..30}; do
  curl -s http://localhost:4321 > /dev/null && break
  sleep 1
done
```

Screenshot via the `mock-render` primitive:

```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render screenshot \
  "http://localhost:4321/_block-preview?block=<Name>" \
  --out "<project-root>/mocks/<Name>.png" \
  --viewport 1440x900
```

Create a simple `_block-preview.astro` route that imports and renders one section by query param — include this in the scaffold.

### 3.3 Show user + get approval

Display the screenshot. Ask via AskUserQuestion:

- Approve — lock and move on
- Iterate — tell me what to change
- Switch variant (A/B/C)
- Swap block entirely

### 3.4 Act on approval

**Approve:**
```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render lock \
  --project <project-root> \
  --section src/sections/<Name>.astro \
  --screenshot mocks/<Name>.png
```
Commit: `feat: lock <Name> section`. Move to next section.

**Iterate:** Ask free-text "What to change?", apply surgical edit to `<Name>.astro` only. Re-render. Loop.

**Switch variant:** Re-run 3.1 with different variant flag.

**Swap block:** Go back to Phase 1 for this section slot.

### 3.5 WYSIWYD invariant check before any later write

Before writing to ANY already-locked section:

```bash
$HOME/.claude/agents/_primitives/_rust/target/release/mock-render verify \
  --project <project-root> \
  --section src/sections/<Name>.astro
# Exit 0: file unchanged since lock, OK to proceed
# Exit 2: DRIFT — re-render + re-approve before continuing
```

This invariant means the final deploy is guaranteed to look like the last screenshot the user approved.

## Phase 4 — Audit (parallel)

After all sections locked, run 4 audits in parallel:

```
/a11y-audit scan src/
/seo-audit <project-root>
/responsive-audit src/pages/index.astro
/perf-audit src/
```

Report findings grouped by severity. For each fix proposed:
1. Run `mock-render verify` on the affected section
2. If verify passes AND fix is minor (e.g., add `alt=""`, tweak meta tag) — apply
3. If fix alters layout — ask user to re-approve (back to 3.2 for that section)
4. If fix spans multiple sections — STOP, report, let user decide

## Phase 5 — Preview

Spin up a preview URL before production deploy:

- Cloudflare Pages: `npx wrangler pages deploy <build-dir> --project-name=<slug>-preview`
- Vercel: `npx vercel --preview` (returns preview URL)
- Local: `npm run preview` on :4321

Send URL to user.

## Phase 6 — Deploy

Only after user explicitly confirms preview:

```
/web-deploy deploy --target=<chosen-in-Phase-0> --project=<project-root>
```

Final output:
- Production URL
- Git commit SHA
- Screenshot grid of all locked sections (from `mocks/*.png`)

## State file — site-state.json

Maintained by `mock-render lock/verify`. Shape:

```json
{
  "sections": {
    "Hero":     {"path": "src/sections/Hero.astro",     "sha256": "6a48...", "locked": true, "screenshot": "mocks/Hero.png"},
    "Features": {"path": "src/sections/Features.astro", "sha256": "...",     "locked": true, "screenshot": "mocks/Features.png"},
    "Pricing":  {"path": "src/sections/Pricing.astro",  "sha256": "...",     "locked": false, "screenshot": null}
  }
}
```

Commands:
- `mock-render lock` — freeze current hash after user approves mock
- `mock-render verify` — assert source unchanged before any new write
- `mock-render status` — list sections, lock state, drift check
- `mock-render screenshot` — Playwright wrapper

## Handoffs (sub-skills called)

| Sub-skill | When |
|---|---|
| `/frontend-design` | Phase 2 — if archetype picked but user wants custom tokens |
| `/design-inspiration` | Phase 0 alt — if user wants to see refs before picking style |
| `/site-teardown` | Phase 0 alt — if user provides a ref site URL to clone-style |
| `/ai-animation` | Phase 3 — video-bg hero or scroll loop |
| `/scroll-animation` | Phase 3 — rich/experimental motion tier with pin/scrub |
| `/3d-scene` | Phase 3 — experimental motion tier with R3F hero |
| `/web-effects` | Phase 3 — shader bg, particles |
| `/motion-design` | Phase 3 — subtle motion tier (Motion library) |
| `/form-builder` | Phase 3 — if contact form yes (Phase 0 Call 2 Q3) |
| `/ui-component` | Phase 3 — novel primitive not in blocks |
| `/web-assets` | Phase 4 — image/font/video optimization |
| `/a11y-audit` | Phase 4 — parallel |
| `/seo-audit` | Phase 4 — parallel |
| `/responsive-audit` | Phase 4 — parallel |
| `/perf-audit` | Phase 4 — parallel |
| `/web-deploy` | Phase 6 — production |

## Forbidden

- Generating a section file without immediately rendering a screenshot of it
- Approving a section without calling `mock-render lock`
- Editing a locked section file without first running `mock-render verify`
- Cascading edits that touch multiple sections at once (violates Constructor Pattern for UI)
- Deploying before user approves the preview URL (Phase 5)
- AI-slop anti-patterns
- Hardcoded colors / fonts / spacing outside `tokens.css`
- Breaking the WYSIWYD invariant — the last screenshot the user approved MUST match what's deployed

## Anti-patterns (AI slop guards)

Enforced at generation time — block the section and regenerate if detected:

1. Generic centered hero + gradient + "AI-powered X" subhead
2. Stock 3D isometric illustrations
3. Lorem-ipsum-tier feature copy
4. Every section animated (motion fatigue)
5. 5+ pricing tiers
6. No specific outcome claim (numbers like "47 seconds", "12x faster")
7. Default stack tell: Inter + Slate/Zinc + rounded-lg + Lucide — pick one deviation

## Output report format

```
=== /SITE-BUILDER REPORT ===
Project: <project-root>
Stack: <Astro 6 / Next 16 / static>
Archetype: <SaaS / multi-page / portfolio / docs>
Style: <premium / dark-tech / editorial / brutalist>
Motion tier: <none / subtle / rich / experimental>

Sections built: <N>
  - Nav       locked, 23 KB screenshot
  - Hero      locked, 87 KB screenshot
  - ...

WYSIWYD check: all locked, 0 drift
Audits: a11y=pass seo=2-minor perf=LCP 1.2s responsive=pass
Preview: <url>
Deploy: <url or "pending user confirm">
```

## References

- `$HOME/.claude/agents/_primitives/_rust/mock-render/` — WYSIWYD enforcer (Rust)
- `skills/landing-page/SKILL.md` — predecessor (single-page only)
- `skills/frontend-design/SKILL.md` — archetype philosophy
- `skills/motion-design/SKILL.md` — motion library choices
- `skills/scroll-animation/SKILL.md` — GSAP / scroll-timeline patterns
- `skills/web-deploy/SKILL.md` — CF Pages / Vercel deploy
