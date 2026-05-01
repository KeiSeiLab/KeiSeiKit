---
name: figma-to-code
description: Use when converting Figma designs to code — screenshot, context, tokens, responsive implementation
arguments:
  - name: url
    description: Figma URL (figma.com/design/... or figma.com/make/...)
    required: true
---

# Figma to Code Workflow

## Step 1: Extract from Figma
- Parse URL to get fileKey and nodeId
- Call `get_design_context` with fileKey and nodeId (Figma MCP / REST)
- Call `get_screenshot` for visual reference
- Review returned code, tokens, and component mappings

## Step 2: Analyze Design
From the Figma output, identify:
- **Layout:** flex/grid structure, spacing, alignment
- **Typography:** font family, size, weight, line-height, color
- **Colors:** map to project's design tokens or CSS variables
- **Components:** map to existing project components
- **Responsive hints:** auto-layout direction, min/max widths
- **Interactions:** hover states, transitions, animations

## Step 3: Map to Project
- Match Figma colors → project design tokens
- Match Figma fonts → project typography scale
- Match Figma components → existing project components
- Identify gaps: new components or tokens needed
- If Code Connect mappings exist, use them directly

## Step 4: Implement
- Use project's stack (React, Next.js, Astro, SvelteKit, etc.)
- Mobile-first responsive implementation
- Match pixel-perfect on design breakpoint
- Adapt gracefully to other breakpoints
- Use design system components where they match

### Responsive Breakpoints
- Mobile: 375px (default)
- Tablet: 768px
- Desktop: 1024px
- Wide: 1280px

## Step 5: Verify
- Compare screenshot with implementation side-by-side
- Check all breakpoints
- Verify interactive states (hover, focus, active)
- Accessibility check (contrast, keyboard nav, ARIA)
- Cross-browser check (Chrome, Safari, Firefox)

## Step 6: Commit
- `feat: implement <component/page> from Figma design`
