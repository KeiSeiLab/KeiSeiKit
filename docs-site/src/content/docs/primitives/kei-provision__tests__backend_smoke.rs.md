---
title: backend_smoke.rs
path: kei-provision/tests/backend_smoke.rs
dna_hash: sha256:39c1e4e48d824823
language: rust
size_loc: 264
generated: by-keidocs
---

# kei-provision/tests/backend_smoke.rs

Smoke tests for kei-provision backends.

Strategy: no real cloud calls. We inject a tempdir onto PATH containing
fake `hcloud` / `vultr-cli` shell scripts that echo canned JSON matching
the real v1 / v3 CLI output shapes. The Backend impls then parse these
exactly as they would production output.

## Public API

- Create a fake CLI script at `<dir>/<bin>` that emits `stdout` verbatim
- Install a fake that always exits non-zero (simulates "server absent").
- Install a fake that emits `stderr_text` to stderr then exits non-zero.
- Prepend tempdir to PATH so the fake binary wins, but keep the rest of

## Related

- parent: `kei-provision/tests`
- imports: kei_provision, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
