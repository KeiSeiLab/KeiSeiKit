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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
