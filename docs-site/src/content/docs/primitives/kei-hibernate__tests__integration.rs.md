---
title: integration.rs
path: kei-hibernate/tests/integration.rs
dna_hash: sha256:ffb54e67f34990a5
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-hibernate/tests/integration.rs

Core integration tests — the 6 cases named in the spec.

1. export_round_trip
2. export_excludes_non_kei_files
3. import_dry_run_makes_no_changes
4. import_refuses_version_mismatch
5. inspect_lists_contents
6. manifest_sha256_verified_on_import

## Public API

- Defence-in-depth: our importer rejects archive entries containing

## Related

- parent: `kei-hibernate/tests`
- imports: common, kei_hibernate, std, tempfile

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
