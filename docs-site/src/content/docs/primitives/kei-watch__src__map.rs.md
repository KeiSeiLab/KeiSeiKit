---
title: map.rs
path: kei-watch/src/map.rs
dna_hash: sha256:b7809e945126135d
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-watch/src/map.rs

Mapping: `notify::Event` → zero or more canonical [`Event`].

Folding rules:
- `Create(*)`       → `EventKind::Created`
- `Modify(Data*)` / `Modify(Any)` / `Modify(Other)` → `EventKind::Modified`
- `Remove(*)`       → `EventKind::Deleted`
- `Modify(Name(*))` → `EventKind::Renamed` (from_path populated if both
endpoints present in `paths`; else None)
- `Access(*)` / `Modify(Metadata(*))` / `Other` / `Any` → SKIP

Rationale: Access events fire constantly on macOS fsevents and are
rarely what a hot-reload / drift-detection consumer wants. Metadata
changes (mtime-only touch) are likewise noise.

## Public API

- `pub fn from_notify` — Convert one `notify::Event` into 0..N canonical [`Event`]s.
- Emit one canonical event per path in `ev.paths`.
- Rename mapping. `RenameMode::Both` carries `[from, to]` in paths;

## Related

- parent: `kei-watch/Cargo.toml`
- imports: crate, notify, std

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
