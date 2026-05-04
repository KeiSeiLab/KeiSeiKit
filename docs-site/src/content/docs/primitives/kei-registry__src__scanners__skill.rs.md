---
title: skill.rs
path: kei-registry/src/scanners/skill.rs
dna_hash: sha256:ecaa35cfc47ba3aa
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-registry/src/scanners/skill.rs

Skill scanner — walks `<kit-root>/skills/*/SKILL.md`.

Constructor Pattern: this cube knows the SKILL.md convention only.
Body bytes = raw markdown; name = first H1 line stripped of leading
`# ` (fallback: directory name); caps = `md`.

## Public API

- `pub struct SkillScanner` — `<kit-root>/skills/<name>/SKILL.md` adapter.
- `pub fn scan_one_skill` — Scan a single skill directory directly (used by `register-skill`
- Extract the first H1 line (`# Title`) from markdown bytes. Returns None

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
