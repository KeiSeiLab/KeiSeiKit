---
title: patch.rs
path: kei-refactor-engine/src/patch.rs
dna_hash: sha256:c5e5a792391928c1
language: rust
size_loc: 131
generated: by-keidocs
---

# kei-refactor-engine/src/patch.rs

Auto-resolve plan writer.

v0.14.1 retraction: this module used to emit a `*.patch` file with
`--- a/<file>` / `+++ b/<file>` headers that *looked* like unified-diff
but had no real hunk bodies. `git apply --check` rejects that format.
The claim "git apply-ready patch" was incorrect.

New behaviour: we write a companion markdown file
(`plan-autoresolve.md`) listing the auto-apply candidates so the user
can review + apply them manually. File-content diffs would require
reading each source file, which is out of scope for this crate and
risks hallucinated edits (RULE 0.4). The "applied fork" path in
deep-sleep still produces a real branch via rename/move ops — those
are performed by the orchestrator, not by this file emitter.

Only items whose `resolution == AutoApply` are listed here; the
zero-conflict guarantee keeps `requires_human_decision` items out.

## Public API

- `pub fn write_autoresolve` — Write the auto-resolve review markdown. Returns the count of auto items.

## Related

- parent: `kei-refactor-engine/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
