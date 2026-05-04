---
title: ssh.rs
path: kei-compute-baremetal/src/ssh.rs
dna_hash: sha256:f22cf1ff3c24abba
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-compute-baremetal/src/ssh.rs

Minimal async SSH wrapper. Shells out to the system `ssh` binary —
no Rust SSH client linked in. Two operations only: `ping` (returns
Ok if remote shell exits 0) and `run_remote` (returns stdout).

`binary` field exists so unit tests can swap `ssh` for `echo` without
reaching the network. Production callers always use the default `ssh`.

## Public API

- `pub fn is_safe_user` — Validate SSH username: alphanumeric + `-_.`, not starting with `-`, max 64 chars.
- `pub fn is_safe_host` — Validate SSH hostname: alphanumeric + `-_.`, not starting with `-`, max 64 chars.
- SSH endpoint. `port` is optional; default is 22 when None.
- Override the binary name. Production: `"ssh"`. Tests: `"echo"`.
- Probe reachability — runs `echo ok` over SSH. Ok iff exit 0.
- Run an arbitrary shell snippet on the remote box. Returns combined

## Related

- parent: `kei-compute-baremetal/Cargo.toml`
- imports: crate, tokio

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
