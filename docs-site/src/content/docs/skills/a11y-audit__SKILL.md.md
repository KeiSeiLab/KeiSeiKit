---
title: SKILL
path: a11y-audit/SKILL.md
dna_hash: sha256:f7e34617c549fbb5
language: markdown
size_loc: 102
generated: by-keidocs
---

# a11y-audit/SKILL.md

## Public API

- `Accessibility Audit — WCAG 2.2 AA` — ---
- `Legal Context` — - **EAA (EU):** In force since June 2025. Penalties: up to 100K EUR / 4% revenue
- `Top 10 Violations` — 1. Missing alt text on images
- `Automated Testing` — ```bash
- `Lighthouse CLI` — npx lighthouse <url> --output=json --only-categories=accessibility
- `axe-core` — npx @axe-core/cli <url> --tags wcag2a,wcag2aa,wcag22aa
- `CSS Media Queries` — ### prefers-reduced-motion
- `WCAG 2.2 New Criteria` — - **2.5.8:** Touch targets min 24x24 CSS px
- `Semantic HTML Reference` — ```html
- `Manual Checklist` — - [ ] Keyboard-only: Tab through entire page, verify focus order

## Related

- parent: `a11y-audit`

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
