---
title: types.rs
path: kei-backend-daytona/src/types.rs
dna_hash: sha256:b02ddbe912d2892e
language: rust
size_loc: 127
generated: by-keidocs
---

# kei-backend-daytona/src/types.rs

DTOs for the Daytona REST API.

These mirror the subset of the Daytona Python SDK we depend on
(resume-or-create, exec, file sync). Fields marked `#[serde(default)]`
tolerate unknown / older API responses.

## Public API

- Lifecycle state of a sandbox. Matches Daytona Python SDK `SandboxState`.
- Sandbox is up and accepting commands.
- Stopped but filesystem preserved (resume target).
- Hibernated (filesystem snapshotted, deeper than stopped).
- Sandbox terminal-failed; not resumable.
- Pending creation / starting up.
- Anything we don't know yet.
- CPU / memory / disk envelope. Memory + disk in GiB.
- REST API representation of a sandbox.
- Daytona-assigned unique id.
- Human-readable name (we use this as our `task_id`-derived handle).
- Current lifecycle state.
- Container image used to create the sandbox.
- Resource envelope.
- Free-form labels (we store `task_id` here).
- Request body for `POST /sandboxes`. Subset of `CreateSandboxFromImageParams`.
- `auto_stop_interval = 0` disables idle hibernation; `30` = 30-min idle.
- `pub fn new` — Builder convenience: `image` + `name`, defaults for everything else.
- `pub fn with_resources` — Override resources.
- `pub fn with_label` — Add a label (e.g. `"task_id" → task-uuid`).
- `pub fn with_persistent` — Disable auto-stop (caller manages stop/start manually).
- Output from a single `exec` call.
- Combined stdout+stderr (Daytona's `result` field).
- Process exit code.
- Internal: Daytona REST exec response shape.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: serde

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
