---
title: fleet.rs
path: kei-pet/src/fleet.rs
dna_hash: sha256:262d9466baf51124
language: rust
size_loc: 124
generated: by-keidocs
---

# kei-pet/src/fleet.rs

Multi-pet fleet per user.

One user_id owns N pet personas. All pets under that user share one
user-level memory scope (shared_memory_key), but each pet keeps its own
conversation stream (per_pet_memory_key). Fleet state is serialized to
`<fleet_root>/<user_id>/fleet.toml`; per-pet manifests are written by
the caller at paths recorded in `PetHandle::manifest_path`.

Scope boundary: this module owns only the fleet index file. It never
reads or writes individual pet manifests — those are the caller's
responsibility, referenced here by `PathBuf` only.

## Public API

- Fleet = ordered list of pet handles plus the currently active pet.
- Pointer to one pet persona + its role + manifest location on disk.
- Errors surfaced by fleet operations.
- `pub fn fleet_path` — Canonical on-disk path for a user's fleet index file.
- `pub fn load_fleet` — Load fleet for `user_id`. If the index file does not yet exist, return
- `pub fn save_fleet` — Serialize fleet to `<fleet_root>/<user_id>/fleet.toml`, creating the
- `pub fn add_pet` — Append `handle` to the user's fleet. If this is the first pet added,
- `pub fn switch_active` — Set `active_pet` to `pet_name`. Errors if the fleet is absent or the
- `pub fn shared_memory_key` — Shared memory key (all pets under this user share this scope).
- `pub fn per_pet_memory_key` — Per-pet memory key (one conversation stream per (user, pet) pair).

## Related

- parent: `kei-pet/Cargo.toml`
- imports: serde, std

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
