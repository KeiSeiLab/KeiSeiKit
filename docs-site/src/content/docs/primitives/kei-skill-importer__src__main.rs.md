---
title: main.rs
path: kei-skill-importer/src/main.rs
dna_hash: sha256:dac6ac8adfe65840
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-skill-importer/src/main.rs

kei-skill-importer CLI — `parse` (JSON), `convert` (write file),
`batch` (walk + convert; JSONL summary). Info logs go to stderr;
stdout is reserved for machine-readable output.

## Public API

- Parse a skill file and print canonical JSON to stdout.
- Parse + decide emit path + write file(s) into <output_dir>.
- Walk <input_dir> and convert every candidate file.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, clap, kei_skill_importer, serde, std, walkdir

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
