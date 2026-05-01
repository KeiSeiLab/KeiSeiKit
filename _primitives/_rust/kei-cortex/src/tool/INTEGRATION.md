# Integration handoff — `tool/` substrate → `handlers/chat.rs`

## Wave 44c update (2026-04-24, F-HIGH-5)

`run_with_tools` now takes `tokio_util::sync::CancellationToken` instead of
`tokio::sync::oneshot::Receiver<()>`. The orchestrator wires the cancel
plumbing at the SSE entry as follows:

```rust
use tokio_util::sync::CancellationToken;

// (1) Create the token at the chat_stream.rs entry point
let cancel = CancellationToken::new();

// (2) Pass a clone to the loop
let loop_stream = tool::run_with_tools(
    invoker, registry, tool_defs,
    message, conv_id, cancel.clone(),
);

// (3) Hold the original inside a Drop-guard so SSE-client disconnect
//     (stream gets dropped before completion) cancels the loop:
struct CancelOnDrop { token: CancellationToken }
impl Drop for CancelOnDrop { fn drop(&mut self) { self.token.cancel(); } }
let _hold = CancelOnDrop { token: cancel };
```

Plumbing diagram:

```
┌─ orchestrator (chat.rs) ────────────────────────────────┐
│  CancellationToken::new()                                │
│         │           │                                    │
│         ▼           ▼                                    │
│  CancelOnDrop   run_with_tools(..., cancel.clone())      │
│  (RAII guard)        │                                   │
│  in stream! block    ▼                                   │
│  Drop on SSE  ┌─ loop_driver inner_loop ───────────────┐ │
│  disconnect   │  tokio::select! { biased;             │ │
│  → cancel()   │     _ = cancel.cancelled() => abort,  │ │
│         ─────►│     o = invoke_one_turn(...) => use,  │ │
│               │  }                                     │ │
│               │  (cancel checked PER TURN, not just    │ │
│               │   at top of for-loop)                  │ │
│               └────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
```

Per-turn behaviour: a long `bash` (60s) or `agent` (120s+) call in the
tool dispatch no longer holds the cancel signal for the whole turn. The
`select!` races the in-flight invoker future against `cancel.cancelled()`
and aborts within a tokio-poll cycle (sub-millisecond) of the cancel.

Per-tool dispatch is **not** yet cancel-aware. If a tool-dispatch is
multi-second (e.g. agent loops, web fetch) and the user disconnects mid-
dispatch, the dispatch completes before the next `select!` cycle. To
extend cancellation INTO tool dispatch, plumb the `CancellationToken`
through `dispatch::dispatch_outcome` and into each tool's `Executor`
trait — out of scope for Wave 44c.



This doc is read by the orchestrator at merge time. It is **NOT** a code
change made by this branch. `chat.rs` is the merge-conflict hotspot;
parallel agents are working there, so this branch leaves it untouched.

## What's ready (in this branch)

- `kei_cortex::tool::ToolRegistry::with_project_root(...)` — production
  registry constructor (8 tools), captures the chroot
- `kei_cortex::tool::ToolRegistry::default()` — falls back to
  `project_root = PathBuf::from(".")`. **Tests only** — production must
  call `with_project_root`.
- `kei_cortex::tool::tool_definitions()` — JSON-Schema for Anthropic body
- `kei_cortex::tool::run_with_tools(...)` — agentic loop returning `Stream<LoopEvent>`
- `kei_cortex::tool::ModelInvoker` — function-pointer alias the orchestrator wires

## Wave 44a — REQUIRED orchestrator-side patch (chat_stream.rs)

The current `handlers/chat_stream.rs::run_loop_stream` calls
`tool::ToolRegistry::default()`. After Wave 44a merge, that default
chroots the daemon to the literal `.` directory, which is almost never
the user's project root. The orchestrator MUST replace it with:

```rust
// Before (chat_stream.rs:49):
let registry = Arc::new(tool::ToolRegistry::default());

// After:
let registry = Arc::new(tool::ToolRegistry::with_project_root(
    state.config().project_root.clone(),
));
```

This single-line change is the entire Wave 44a integration delta on the
chat_stream.rs side. It is left unmodified in this branch per RULE 0.13
(orchestrator owns merges).

## What Wave 44a fixes (security findings)

| Finding | Fix location |
|---|---|
| F-CRIT-1 bash deny-list bypassable | `bash.rs` + `bash_denylist.rs` (tokenizer + allow-list) |
| F-CRIT-2 webfetch SSRF | `webfetch.rs` + new `ip_filter.rs` |
| F-CRIT-3 read tool no chroot | `read.rs` + new `path_sandbox.rs` |
| F-HIGH-1 write tool no secrets-block | `write.rs` + `path_sandbox.rs` |
| F-HIGH-4 edit non-atomic write | `edit.rs` + new `atomic_io.rs` (shared) |
| F-HIGH-6 agent.rs TOML injection | `agent.rs` (toml::Value::Table builder) |
| F-MED-1 webfetch unbounded cache | `webfetch.rs` (LRU 256, lru crate) |

## Cubes added

- `tool/path_sandbox.rs` — chroot + basename + dotfile deny (~150 LOC)
- `tool/atomic_io.rs`    — shared `atomic_write` (~50 LOC)
- `tool/ip_filter.rs`    — SSRF deny-list (v4 + v6 ranges) (~100 LOC)

## Cubes rewritten

- `tool/bash.rs`           — tokenizer-based check (shell-words crate)
- `tool/bash_denylist.rs`  — split: BANNED_ARGV0 + ALLOWED_ARGV0 + BANNED_SUBSTRINGS
- `tool/webfetch.rs`       — bounded LRU + IP filter + redirects=none + url crate
- `tool/read.rs`           — accepts &Path project_root + path_sandbox::check_all
- `tool/write.rs`          — same chroot + uses atomic_io
- `tool/edit.rs`           — same chroot + uses atomic_io (atomic write)
- `tool/glob_tool.rs`      — chroot enforcement when path supplied
- `tool/grep.rs`           — chroot enforcement when path supplied
- `tool/agent.rs`          — TOML built via toml::Value::Table (no f-string interpolation)
- `tool/registry.rs`       — `with_project_root` + `empty(PathBuf)` + closure capture
- `tool/types.rs`          — added `OutsideRoot` + `ShellParse` ToolError variants

## New workspace deps (Cargo.toml)

```toml
shell-words = "1"   # tokenize bash command before deny-list check
url = "2"           # robust URL parsing + host extraction for SSRF
lru = "0.12"        # bounded webfetch cache (256 entries)
```

## SSRF + secrets handling notes for the orchestrator

- `webfetch.rs` reject DNS-resolved IPs in private/loopback/link-local/
  CGNAT ranges. Tailscale users hit this; opt-in escape hatch is
  `KEI_WEBFETCH_ALLOW_PRIVATE=1` in the daemon's environment.
- Redirects are disabled. If you want to follow them, set
  `reqwest::redirect::Policy::limited(3)` AND re-validate every hop's
  resolved IP against `is_blocked_ip` before connect.
- The basename deny in `path_sandbox.rs` rejects `*.env`, `id_rsa*`,
  `*.pem`, `*.key`, `credentials*` regardless of containing dir. If
  the daemon legitimately needs to read its own `.env` for tooling
  (it shouldn't — `RULE 0.8 SECRETS-SINGLE-SOURCE`), use `std::env`
  directly, not the read tool.

## The 3-line patch the orchestrator applies

Inside `handlers/chat.rs::chat()`, replace the current
`anthropic::open_stream(...)` call site:

```rust
// (1) Build a ModelInvoker that wraps a non-streaming Anthropic
// messages.create call. The orchestrator owns this glue (one ~30-LOC fn
// in anthropic.rs that returns ModelTurn { content, stop_reason }).
let invoker = anthropic::tool_use_invoker(system.clone());

// (2) Run the agentic loop instead of the raw text stream:
let registry = std::sync::Arc::new(
    kei_cortex::tool::ToolRegistry::with_project_root(
        state.config().project_root.clone(),
    ),
);
let tool_defs = kei_cortex::tool::tool_definitions();
let (_cancel_tx, cancel_rx) = tokio::sync::oneshot::channel();
let loop_stream = kei_cortex::tool::run_with_tools(
    invoker, registry, tool_defs,
    req.message.clone(), conversation_id.clone(), cancel_rx,
);

// (3) Translate LoopEvent → axum SSE Event. A small `loop_event_to_sse`
// helper (≤30 LOC) maps each variant to a `data:` payload matching the
// existing token/error/sentiment/done shapes the UI already parses.
let stream = loop_stream.map(loop_event_to_sse);
```

## Notes for the orchestrator

- `ModelInvoker` is `Arc<dyn Fn(...) -> BoxFuture<...>>`. Build it once at
  startup (or per-request — it is cheap to clone). Wrap your Anthropic
  HTTP call inside the closure; return `Result<ModelTurn, String>`.
- The cancel channel can be wired to `axum`'s connection-close signal to
  stop the loop when the client disconnects. v0 leaves it inert (drops
  the sender immediately).
- The 8 tools share NO state with `AppState`. If you need the daemon to
  expose `state` to a tool (e.g., for a future `pet_status` tool), add a
  second registry constructor that captures the state via closure.
- Per-tool SSE event names suggested for the UI:
  - `LoopEvent::AssistantText` → `{"type":"token","text":"..."}`
  - `LoopEvent::ToolUseStart`  → `{"type":"tool_use_start","tool":"...","input":{...}}`
  - `LoopEvent::ToolUseResult` → `{"type":"tool_use_result","tool_use_id":"...","is_error":bool}`
  - `LoopEvent::Error`         → `{"type":"error","message":"..."}`
  - `LoopEvent::Done`          → `{"type":"done","conversation_id":"...","turns":N}`

## Why this is a handoff, not a wired change

RULE 0.13 — orchestrator owns merges. Six parallel agents are touching
adjacent files; `chat.rs` is the hotspot. Wiring this in-branch would
guarantee a merge conflict.

## Reconciling `atomic_write` with `handlers/tool_apply.rs`

`handlers/tool_apply.rs:172-185` has its own private `atomic_write`
function. Wave 44a adds a public `tool::atomic_io::atomic_write` cube
which `tool::write::run` and `tool::edit::run` now use. The orchestrator
should — at next refactor pass for the tool_apply cube — update
`tool_apply.rs` to import `tool::atomic_io::atomic_write` and delete the
duplicate. We do NOT make this change in Wave 44a because tool_apply.rs
is wave44b territory.

## Test changes summary

- `tests/bash_sandbox_denies.rs` — extended from 18 to 47 cases
  covering tokenizer-bypass classes (quote, IFS, escape, cmdsub,
  chained statements, allow-list default-deny).
- `tests/edit_unique_old_string.rs` — updated 5 cases to pass project_root.
- `tests/registry_dispatch.rs` — added `dispatch_read_outside_root_errors`
  case; `dispatch_read_returns_file_contents` now uses
  `with_project_root` to scope reads.
- `tests/loop_terminates_on_max_turns.rs` — `ToolRegistry::empty(...)` now
  takes a project_root argument; updates trivial.
- New unit tests inline in `path_sandbox.rs`, `atomic_io.rs`,
  `ip_filter.rs`, `bash_denylist.rs`, `webfetch.rs` (SSRF cases),
  `read.rs`, `write.rs`, `edit.rs`, `agent.rs` (TOML injection cases).
