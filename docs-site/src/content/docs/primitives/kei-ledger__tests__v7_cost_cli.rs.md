---
title: v7_cost_cli.rs
path: kei-ledger/tests/v7_cost_cli.rs
dna_hash: sha256:c0b29f9d4f39f640
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-ledger/tests/v7_cost_cli.rs

v7 CLI binary tests for `record-cost` (Wave 44c, 2026-04-24).

Constructor Pattern: extracted from `v6_cost.rs` so each test file
stays under the 200-LOC ceiling. Loads source modules via `#[path]`
to avoid forcing all callers through the public lib API.

## Public API

- v6-T6: `record-cost` CLI subcommand round-trips a real binary build.
- v7-T6b (Wave 44c): `record-cost` CLI defaults to ADDITIVE; three
- v7-T6c: `--replace` flag restores last-write-wins behavior for
- v6-T7: `record-cost` CLI on a missing agent prints to stderr and exits 1.

## Related

- parent: `kei-ledger/tests`
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
