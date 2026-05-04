---
title: config.rs
path: kei-store/src/config.rs
dna_hash: sha256:a4abe75ea2911d57
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-store/src/config.rs

TOML config loader.

Example `store-config.toml`:

```toml
[active]
backend = "github"

[github]
url = "git@github.com:user/memory-repo.git"
ssh_key_env = "KEI_MEMORY_SSH_KEY"

[filesystem]
path = "~/.claude/memory/sync-repo"
```

Secrets (PATs, SSH keys) live in `~/.claude/secrets/.env` per RULE 0.8;
this file only stores env-var NAMES.

## Public API

- Local cache / manifest root. REQUIRED — S3 impl stores a manifest

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
