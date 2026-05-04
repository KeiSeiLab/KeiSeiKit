---
title: toolbox.rs
path: kei-backend-daytona/src/toolbox.rs
dna_hash: sha256:52d31844f3c2f15a
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-backend-daytona/src/toolbox.rs

Toolbox API helpers — exec, file upload/download via the per-sandbox proxy.

The Daytona Toolbox API is a separate HTTP surface from the management API.
Its base URL is fetched lazily via `GET /sandbox/{id}/toolbox-proxy-url`
(management API) and cached per sandbox ID.

Spec endpoints used:
POST <toolbox_base>/toolbox/{id}/toolbox/process/execute
POST <toolbox_base>/toolbox/{id}/toolbox/files/upload?path=<p>
GET  <toolbox_base>/toolbox/{id}/toolbox/files/download?path=<p>

## Public API

- Response schema for `GET /sandbox/{id}/toolbox-proxy-url`.
- Per-sandbox toolbox URL cache.
- Resolve the toolbox base URL for `sandbox_id`.
- Execute a command in the sandbox via the Toolbox API.
- Upload a file to the sandbox via the Toolbox API.
- Download a file from the sandbox via the Toolbox API.
- Check HTTP status and convert errors.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: crate, reqwest, serde, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
