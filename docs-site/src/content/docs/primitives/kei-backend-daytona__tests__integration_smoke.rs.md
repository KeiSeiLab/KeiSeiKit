---
title: integration_smoke.rs
path: kei-backend-daytona/tests/integration_smoke.rs
dna_hash: sha256:2c792fbc3a97aa26
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-backend-daytona/tests/integration_smoke.rs

End-to-end smoke test against the real Daytona service.

Skipped by default. To run:

```bash
export DAYTONA_API_KEY=...
export DAYTONA_BASE_URL=https://app.daytona.io/api
cargo test -p kei-backend-daytona --test integration_smoke -- --ignored --nocapture
```

The test acquires a sandbox keyed by a fixed task id, runs `echo hi`,
then **stops** (does NOT delete) so that the next run exercises the
resume-from-hibernated branch.

## Related

- parent: `kei-backend-daytona/tests`
- imports: kei_backend_daytona

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
