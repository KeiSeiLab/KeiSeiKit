---
title: file_sync_wiring.rs
path: kei-backend-daytona/tests/file_sync_wiring.rs
dna_hash: sha256:dbc68a6f3712ca34
language: rust
size_loc: 275
generated: by-keidocs
---

# kei-backend-daytona/tests/file_sync_wiring.rs

Verify `DaytonaBackend::with_sync(...)` actually wires file-sync into
`acquire` (push) and `release` (pull).

These tests use `wiremock` to assert that the expected REST endpoints
are invoked in the right order. They do NOT exercise the underlying
Daytona service.

Architecture note (post Patch B):
- File operations go through the Toolbox API, not the management API.
- `GET /sandbox/{name}/toolbox-proxy-url` returns `{"url":"<toolbox_base>"}`.
- Upload:   POST <toolbox_base>/toolbox/{name}/toolbox/files/upload?path=<p>
- Download: GET  <toolbox_base>/toolbox/{name}/toolbox/files/download?path=<p>

In tests the toolbox proxy URL points back at the same wiremock server,
so all mocks live on a single MockServer instance.

## Public API

- Seed `local_root` with a single file so push has work to do.
- Register a mock that returns the wiremock server's own URI as the toolbox

## Related

- parent: `kei-backend-daytona/tests`
- imports: kei_backend_daytona, serde_json, std, tempfile, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
