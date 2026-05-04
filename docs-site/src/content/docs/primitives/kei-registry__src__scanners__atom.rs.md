---
title: atom.rs
path: kei-registry/src/scanners/atom.rs
dna_hash: sha256:1da52a8efa538681
language: rust
size_loc: 127
generated: by-keidocs
---

# kei-registry/src/scanners/atom.rs

Atom scanner — walks markdown files in `<kit-root>/` recursively and
picks out files whose YAML frontmatter declares `type: atom`.

Constructor Pattern: this cube knows only the atom frontmatter
convention. The minimal regex parser deliberately does NOT pull in
kei-atom-discovery — that crate is not yet a workspace dependency
here, and a 30-LOC regex pair covers the `type:` and `name:` keys we
need.

## Public API

- `pub struct AtomScanner` — Recursive markdown walker that filters on YAML frontmatter `type: atom`.
- Pull the contents between `---` fences at the start of the file. Returns
- Minimal YAML-key extraction: `^<key>:\s*(.*)$` per line. Strips quotes

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, regex, std, walkdir

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
