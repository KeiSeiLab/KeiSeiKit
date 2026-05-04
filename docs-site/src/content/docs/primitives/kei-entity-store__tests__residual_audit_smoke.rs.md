---
title: residual_audit_smoke.rs
path: kei-entity-store/tests/residual_audit_smoke.rs
dna_hash: sha256:bfc1e57a154cf01c
language: rust
size_loc: 364
generated: by-keidocs
---

# kei-entity-store/tests/residual_audit_smoke.rs

Residual audit regression tests (Wave 14, 2026-04-23).

Each block names the residual it pins:
* A — ddl.rs panic-free across every FieldKind variant
* B — update.rs FTS reindex non-input-column invariant
* C — engine.rs WAL pragma fallback on a read-only FS
* D — search.rs has_searchable_token Unicode edge cases

Scope: kei-entity-store only. No workspace / cross-crate changes.

## Public API

- Compile-time exhaustive match over `FieldKind`. If a new variant is
- Debug-only: if a future BEFORE UPDATE trigger mutates an FTS column

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, serde_json, std

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
