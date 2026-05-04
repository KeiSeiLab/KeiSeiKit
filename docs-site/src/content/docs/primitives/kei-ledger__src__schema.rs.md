---
title: schema.rs
path: kei-ledger/src/schema.rs
dna_hash: sha256:4a92b9cdc2d4ea36
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-ledger/src/schema.rs

SQL schema runner for the agent ledger.

Constructor Pattern: this cube is the runner; the DDL list lives in
the sibling [`crate::migrations_list`] module. Splitting keeps the
file under the 200-LOC ceiling now that v8 (skill_invocations) has
landed.

## Public API

- `pub const MAX_BRANCH_LEN` — Maximum length (chars) accepted for `branch` and `parent_branch` columns.
- Re-export the migration list for backward-compat. Existing callers
- `pub const SCHEMA_VERSION` — Schema version constant — index of the latest migration entry.
- Schema version the v5 pre-check guards. Kept as a named constant so the
- `pub fn migrate` — Apply all pending migrations atomically (one transaction per version).
- v5 pre-check — scan existing rows for duplicate non-NULL DNAs. If any
- Apply a single migration atomically: DDL + user_version bump in one txn.
- `pub const REQUIRED_ARTEFACTS` — Six required artefacts per agent (RULE 0.12 §completion bundle).

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: crate, rusqlite

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
