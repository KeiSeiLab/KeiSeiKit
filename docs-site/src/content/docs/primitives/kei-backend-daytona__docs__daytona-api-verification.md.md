---
title: daytona-api-verification
path: kei-backend-daytona/docs/daytona-api-verification.md
dna_hash: sha256:6348b66ca876d12e
language: markdown
size_loc: 65
generated: by-keidocs
---

# kei-backend-daytona/docs/daytona-api-verification.md

## Public API

- `Daytona REST API Verification (2026-04-30)` — Source: https://raw.githubusercontent.com/daytonaio/daytona/main/libs/api-client-go/api/openapi.yaml
- `client.rs path coverage` — | client.rs call | spec path | match | notes |
- `Mismatches summary` — 1. **Global prefix wrong**: all `client.rs` CRUD calls use `/sandboxes` (plural) but the spec uses `/sandbox` (singular) throughout.
- `Conclusion` — 7 of 9 client.rs calls have wrong paths (5 require only the `sandboxes` → `sandbox` rename; 2 require full architectural rework because exec and file operations use a separate toolbox API endpoint, not the management API).
- `Suggested patches` — ### Patch A — Global rename: `/sandboxes` → `/sandbox` (safe, mechanical)

## Related

- parent: `kei-backend-daytona/docs`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
