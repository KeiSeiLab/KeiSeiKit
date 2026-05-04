---
title: registry.rs
path: kei-agent-runtime/src/registry.rs
dna_hash: sha256:fd785b0b3e708161
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-agent-runtime/src/registry.rs

Registry — `&str → &'static dyn Capability` lookup for all 14
capability implementations.

`get(name)` is the single dispatch point used by both the
`kei-agent-runtime verify` binary and the `kei-capability` hook adapter.

## Aliases (v0.17)

Two capabilities were renamed in v0.17 for clarity. Their old names
still resolve here via a small alias table; a deprecation warning is
emitted to stderr on lookup (once per process via `OnceLock`).

- `tools::read-only` → `tools::deny-tools`
- `tools::cargo-only-bash` → `tools::bash-allowlist`

Alias resolution is transparent: `get()` / `get_gate()` / `get_verify()`
return the new implementation when queried with the old name. The new
name is what the impl reports via `Capability::name()`.

## Convergence wave v0.18

5 of 6 gates + 3 of 8 verifies are now `const PatternGate { … }` /
`const CommandVerify { … }` declarations. Registry points at the
const by reference (`&POLICY_NO_GIT_OPS_GATE`) — same `&'static dyn Capability`
dispatch shape as before.

## Public API

- Alias table — (old name → new name). Checked before every resolution.
- Resolve an alias (if any) and emit a one-shot deprecation warning.
- Log a deprecation warning to stderr at most once per (old, new) pair
- `pub fn get` — Look up a capability by its canonical `<category>::<slug>` name.
- `pub fn get_gate` — Look up only the gate-side impl. Used by `kei-capability check`.
- `pub fn get_verify` — Look up only the verify-side impl. Used by `kei-capability verify`.
- Gate-only lookup by canonical name (no alias resolution, no warning).
- Verify-only lookup by canonical name (no alias resolution, no warning).
- `pub fn all_names` — All known canonical capability names (union of gate + verify). Used by
- `pub fn deprecated_aliases` — List of (old-name, new-name) pairs still honored as aliases. Used by

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, std

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
