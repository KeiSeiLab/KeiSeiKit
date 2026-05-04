---
title: main.rs
path: ssh-check/src/main.rs
dna_hash: sha256:bd2ca5236d508217
language: rust
size_loc: 102
generated: by-keidocs
---

# ssh-check/src/main.rs

ssh-check — pre-deploy sshd_config linter for KeiSeiKit.

Reads /etc/ssh/sshd_config + every /etc/ssh/sshd_config.d/*.conf (or
user-supplied paths), merges directives via last-wins precedence, and
reports violations of the hardened-baseline rule matrix.

USAGE
ssh-check                                    # default system paths
ssh-check --config /etc/ssh/sshd_config --drop-in /etc/ssh/sshd_config.d
ssh-check --json                             # JSON output for CI
ssh-check --allow-user admin                 # extra allowed user

EXIT
0  no violations
1  usage / parse error
2  violations found

## Public API

- Main sshd_config file.
- Drop-in directory (sshd_config.d). Pass empty string to skip.
- Usernames that are acceptable in AllowUsers (repeatable).
- Emit JSON instead of human text.

## Related

- parent: `ssh-check/Cargo.toml`
- imports: clap, std

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
