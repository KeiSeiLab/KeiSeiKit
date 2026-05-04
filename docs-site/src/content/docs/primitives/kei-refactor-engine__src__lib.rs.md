---
title: lib.rs
path: kei-refactor-engine/src/lib.rs
dna_hash: sha256:d6b79386f2010cf5
language: rust
size_loc: 17
generated: by-keidocs
---

# kei-refactor-engine/src/lib.rs

kei-refactor-engine — library surface.

Consumes `kei-conflict-scan` JSON; produces a structured refactor plan
(markdown) and, optionally, an auto-resolve review markdown
(NOT a unified diff — see patch.rs header, v0.14.1 retraction).

Zero-conflict guarantee: any conflict whose `auto_resolvable = false`
is included in the plan under "Requires human decision" and EXCLUDED
from the auto-resolve markdown.

## Related

- parent: `kei-refactor-engine/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
