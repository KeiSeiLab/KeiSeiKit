---
title: legacy_schema.rs
path: kei-brain-view/tests/legacy_schema.rs
dna_hash: sha256:26f0f0287052f3d2
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-brain-view/tests/legacy_schema.rs

Legacy-schema compat: brain-view must handle a pre-v2 ledger that
lacks the `dna` / `creator_id` / `fork_parent_id` columns. Separate
test file so the main integration suite stays focused.

## Related

- parent: `kei-brain-view/tests`
- imports: kei_brain_view, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
