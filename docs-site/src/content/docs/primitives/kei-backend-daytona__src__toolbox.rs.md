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
