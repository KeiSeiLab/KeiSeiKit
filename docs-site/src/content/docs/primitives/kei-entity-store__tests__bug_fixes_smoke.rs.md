---
title: bug_fixes_smoke.rs
path: kei-entity-store/tests/bug_fixes_smoke.rs
dna_hash: sha256:1245ec4b03c3e8f5
language: rust
size_loc: 480
generated: by-keidocs
---

# kei-entity-store/tests/bug_fixes_smoke.rs

Regression tests for post-convergence audit findings (C1/C2/FTS5
injection/M3/TEXT-cap/M2). Each test names the finding it pins.

## Public API

- FTS + archived_field both configured — required for
- No FTS — required for `delete_succeeds_when_no_fts_configured`.

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
