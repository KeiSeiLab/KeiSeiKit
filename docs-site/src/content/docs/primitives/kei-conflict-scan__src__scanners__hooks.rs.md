---
title: hooks.rs
path: kei-conflict-scan/src/scanners/hooks.rs
dna_hash: sha256:508d9a70996e81e3
language: rust
size_loc: 67
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/hooks.rs

Hook-overlap detector.

Heuristic: two hook scripts in `hooks/` whose first line-match of
`tool_name|matcher|event|PreToolUse|PostToolUse|UserPromptSubmit`
targets the same value. Flags the pair as possibly-redundant.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
