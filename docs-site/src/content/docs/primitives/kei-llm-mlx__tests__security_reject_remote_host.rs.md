---
title: security_reject_remote_host.rs
path: kei-llm-mlx/tests/security_reject_remote_host.rs
dna_hash: sha256:a42c38c8de10189c
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-llm-mlx/tests/security_reject_remote_host.rs

Security — `server --host 0.0.0.0` MUST be rejected.

`mlx_lm.server` defaults to localhost, but a careless user could
type `--host 0.0.0.0` to bind on every interface. This primitive
refuses with `Error::SecurityRefused` so a public bind requires an
explicit operator-level decision (config + audit), not a CLI typo.

On non-target builds the platform gate fires first with NotSupported;
the test branches accordingly so it stays green on every host.

## Related

- parent: `kei-llm-mlx/tests`
- imports: kei_llm_mlx

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
