---
title: rig_clone.rs
path: kei-cortex/src/rig_clone.rs
dna_hash: sha256:91be14c308f3b60c
language: rust
size_loc: 263
generated: by-keidocs
---

# kei-cortex/src/rig_clone.rs

Clone a bundled Cubism sample rig into a per-user directory and swap the
primary face texture.

Pure filesystem utility: no network, no image decoding. The Flux-generated
PNG bytes are written verbatim to `<target>/<textures>/texture_00.png`;
the hair/secondary texture (`texture_01.png` if present) is copied from
the base rig unchanged so the mesh UV layout continues to resolve.

Install is atomic-ish: we stage into `<target>.tmp/` and then rename.
Same-device rename is a single inode flip (cannot be seen partially by
a reader); cross-device falls back to "remove old, rename staged". A
concurrent second writer is prevented by the per-user mutex the caller
holds (see `handlers/portrait.rs`); this module is TOCTOU-safe only
under that contract.

## Public API

- Upper bound on the directory depth we are willing to walk looking for
- Errors from the clone/swap operation. `io::Error` is the common case;
- `pub fn clone_and_swap` — Clone `base_dir` to `target_dir`, then overwrite the primary face texture
- `pub fn install_rig` — Atomic install: stage into `<target>.tmp`, then rename onto the final
- Rename `staging` onto `target`, deleting `target` first if it exists.
- Detect EXDEV ("cross-device link") on Linux/macOS. `ErrorKind::CrossesDevices`
- Derive the staging path for an atomic install. Kept as a sibling of the
- Recursively copy every entry from `src` into `dst`. Regular files are
- Find the first `texture_00.png` anywhere under `root`. Walks via

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tempfile, walkdir

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
