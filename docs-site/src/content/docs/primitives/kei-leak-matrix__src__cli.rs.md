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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
