---
title: recall.rs
path: kei-pet/src/recall.rs
dna_hash: sha256:b2aae43f5da0e495
language: rust
size_loc: 94
generated: by-keidocs
---

# kei-pet/src/recall.rs

Conversational recall — "have we discussed this before?"

Thin adapter over `kei_dna_index::precedent`. Hashes a task body with
SHA-256, truncates to the first 4 bytes (8 hex chars — matches the DNA
`body_sha` width SSoT in `kei_shared::dna`), then asks the ledger for
past agents whose DNA carries the same body_sha.

Scope: reads the `agents` table on the supplied `Connection`. No writes,
no schema mutation. Caller decides whether the connection points at the
real ledger or a test fixture.

## Public API

- One past agent whose DNA body_sha matches the current task body.
- Errors surfaced by recall.
- `pub fn body_sha8` — SHA-256 of `task_body`, truncated to the first 4 bytes rendered as
- First 80 characters of `task_body`, respecting UTF-8 char boundaries.
- Fetch `started_ts` for a given agent_id. Returns 0 when the row is gone
- `pub fn recall_similar` — Find up to `limit` past agents whose DNA body_sha matches the hash of

## Related

- parent: `kei-pet/Cargo.toml`
- imports: kei_dna_index, rusqlite, sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
