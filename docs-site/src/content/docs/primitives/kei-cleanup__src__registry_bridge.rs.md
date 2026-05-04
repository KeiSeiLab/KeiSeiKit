---
title: registry_bridge.rs
path: kei-cleanup/src/registry_bridge.rs
dna_hash: sha256:229f02f079cce280
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-cleanup/src/registry_bridge.rs

Bridge: cleanup findings → kei-registry `cleanup_findings` table.

For each [`Finding`] in a [`CleanupReport`], inserts one row in the
`cleanup_findings` table of the kei-registry SQLite DB. The schema
is owned by kei-registry (migration v5; see Track D); this bridge
only writes — it does NOT create or migrate the table.

Schema (kei-registry v5):
```sql
CREATE TABLE cleanup_findings (
id INTEGER PRIMARY KEY,
workspace_sha TEXT NOT NULL,
timestamp_unix INTEGER NOT NULL,
finding_json TEXT NOT NULL,
severity TEXT NOT NULL,
kind TEXT NOT NULL
) STRICT;
```

NOTE: this code returns `Err` cleanly if the `cleanup_findings`
table is not yet present (Track D ships the migration). Callers
treat that as a soft failure and skip predicate emission.

## Public API

- Bridge-specific error type — kept narrow so callers can match.
- SQLite open / prepare / execute error.
- JSON serialisation failure.
- `pub fn emit_to_registry` — Insert one row per finding into `cleanup_findings`. Returns the
- `pub fn predicate_kind` — Map FindingKind → predicate kind string used by kei-registry.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: chrono, crate, rusqlite, std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
