---
title: invoke_kills_runaway.rs
path: kei-runtime/tests/invoke_kills_runaway.rs
dna_hash: sha256:e41510464732e01b
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-runtime/tests/invoke_kills_runaway.rs

Integration test — runaway atom that floods stdout MUST be killed
after the 16 MiB cap, not buffered to OOM.

Wave 44d resource-cap: replaces the post-hoc `cap_bytes` truncation
with streamed reads in `invoke_io.rs`. This test pins the new
behaviour: a fake atom binary that emits 100 MiB of zeros must
exit non-zero (killed by parent) rather than complete normally
with 100 MiB buffered.

Strategy:
1. Build a tiny shell-script "atom" that ignores stdin and writes
100 MiB of zeros to stdout. We can't use `dd` directly because
the runtime's allowlist enforces `kei-*` crate names — so we
stage a script named `kei-flood` in a temp bin dir.
2. Stage atom YAML naming `kei-flood::pour`.
3. Invoke via the runtime CLI; expect non-zero exit + a stderr
message naming the cap.

## Public API

- Stage a `kei-flood` shell-script in `bin_dir`. When invoked with

## Related

- parent: `kei-runtime/tests`
- imports: std

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
