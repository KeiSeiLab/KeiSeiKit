---
title: main.rs
path: kei-auth/src/main.rs
dna_hash: sha256:afa0bd6817e9560c
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-auth/src/main.rs

kei-auth CLI — issue/verify/revoke.

v0.14.1 security fix: the `--key` CLI flag was removed because it
leaked the HMAC signing secret through `/proc/<pid>/cmdline` and
shell history. The only supported key source is the `KEI_AUTH_KEY`
env var (sourced from `~/.claude/secrets/.env` per RULE 0.8).

Token argument: pass `-` or set `KEI_AUTH_TOKEN` env var to avoid
leaking tokens via shell history or `/proc/<pid>/cmdline`.

## Public API

- Read token from env `KEI_AUTH_TOKEN`, or from stdin when arg is `-`,

## Related

- parent: `kei-auth/Cargo.toml`
- imports: clap, kei_auth, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
