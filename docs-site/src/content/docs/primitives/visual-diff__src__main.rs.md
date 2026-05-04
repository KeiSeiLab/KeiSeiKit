---
title: main.rs
path: visual-diff/src/main.rs
dna_hash: sha256:e591b08f705cf29d
language: rust
size_loc: 83
generated: by-keidocs
---

# visual-diff/src/main.rs

visual-diff — pixel-level PNG comparator for WYSIWYD drift detection.

USAGE
visual-diff <a.png> <b.png> [--out diff.png] [--threshold 5]

Exit codes:
0  images equal (within threshold)
1  usage error
2  images differ beyond threshold

Prints percentage of mismatched pixels to stdout. Writes a red-overlay
diff PNG to <out> (default: ./diff.png) when images differ.

## Related

- parent: `visual-diff/Cargo.toml`
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
