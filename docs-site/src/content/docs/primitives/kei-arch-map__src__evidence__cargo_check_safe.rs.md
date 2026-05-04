---
title: cargo_check_safe.rs
path: kei-arch-map/src/evidence/cargo_check_safe.rs
dna_hash: sha256:fa62019c5a6c752a
language: rust
size_loc: 176
generated: by-keidocs
---

# kei-arch-map/src/evidence/cargo_check_safe.rs

Wave 4 #50C/#53 — whitelisted `cargo check` evidence kind.

`CargoCheckSafe` runs `cargo check --workspace --offline --message-format=json`
ONLY on workspaces whose `manifest_dir` (relative to repo root) appears in
`allowed_paths`. External / untrusted manifests are refused with FAIL.

⚠ build.rs RCE: `cargo check` compiles + runs build scripts. The allowlist
exists so a Plan author can declare "I trust this workspace" explicitly.
For untrusted manifests use `CargoCheckClean` (manifest-validate only).

## Public API

- Verify `manifest_dir` (relative to root, after canonicalize) appears in
- `pub fn check` — Entry point. Runs `cargo check --workspace` on `manifest_dir` ONLY if

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: serde, std, wait_timeout

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
