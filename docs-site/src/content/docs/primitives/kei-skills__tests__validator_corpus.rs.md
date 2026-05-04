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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
