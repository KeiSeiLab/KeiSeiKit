---
title: slide-patterns
path: visual-explainer/references/slide-patterns.md
dna_hash: sha256:05480fa9d1d752b0
language: markdown
size_loc: 1406
generated: by-keidocs
---

# visual-explainer/references/slide-patterns.md

## Public API

- `Slide Deck Patterns` — CSS patterns, JS engine, slide type layouts, transitions, navigation chrome, and curated presets for self-contained HTML slide presentations. All slides are viewport-fit (100dvh), single-file, same philosophy as scrollable pages.
- `Planning a Deck from a Source Document` — When converting a plan, spec, review, or any structured document into slides, follow this process before writing any HTML. Skipping it leads to polished-looking decks that silently drop 30–40% of the source material.
- `Slide Engine Base` — The deck is a scroll-snap container. Each slide is exactly one viewport tall.
- `Typography Scale` — Slide typography is 2–3× larger than scrollable pages. Page-sized text on a viewport-sized canvas looks like a mistake.
- `Cinematic Transitions` — IntersectionObserver adds `.visible` when a slide enters the viewport. Slides animate in once and stay visible when scrolling back.
- `Navigation Chrome` — All navigation is `position: fixed` with high z-index, layered above slides. Styled to be visible on any background.
- `SlideEngine JavaScript` — Add once at the end of the page. Handles navigation, chrome updates, and scroll-triggered reveals. Event delegation ensures slide-internal interactions (Mermaid zoom, scrollable code, overflow tables) don't trigger slide navigation.
- `Auto-Fit` — A single post-render function that handles all known content overflow cases. Agents can't perfectly predict how text reflows at every viewport size, so `autoFit()` is a required safety net. Call it after Mermaid/Chart.js render but before SlideEngine init.
- `Slide Type Layouts` — Each type has a defined HTML structure and CSS layout. The agent can adapt colors, fonts, and spacing per aesthetic, but the structural patterns stay consistent.
- `Decorative SVG Elements` — Inline SVG accents lift slides from functional to editorial. Use sparingly — one or two per slide, never on every slide.
- `Proactive Imagery` — Slides should reach for visuals before defaulting to text alone. If a slide could be more compelling with an image, chart, or diagram, add one.
- `Check availability` — which surf
- `Generate (one per target slide)` — surf gemini "descriptive prompt matching deck palette" --generate-image /tmp/ve-slide-title.png --aspect-ratio 16:9
- `Base64 encode for self-containment (macOS)` — TITLE_IMG=$(base64 -i /tmp/ve-slide-title.png)
- `Linux: TITLE_IMG=$(base64 -w 0 /tmp/ve-slide-title.png)` — 
- `Embed in the slide` — 
- `<div class="slide__bg" style="background-image:url('data:image/png;base64,${TITLE_IMG}')"></div>` — 
- `Clean up` — rm /tmp/ve-slide-title.png
- `Compositional Variety` — Consecutive slides must vary their spatial approach. Three centered slides in a row means push one off-axis.
- `Presentation Readability` — Slides get projected, screen-shared, viewed at distance. Design accordingly:
- `Content Density Limits` — Each slide must fit in exactly 100dvh. If content exceeds these limits, the agent splits across multiple slides — never scrolls within a slide.
- `Responsive Height Breakpoints` — Height-based scaling is more critical for slides than width. Each breakpoint progressively reduces padding, font sizes, and hides decorative elements.
- `Curated Presets` — Starting points the agent can riff on. Each defines a font pairing, palette, and background treatment. The agent adapts these to the content — different decks with the same preset should still feel distinct.

## Related

- parent: `visual-explainer/references`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
