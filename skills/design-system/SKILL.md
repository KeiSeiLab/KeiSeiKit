---
name: design-system
description: Use when building a design system — tokens, base components, Tailwind config, dark mode, docs
arguments:
  - name: name
    description: Design system / project name
    required: true
  - name: style
    description: "Visual direction: minimal, playful, corporate, editorial"
    required: false
---

# Design System Workflow

## When to use

- Building a design system from scratch — tokens, base components, Tailwind config, dark mode, and docs.
- Establishing a shared visual language across a product or set of related apps.
- Auditing or extending an existing design system to add missing tokens or components.

## Step 1: Define Tokens
Design tokens (adapt to existing project if any):

### Colors
- Primary: main brand color + 50-950 scale
- Secondary: accent color + scale
- Neutral: gray scale for text, borders, backgrounds
- Semantic: success, warning, error, info
- Surface: background levels (0, 1, 2, 3)

### Typography
- Font families: heading, body, mono
- Size scale: xs, sm, base, lg, xl, 2xl, 3xl, 4xl
- Weight: normal, medium, semibold, bold
- Line heights: tight, normal, relaxed

### Spacing
- Scale: 0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24

### Other
- Border radius: none, sm, md, lg, full
- Shadows: sm, md, lg, xl
- Transitions: fast (150ms), normal (300ms), slow (500ms)

## Step 2: Tailwind Config
- For Tailwind 4: declare tokens in CSS via `@theme { ... }` (no `tailwind.config.ts`)
- For Tailwind 3: generate `tailwind.config.ts` with all tokens
- CSS custom properties for runtime theming
- Dark mode: `class` strategy with token overrides

## Step 3: Base Components
Core primitives (adjust to framework):
- Button (primary, secondary, ghost, destructive + sizes)
- Input (text, textarea, select + states)
- Card (surface levels, padding variants)
- Badge (status colors + sizes)
- Typography components (Heading, Text, Label)

## Step 4: Dark Mode
- Map all color tokens to dark equivalents
- Test contrast ratios (WCAG AA minimum: 4.5:1 text, 3:1 large text)
- Surface hierarchy inverts (dark bg, lighter cards)

## Step 5: Documentation
- Token reference table
- Component API (props, variants, examples)
- Usage guidelines (do's and don'ts)
