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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
