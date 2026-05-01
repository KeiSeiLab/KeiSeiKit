# context — orchestrator integration

This module is **disjoint**: it does not import `chat.rs`, `persona.rs`, or
`AppConfig`. The orchestrator wires it in once the rest of the daemon is
ready. Below is the exact patch the orchestrator should land in
`handlers/chat.rs`. Nothing in this file is wired yet.

## Required `AppConfig` additions (orchestrator-side)

`config.rs` must expose `cwd: PathBuf` and `project_root: PathBuf` (both
absolute, both resolved at startup). Default `cwd` to `std::env::current_dir()`,
default `project_root` to the same value. The daemon owner can override via
flag/env if needed.

## Patch for `handlers/chat.rs::chat`

```rust
// imports near the top
use crate::context;

// inside chat(), AFTER `validate_body(&req)?` and BEFORE
// `let (_, system) = load_and_render(...)?;`
let (_, persona) = load_and_render(&state.config().pet_root, &user_id)?;

// 1. Walk up cwd, collecting CLAUDE.md / AGENTS.md / SOUL.md.
let ctx_files = context::discover(&state.config().cwd);

// 2. Match leading /skill-name in the user message.
let skill_match = context::match_skill_command(
    &req.message,
    &state.config().project_root,
);

// 3. Build the augmented system prompt (capped at 50 KiB).
let system = context::build_system_prompt(
    &persona,
    &ctx_files,
    skill_match.as_ref(),
);

// 4. Pass `system` (not `persona`) to the upstream call.
let upstream = anthropic::open_stream(&system, &messages)
    .await
    .map_err(upstream_to_app_error)?;
```

## Surface guarantees

- `discover` is pure I/O on the local filesystem. No network. Symlinks
  are not followed (`WalkDir::follow_links(false)` semantics are
  implemented manually via `symlink_metadata`).
- `match_skill_command` returns `None` when the message has no leading
  `/<name>` token; the caller should NOT attempt to interpret command
  semantics — that lives at a higher layer.
- `build_system_prompt` will always include the persona, even when the
  persona alone exceeds the 50 KiB cap (in which case the persona is
  truncated with a `[truncated]` marker rather than silently dropped).

## Testing notes for the orchestrator

When orchestrator-side tests stub `cwd` / `project_root`, point both at a
`tempdir` and write fake CLAUDE.md / AGENTS.md / SOUL.md under it. The
nested-fixture pattern in `tests/discover_walks_up.rs::fixture()` is the
canonical example.
