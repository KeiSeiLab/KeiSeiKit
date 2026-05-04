---
title: github.rs
path: kei-store/src/github.rs
dna_hash: sha256:5d1004202fae152c
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-store/src/github.rs

GitHubStore — git-over-SSH/HTTPS.

Wraps FilesystemStore for local ops, adds push/pull to a configured
remote. SSH auth via `KEI_MEMORY_SSH_KEY` (path to key); HTTPS via
`KEI_MEMORY_PAT` (token). Exactly the pattern used in v0.11
`kei-sleep-setup.sh`.

v0.14.1: pushes to `github.com` are blocked by default under RULE 0.1
(patent-IP protection). Forks on Forgejo / Gitea / self-hosted are
unaffected since they do not resolve to `github.com`. Override for a
genuinely public repo: `KEI_STORE_ALLOW_GITHUB_PUSH=1`.

## Public API

- RULE 0.1 enforcement point for the kei-store push path.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
