# Integration handoff — `tool_apply.rs` → cortex-ui DiffPane

Endpoint: `POST /api/v1/cortex/tool/apply`. Wired in `routes.rs` inside the
bearer-auth group.

## Trust posture (load-bearing)

This endpoint applies file edits proposed by the agentic loop directly to
disk. It is a TRUSTED operation. Three guarantees stand between an
attacker and a write:

1. **Bearer-auth required** — the route lives under the bearer
   middleware; an unauthenticated request gets 401.
2. **Path sandbox** — every request path must (a) be absolute, (b) be
   free of `..`, (c) NOT live under a root-level system dir
   (`/etc`, `/var`, `/usr`, `/bin`, `/sbin`, `/boot`), and (d)
   resolve to a location under `state.config().project_root`. The
   resolution walks up to the deepest existing ancestor and
   canonicalises THAT — so symlinks in the existing parent chain are
   followed (an attacker can't `ln -s /etc <root>/escape` and write
   to `<root>/escape/passwd-clone`). These checks reuse
   `tool::read::validate_path` and `tool::write::deny_system_dirs` —
   the same sandbox the agentic loop tools obey, so the human-
   confirmed Apply path cannot escape limits the agent itself respects.

   **Wave 44b F-CRIT-4 hardening** (`tool_apply_atomic.rs`): the
   write step itself now uses `openat(parent_fd, "...", O_NOFOLLOW |
   O_CREAT | O_EXCL)` for the staging tempfile and `renameat` for
   the swap-into-place, plus a post-rename canonical re-check of the
   final destination against `project_root_canon`. A symlink injected
   between `resolve_under_root` and the write step is detected and
   the leaf is unlinked before we return success. Residual race: an
   attacker who can swap a parent dir for a symlink AND swap it back
   between rename and the final canonicalize can in principle still
   escape; closing that fully requires `openat2(RESOLVE_BENEATH |
   RESOLVE_NO_SYMLINKS)` (Linux ≥5.6) and is left to a future Wave.
3. **No overwrite without `force`** — `write` rejects existing paths
   with 409 Conflict. The UI today only sends `edit`, so this is
   purely a future-proofing rail; if/when a `write` use case lands,
   the caller must explicitly pass `force=true` to confirm overwrite.

## What is NOT yet enforced (future Wave)

- **Per-conversation provenance**: signature verification tying an
  applied edit to the `conversation_id` that proposed it. Today the
  bearer token is a single shared secret; any caller with the token
  can apply any diff at any time.
- **Patch-level diffing**: we treat the body as `(old_string,
  new_string)` and rely on uniqueness for in-place safety. A future
  Wave can adopt a true `diff_id`-keyed flow where the daemon emits an
  opaque id alongside `tool_use_start{name:"edit"}` and only accepts
  apply requests carrying that id.

## Wire shape

Request:
```jsonc
{
  "tool": "edit" | "write",       // optional; default "edit"
  "path": "/abs/path/under/root",
  // edit:
  "old_string": "...",            // OR "old_text" (UI alias)
  "new_string": "...",            // OR "new_text" (UI alias)
  "replace_all": false,           // optional
  // write:
  "content": "...",               // required for write
  "force": false                  // required true to overwrite
}
```

Response (200):
```json
{
  "applied": true,
  "tool": "edit",
  "path": "/abs/path/under/root",
  "diff_summary": { "lines_changed": 3 }
}
```

Errors:
- `400 bad_request` — missing fields, unknown tool name, invalid path
- `403 forbidden` — path outside `project_root` OR system-dir denial
- `404 not_found` — edit target file does not exist
- `409 conflict` — `old_string` not found, `old_string` matched >1 time
  without `replace_all`, OR write target exists without `force`
- `413 payload_too_large` — file or content > 10 MiB
- `500 internal` — IO failure during read / atomic-write

## UI compatibility

The current `applyToolEdit` in
`_ts_packages/packages/cortex-ui/src/lib/api.ts` POSTs the shape
`{ path, old_text, new_text }` (no `tool` field, no `old_string`). The
handler accepts both naming conventions transparently:
- `tool` defaults to `"edit"` when absent.
- `old_string`/`new_string` are preferred; `old_text`/`new_text` are
  honoured as aliases.

So no UI change is required to flip from "404 silently" to "actually
applies".

## Edit semantics summary

- `replace_all=false` (default): exactly one occurrence required;
  multiple matches → 409.
- `replace_all=true`: every occurrence is replaced; a count of zero
  still 409s.
- Empty `old_string` is rejected (400).

## Write safety summary

- New path → write succeeds and creates parent dirs as needed.
- Existing path → 409 Conflict unless `force=true`. Even with `force`
  the path must clear all sandbox checks.

## Atomicity

Both edit and write stage to a sibling tempfile inside the destination
directory and `rename()` into place. Same-directory rename is atomic on
POSIX and Windows so partial writes never appear on disk.
