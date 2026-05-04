---
title: main.rs
path: kei-conflict-scan/src/main.rs
dna_hash: sha256:958fa0dc6d8e5fc2
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-conflict-scan/src/main.rs

kei-conflict-scan — binary entry point.

See lib.rs for overview. CLI spec:
kei-conflict-scan --path <root> [--format json|human] [--only rules|hooks|blocks|orphans|cp]

Exit codes:
0 — scan completed (hits or no hits)
1 — usage / I/O error
2 — hits found AND --exit-on-hit set

## Public API

- Root directory to scan (e.g. ~/.claude or a cloned memory repo).
- Output format.
- Only run one category; default = run all.
- Exit 2 if any conflict is reported.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: clap, kei_conflict_scan, std

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
