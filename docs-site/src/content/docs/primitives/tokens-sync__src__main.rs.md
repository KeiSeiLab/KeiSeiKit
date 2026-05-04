---
title: main.rs
path: tokens-sync/src/main.rs
dna_hash: sha256:6878d5f383126232
language: rust
size_loc: 85
generated: by-keidocs
---

# tokens-sync/src/main.rs

tokens-sync — emit Tailwind config + CSS custom properties from a single
design-tokens JSON file. One SSoT; no drift between CSS/JS sides.

USAGE
tokens-sync <tokens.json> --out-tailwind <path> --out-css <path>

Input JSON shape (minimum):
{
"colors":   { "primary": "oklch(0.6 0.2 250)", ... },
"fonts":    { "display": "Fraunces Variable, serif", ... },
"spacing":  { "sm": "0.5rem", ... },
"radius":   { "card": "0.75rem", ... }
}

At least one of --out-tailwind or --out-css must be supplied.

## Related

- parent: `tokens-sync/Cargo.toml`
- imports: std

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
