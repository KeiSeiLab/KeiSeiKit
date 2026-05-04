---
title: main.rs
path: mock-render/src/main.rs
dna_hash: sha256:c96f72235d2fd2d6
language: rust
size_loc: 55
generated: by-keidocs
---

# mock-render/src/main.rs

mock-render — enforces the WYSIWYD invariant (What You See Is What's Deployed)
for block-based site-builder. Every section = one source file; screenshot is
a render of that file; lock freezes the hash; verify fails if source mutated.

USAGE
mock-render screenshot <url> --out <png> [--viewport WxH]
mock-render lock    --project <dir> --section <src> [--screenshot <png>]
mock-render verify  --project <dir> --section <src>
mock-render status  --project <dir>

## Related

- parent: `mock-render/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
