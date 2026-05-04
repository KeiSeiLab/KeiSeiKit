---
title: golden.rs
path: kei-export-trajectories/tests/golden.rs
dna_hash: sha256:6983f8890e67c511
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-export-trajectories/tests/golden.rs

Golden-file test: insert two synthetic agents into an in-memory
ledger + memory database, run the export library through its public
API, compare emitted JSONL line-by-line against a checked-in
fixture.

Constructor Pattern: fixture builders live in `tests/common/`.

## Related

- parent: `kei-export-trajectories/tests`
- imports: kei_export_trajectories, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
