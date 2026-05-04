---
title: runner.rs
path: kei-arch-map/src/runner.rs
dna_hash: sha256:b315c1681268fb74
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-arch-map/src/runner.rs

Verify-run orchestrator. Dispatches each Claim to the matching
`evidence::*::check` function, prints a per-claim PASS/FAIL line,
and returns Err on any FAIL.

## Public API

- `pub fn run` — Run all claims in `plan_path`. Err if any FAIL.
- `pub fn check_claim` — Check a single claim. Returns (passed, reason_if_failed).

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, kei_arch_map, std

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
