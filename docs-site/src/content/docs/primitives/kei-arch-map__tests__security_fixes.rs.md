---
title: security_fixes.rs
path: kei-arch-map/tests/security_fixes.rs
dna_hash: sha256:76e780b2f3a21515
language: rust
size_loc: 156
generated: by-keidocs
---

# kei-arch-map/tests/security_fixes.rs

Regression tests for Wave 4 security fixes (Task #50 A–F).

Mirror of `tests/evidence.rs` style — uses the same `support` module
to keep call-sites short. Each #[test] points at the specific fix it
protects so a future regression is traceable to its origin task.

## Public API

- Fix A: hostnames must be DNS-resolved and EVERY resolved address checked
- Fix C: `cargo metadata` (no build.rs run) replaces `cargo check`. Empty
- Fix D: `..` traversal must not let a claim reach above repo root.
- Fix E: actual JSON value must NOT appear verbatim in the FAIL reason.
- Fix #50C/#53: `CargoCheckSafe` refuses to run on a manifest whose path
- Fix #50C/#53: when `manifest_dir` matches an allowlist entry, the
- Fix #50B: cargo binary is resolved to an ABSOLUTE path via `which`.

## Related

- parent: `kei-arch-map/tests`
- imports: std, support, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
