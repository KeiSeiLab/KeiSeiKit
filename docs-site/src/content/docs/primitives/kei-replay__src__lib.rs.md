---
title: lib.rs
path: kei-replay/src/lib.rs
dna_hash: sha256:62269f8ded863328
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-replay/src/lib.rs

kei-replay — reconstruct an agent spawn from its DNA string.

Given a DNA `role::caps::scope::body-nonce`, look up the ledger row,
locate the archived `task.toml` for that agent, re-run the compose
pipeline, and compare the resulting body hash to the DNA's body segment.
A mismatch is schema drift since the original spawn.

Constructor Pattern: one responsibility per cube. No I/O beyond SQLite
read + `std::fs` on task files + stdout.

Modules:
- `replay`        — reconstruct composed prompt from DNA
- `diff`          — compare two DNAs (facets + bodies)
- `ledger_lookup` — SQLite direct read of ledger rows by DNA

## Related

- parent: `kei-replay/Cargo.toml`

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
