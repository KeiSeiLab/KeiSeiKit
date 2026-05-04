---
title: validator_corpus.rs
path: kei-skills/tests/validator_corpus.rs
dna_hash: sha256:532d525a47a781c7
language: rust
size_loc: 98
generated: by-keidocs
---

# kei-skills/tests/validator_corpus.rs

20-sample invalid-corpus test. Each fixture documents WHICH
`IssueKind` it is meant to elicit; the test asserts that exactly that
kind appears in the issue list (additional issues OK — multiple
findings stack, see `validator::validate`).

## Related

- parent: `kei-skills/tests`
- imports: kei_skills, std

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
