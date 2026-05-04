---
title: glob_tool.rs
path: kei-cortex/src/tool/glob_tool.rs
dna_hash: sha256:846b6be338c9490d
language: rust
size_loc: 154
generated: by-keidocs
---

# kei-cortex/src/tool/glob_tool.rs

`glob` tool — file discovery by glob pattern.

Composition: walkdir over the search root → match each entry against
the pattern (translated to a `Regex`) → sort by mtime descending →
cap at 100 results. Returns one path per line.

Sandbox: when an absolute `path` is supplied, it must resolve INSIDE
`project_root` (canonicalised). When omitted, the search root is
`project_root` itself.

Pattern syntax: standard shell globs — `*` matches any chars except
`/`, `**` matches any chars including `/`, `?` matches any single
char, `[abc]` matches a character class.

## Public API

- Walk `root`, return up to `MAX_RESULTS` paths matching `regex`,
- Translate a glob into an anchored regex.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: regex, serde, serde_json, std, tempfile, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
