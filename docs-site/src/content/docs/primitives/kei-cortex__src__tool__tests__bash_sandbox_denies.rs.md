---
title: bash_sandbox_denies.rs
path: kei-cortex/src/tool/tests/bash_sandbox_denies.rs
dna_hash: sha256:39ca8585a896ed04
language: rust
size_loc: 306
generated: by-keidocs
---

# kei-cortex/src/tool/tests/bash_sandbox_denies.rs

Validates the bash tool's tokenizer-based deny-list:
- root rm and root-glob rm are blocked (raw substring layer)
- github push attempts are blocked
- sudo / doas / pkexec are blocked even via quote/escape/cmdsub bypass
- pipe-to-shell remote-exec is blocked
- writes to system dirs (/etc, /var, /usr, /bin, /sbin, /boot) are blocked
- chmod 777 / chown root are blocked
- argv0 not on allow-list is rejected (default-deny gate)
- safe commands pass through

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate

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
