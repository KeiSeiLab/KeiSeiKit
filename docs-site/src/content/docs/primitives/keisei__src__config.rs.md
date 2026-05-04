---
title: config.rs
path: keisei/src/config.rs
dna_hash: sha256:f005fcefc6eaec45
language: rust
size_loc: 197
generated: by-keidocs
---

# keisei/src/config.rs

SSoT for the active attach: `~/.keisei/attached.toml` (v0.22+).

Schema v4 (v0.22, multi-brain per marker):
```toml
schema_version = 4

[[attachments]]
brain_path  = "/Volumes/Brain1"
brain_name  = "brain-a"
client_type = "claude-code"
config_path = "/Users/me/.claude/settings.json"
scope       = "user"
attached_at = "2026-04-22T14:23:00Z"
```

Older schemas (v1/v2/v3) still read transparently — migrated in-memory
to v4 on first `read()` (see `config_migrate.rs`). One-line stderr
notice fires so operators see the shape flip. Location migration
(v0.20 legacy path `~/.claude/keisei-attached.toml` → v0.21 path
`~/.keisei/attached.toml`) happens in the same pass.

Constructor Pattern: single responsibility — read/write the attach
marker + one-shot location migration. Schema migration lives in
`config_migrate.rs`; time helpers in `time.rs`.

Testability: `$KEISEI_HOME` overrides `$HOME` so integration tests
isolate state per tmpdir.

## Public API

- `pub const ATTACHED_FILENAME` — v0.21+ filename. The marker lives at `$KEISEI_HOME/.keisei/attached.toml`.
- `pub const LEGACY_ATTACHED_FILENAME` — Legacy (v0.20 and earlier) filename, under `$KEISEI_HOME/.claude/`.
- `pub const CURRENT_SCHEMA` — Current on-disk schema version.
- A single brain ⇄ client attachment. v4 pulls `brain_path`, `brain_name`,
- `pub fn keisei_state_dir` — Keisei's state directory — `$KEISEI_HOME/.keisei/`.
- `pub fn attached_path` — Current marker path (v0.21+): `$KEISEI_HOME/.keisei/attached.toml`.
- `pub fn legacy_attached_path` — Legacy marker path (v0.20 and earlier).
- `pub fn read` — Read the marker, performing one-shot v0.20→v0.21 location migration if
- `pub fn migrate_from_legacy` — If `~/.claude/keisei-attached.toml` exists AND `~/.keisei/attached.toml`
- On unix, restrict the marker to owner-only (0o600). No-op on windows.
- `pub fn now_utc_string` — Thin re-export so call-sites elsewhere in the crate don't have to

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, serde, std

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
