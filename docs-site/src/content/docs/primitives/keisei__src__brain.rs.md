---
title: brain.rs
path: keisei/src/brain.rs
dna_hash: sha256:e45a789218419bb1
language: rust
size_loc: 198
generated: by-keidocs
---

# keisei/src/brain.rs

Brain — portable exobrain directory representation.

A "brain" is a self-contained directory on any filesystem (USB, iCloud,
remote mount) attached to an AI client via the `keisei` CLI. It
declares its layout in a top-level `manifest.toml`.

Two schemas are supported:

* **v1** — single-string `mcp_server = "bin/kei-mcp-server-<os>-<arch>"`
(one brain per platform).
* **v2** — `[paths.mcp_server]` table keyed by `<os>-<arch>` so a
single brain on USB serves every host automatically.

# Invariants (audit-hardened, v0.19 + v0.20)

- **Path confinement** — every path under `[paths]` MUST be relative;
absolute paths and `..` components are rejected syntactically, and
the canonical form must remain inside the brain root
(`Error::PathEscape`). In schema v2 every map value is checked
independently.
- **Symlink reject** — the brain-root input itself cannot be a
symlink; the user must pass the canonical path to close the
USB → `$HOME` pivot (`Error::BrainIsSymlink`).
- **Name regex** — `^[a-z][a-z0-9_-]{0,63}$` on `brain.name`: lowercase
letter start, up to 64 chars, word chars + hyphen only
(`Error::InvalidName`).
- **Manifest size bound** — `manifest.toml` is capped at 64 KiB
(`brain_validate::MAX_MANIFEST_BYTES`); anything larger returns
`Error::ManifestTooLarge` before the toml parser sees a byte.
- **Schema range** — `schema_version ∈ {1, 2}` accepted (see
`MIN_SCHEMA..=MAX_SCHEMA`). v1 = single-string `mcp_server`; v2 =
`[paths.mcp_server]` map keyed by `<os>-<arch>`.

Platform key format (v2): derived from `std::env::consts` with renames
`macos → darwin`, `x86_64 → x64`, `aarch64 → arm64`. See
[`Brain::current_platform_key`]. A brain may ship only a subset of
platforms — the missing ones surface as [`Error::NoPlatformBinary`]
at `mcp_server_path()` call time, NOT at load time, so
`keisei status` can still inspect a partial brain.

Constructor Pattern: single responsibility — parse + compose the
validation primitives from `brain_validate.rs` into the load pipeline.

## Public API

- `pub const MIN_SCHEMA` — Lowest schema version understood by this binary.
- `pub const MAX_SCHEMA` — Highest schema version understood by this binary.
- v1 carries a single relative path; v2 carries a map keyed by
- Schema v1 form: single relative path good for one platform only.
- Schema v2 form: `{ "darwin-arm64": "bin/...", "linux-x64": "bin/..." }`.
- Required. Path(s) to the MCP server binary, relative to the brain root.
- Optional. If present, must be relative + in-root.
- Optional. If present, must be relative + in-root.
- Optional. If present, must be relative + in-root.
- `pub fn load` — Load a brain from `<root>/manifest.toml`.
- `pub fn current_platform_key` — Return the `<os>-<arch>` key used to look up v2 platform entries.
- `pub fn mcp_server_path` — Resolve the mcp_server binary for the current host and canonicalize

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
