---
title: skill_extractor.rs
path: kei-import-project/src/skill_extractor.rs
dna_hash: sha256:27b9e8a90a85e7fb
language: rust
size_loc: 155
generated: by-keidocs
---

# kei-import-project/src/skill_extractor.rs

Phase 3 core: extract skills from a repo's documentation and register them.

Algorithm:
1. Walk doc paths via doc_walker.
2. For each file: split on H2 headings via md_splitter.
3. Filter fragments with meaningful heading (≥3 chars) + non-empty body.
4. Write SKILL.md to fragments_dir via fragment_writer.
5. Register as BlockType::Skill in kei-registry.

## Public API

- One extracted skill fragment.
- Aggregate outcome from a single extract_skills call.
- `pub fn extract_skills` — Extract skills from `repo_root` docs, writing to `fragments_dir`.
- Return the file stem (filename without extension) as a sanitized slug.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
