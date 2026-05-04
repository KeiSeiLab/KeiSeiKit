---
title: css-patterns
path: visual-explainer/references/css-patterns.md
dna_hash: sha256:0f485853f9002d2c
language: markdown
size_loc: 1833
generated: by-keidocs
---

# visual-explainer/references/css-patterns.md

## Public API

- `CSS Patterns for Diagrams` — Reusable patterns for layout, connectors, theming, and visual effects in self-contained HTML diagrams.
- `Theme Setup` — Always define both light and dark palettes via custom properties. Start with whichever fits the chosen aesthetic, ensure both work.
- `Background Atmosphere` — Flat backgrounds feel dead. Use subtle gradients or patterns.
- `Link Styling` — **Never rely on browser default link colors.** The default blue (`#0000EE`) has poor contrast on dark backgrounds. Style links with `color: var(--accent)` and keep underlines for discoverability. On dark backgrounds, use bright accents (`#22d3ee`, `#34d399`, `#fbbf24`). On light backgrounds, use deeper tones (`#0891b2`, `#059669`, `#d97706`).
- `Section / Card Components` — The fundamental building block. A colored card representing a system component, pipeline step, or data entity.
- `Code Blocks` — Code blocks need explicit whitespace preservation and a max-height constraint. Without these, code runs together and long files overwhelm the page.
- `Directory Tree` — For file structures, use `<pre>` with monospace + `white-space: pre`. Tree connectors (`├──`, `└──`, `│`) only work when vertically aligned — they become noise if text wraps.
- `Overflow Protection` — Grid and flex children default to `min-width: auto`, which prevents them from shrinking below their content width. Long text, inline code badges, and non-wrapping elements will blow out containers.
- `Mermaid Containers` — Mermaid diagrams have two common layout issues: they render too small to read, and they left-align in their container leaving awkward dead space (especially for narrow vertical flowcharts).
- `Grid Layouts` — ### Architecture Diagram (2-column with sidebar)
- `Connectors` — ### CSS Arrow (vertical, between stacked sections)
- `Animations` — ### Staggered Fade-In on Load
- `Sparklines and Simple Charts (Pure SVG)` — For simple inline visualizations without a library:
- `Responsive Breakpoint` — Include a single breakpoint for narrow viewports:
- `Badges and Tags` — Small inline labels for categorizing elements:
- `Lists Inside Nodes` — For tool listings, feature lists, table columns:
- `KPI / Metric Cards` — Large hero number with trend indicator and label. For dashboards, review summaries, and impact sections.
- `Before / After Panels` — Two-column comparison with diff-colored headers. For review pages, migration docs, and feature comparisons.
- `Collapsible Sections` — Native `<details>/<summary>` with styled disclosure. Zero JS, accessible. For lower-priority content: file maps, decision logs, reference sections.
- `Prose Page Elements` — Patterns for documentation, articles, blog posts, and other reading-first content. The key difference from visual explanations: optimize for sustained reading, not scanning.
- `Generated Images` — For AI-generated illustrations embedded as base64 data URIs via `surf gemini --generate-image`. Use sparingly — hero banners, conceptual illustrations, educational diagrams, decorative accents.

## Related

- parent: `visual-explainer/references`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
