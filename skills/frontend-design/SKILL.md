---
name: frontend-design
description: Use when designing web UI before coding — anti-AI-slop aesthetic philosophy, typography pairing, color theory, spatial composition, motion guidelines, design archetypes. Triggers on "design", "UI design", "frontend design", "anti-slop", "make it look premium", "design thinking".
arguments:
  - name: archetype
    description: "Archetype: editorial, swiss, brutalist, minimal, maximalist, retro-futuristic, organic, industrial, art-deco, lo-fi (auto-suggest if omitted)"
    required: false
  - name: differentiator
    description: "The ONE thing someone will remember about this design"
    required: false
---

# Frontend Design — Think Before You Code

> Design-first, code-second. Every implementation starts with a design decision, not a div.

## Phase Gate (MANDATORY before writing any UI code)

1. **Purpose** — What problem? Who uses it? (1 sentence)
2. **Archetype** — Pick from 10 below (sets the aesthetic DNA)
3. **Differentiator** — "The one thing someone remembers" (1 sentence)
4. **Anti-references** — Name 3 sites/patterns this is NOT
5. **Tokens** — Define palette + fonts + spacing in CSS variables

Skip this gate = skip the skill. Code without design intent = AI slop.

## Hard Bans (Anti-AI-Slop)

**Typography:**
- Inter, Roboto, Arial, system font stacks
- Space Grotesk (overused in AI-generated sites)
- Same font for heading and body

**Color:**
- Purple gradients on white backgrounds
- Evenly distributed palettes (everything gets equal weight)
- Pure #000 or #fff without tinting

**Layout:**
- Centered card grids as default composition
- Hero → Cards → Testimonials → Footer (the template trap)
- Even spacing everywhere (no rhythm)

**Motion:**
- `linear` easing on UI transitions
- `scale(0)` animation origins
- Default `ease` without custom cubic-bezier

## 10 Archetypes

| # | Name | Typography | Color | Layout | Motion |
|---|------|-----------|-------|--------|--------|
| 1 | **Editorial** | Serif display + sans body | Warm neutrals + 1 accent | Asymmetric columns, pull quotes | Subtle parallax, text reveals |
| 2 | **Swiss** | Geometric sans (Helvetica Now, Neue Haas) | Black/white + 1 primary | Strict grid, mathematical spacing | Minimal, precision timing |
| 3 | **Brutalist** | Monospace or system | Raw, high contrast | Exposed structure, raw HTML aesthetic | Glitch, intentional jank |
| 4 | **Minimal** | 1 refined sans, extreme weight contrast | 2 colors max + neutral | Massive whitespace, single column | Fade only, ultra-slow |
| 5 | **Maximalist** | Mixed display fonts, decorative | Saturated, 4+ colors | Layered, overlapping, collage | Everything moves, scroll-driven |
| 6 | **Retro-Futuristic** | Futuristic display + mono | Neon on dark, CRT glow | Scanlines, terminal aesthetic | Typing effects, flicker |
| 7 | **Organic** | Rounded sans + handwritten accent | Earth tones, muted | Curved containers, blob shapes | Fluid, spring physics |
| 8 | **Industrial** | Condensed bold sans | Dark grays + safety yellow/orange | Dense info, data-heavy | Mechanical, step-based |
| 9 | **Art Deco** | Geometric display, high contrast weight | Gold/brass + deep navy/black | Symmetrical, ornamental borders | Elegant reveals, fade + scale |
| 10 | **Lo-Fi** | Hand-drawn or pixel font | Paper/notebook palette | Sketch-like borders, tape/sticker elements | Wobbly, imperfect |

## Typography Rules

- Max 2 fonts: 1 display (headings) + 1 body (text)
- Use `clamp()` for fluid scaling: `font-size: clamp(1rem, 2.5vw, 1.5rem)`
- Body `line-height`: 1.4-1.6 | Display `line-height`: 1.0-1.2
- 3-5 clear hierarchy levels with dramatic size contrast (4:1 heading-to-body)
- Tune `letter-spacing` per size: tighter for large, looser for small caps
- `font-feature-settings` for ligatures, tabular numbers where needed

## Color System (OKLCH)

```css
@theme {
  --brand-hue: 250;
  --color-primary: oklch(0.6 0.2 var(--brand-hue));
  --color-surface: oklch(0.995 0.005 var(--brand-hue));
  --color-text: oklch(0.15 0.02 var(--brand-hue));
  --color-muted: oklch(0.55 0.01 var(--brand-hue));
  --color-accent: oklch(0.7 0.25 calc(var(--brand-hue) + 30));
  --color-border: oklch(0.9 0.01 var(--brand-hue));
}
```

**60-30-10 rule:** 60% dominant (surface/bg), 30% secondary (text/containers), 10% accent (CTAs, highlights).

OKLCH = perceptually uniform. One `--brand-hue` controls entire palette.

## Spatial Composition

- Consistent scale: `--space-xs: 0.25rem` through `--space-3xl: 4rem`
- Whitespace is structural, not leftover
- At least ONE grid-breaking moment per page (full-bleed, overlap, offset)
- 8px base grid for alignment
- Dramatic rhythm changes between sections (dense → spacious → dense)

## Visual Depth & Texture

- Noise/grain via SVG `<feTurbulence>` filter or CSS pseudo-element
- Multi-value `box-shadow` for realistic depth
- `backdrop-filter: blur()` for glass effects
- `clip-path` for non-rectangular shapes
- Background: gradients, patterns, grain — never flat solid white

## Motion Guidelines

- Custom `cubic-bezier()` per element — never default `ease`
- Staggered page-load: 50-100ms increments between elements
- Duration: productivity UI <300ms, creative 200-500ms
- Spring physics for interactive elements (bounce: 0, no jello)
- Exit animations subtler than enter
- `prefers-reduced-motion`: replace motion with fade, keep <200ms
- Keyboard-initiated actions: NO animation

### Enter Animation Recipe (Motion/Framer Motion)

```jsx
initial={{ opacity: 0, y: 8, filter: "blur(4px)" }}
animate={{ opacity: 1, y: 0, filter: "blur(0px)" }}
transition={{ type: "spring", duration: 0.45, bounce: 0 }}
```

## Output Contract

Every frontend-design invocation MUST produce:
1. **Stated direction** — archetype + differentiator + anti-references
2. **Design tokens** — CSS custom properties (colors, type, spacing)
3. **Typography selection** — 2 fonts with Google Fonts / Fontsource links
4. **Working code** — implementation matching the stated direction
5. **Responsiveness** — mobile-first, tested at 375px and 1280px

## The Blur Test

At 20% visibility, the layout silhouette should be distinguishable from anti-references. If blurred Stripe and blurred Your-Page look the same → composition is not distinctive.

## Diverge-Kill-Mutate

If output feels generic:
1. **Diverge** — generate 3 structurally different variants (different spatial logic, not color swaps)
2. **Kill** — binary: alive or dead. NO blending (blending = averaging = AI slop)
3. **Mutate** — within survivor, introduce named "breaks" (violations of convention)
4. **Repeat** — each cycle moves further from center
