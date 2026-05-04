---
title: cli.rs
path: kei-leak-matrix/src/cli.rs
dna_hash: sha256:d99ffbcb1a5e6c46
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-leak-matrix/src/cli.rs

CLI surface — clap structs for every subcommand.

Parsing only. Side-effects (file I/O, stdout, exit codes) live in
main.rs dispatch + the scanner / substituter / matrix modules.

## Public API

- Override matrix path (default: $KEI_LEAK_MATRIX_PATH or
- Scan a single file; emit JSON violations.
- Recurse a directory; aggregate violations as JSON.
- Read stdin, apply substitute-severity rules, write to stdout.
- Test if a candidate pattern is already covered by a rule.
- List all rules in markdown table form.
- Scan a literal command string (for hook integration).

## Related

- parent: `kei-leak-matrix/Cargo.toml`
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
