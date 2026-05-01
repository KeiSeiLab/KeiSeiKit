# kei-router LLM provider — INTEGRATION.md

> v0.40 Wave 32. Multi-provider LLM abstraction. This file is the orchestrator
> wiring guide for `kei-cortex/src/handlers/chat.rs`. The kei-router crate
> ships standalone — wiring it into kei-cortex is a separate commit owned by
> the orchestrator (RULE 0.13: agent writes files only; orchestrator commits).

## Three-step orchestrator wire-in (kei-cortex)

### 1. CLI flag on the daemon (`kei-cortex/src/main.rs`)

Add a `--provider <name>` flag to the existing clap config. Default = `anthropic`.

```rust
#[arg(long, default_value = "anthropic")]
provider: String,
```

Pass it into `AppState` at startup.

### 2. Replace direct `anthropic::open_stream` with `LlmRouter::pick`

In `kei-cortex/src/handlers/chat.rs`, find the call site:

```rust
let stream = anthropic::open_stream(system, &messages).await?;
```

Replace with:

```rust
let provider = state.router.pick(&provider_name)?;
let stream = provider.stream_message(system, &messages, None).await?;
```

The stream type changes from `Stream<Item = Result<String, Error>>` to
`BoxStream<'static, Result<StreamEvent, LlmError>>`. Update the handler's
SSE forwarding loop to match on `StreamEvent::Token(t)` for tokens and
`StreamEvent::Done` for clean stream end.

### 3. Wire `LlmRouter` into `AppState`

In `kei-cortex/src/state.rs`:

```rust
use std::sync::Arc;
use kei_router::LlmRouter;

pub struct AppState {
    // ... existing fields ...
    pub router: Arc<LlmRouter>,
}
```

At startup (`kei-cortex/src/main.rs::run` or wherever `AppState` is
constructed):

```rust
let router = Arc::new(LlmRouter::from_env());
let state = AppState { /* ..., */ router };
```

`LlmRouter::from_env()` registers any provider whose API key is present:
`ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `KIMI_API_KEY` (or
`MOONSHOT_API_KEY` as fallback for kimi).

## Errors to surface to clients

`kei_router::LlmError` enum maps to:

| LlmError variant       | HTTP status  | Notes                             |
|------------------------|--------------|-----------------------------------|
| `MissingKey(_)`        | 500 (config) | Daemon misconfig — log + alert    |
| `RateLimit(_)`         | 429          | Pass through to client            |
| `ServiceUnavailable(_)`| 503          | Pass through                      |
| `Timeout(_)`           | 504          | Per-provider 60s handshake budget |
| `Upstream { ... }`     | 502          | With truncated body in log only   |
| `UnknownProvider(_)`   | 400          | Bad `--provider` value            |
| `Http(_)`              | 502          | Connection-level                  |

## Cost-based dispatch (optional, v0.40+)

When the caller provides a token estimate, `LlmRouter::cheapest_for_estimated_tokens`
returns the cheapest registered provider. Useful for batch / non-interactive
flows. NOT used for interactive chat by default — interactive flows pin the
provider via `--provider`.

## Testing the integration

After the three steps above, run the existing kei-cortex tests AND:

```bash
ANTHROPIC_API_KEY=sk-ant-... \
  cargo test -p kei-cortex chat_smoke_test
```

A new smoke test should hit `/chat` with `?provider=anthropic` and assert
stream of `data: { "type": "token", "text": "..." }` events.

## Constructor Pattern compliance

| File                                  | LOC | Cubes                |
|---------------------------------------|-----|----------------------|
| `src/provider.rs`                     | ~80 | trait + 4 types      |
| `src/llm_router.rs`                   | ~100| `LlmRouter` only     |
| `src/providers/anthropic.rs`          | ~140| 1 provider           |
| `src/providers/openai.rs`             | ~140| 1 provider           |
| `src/providers/kimi.rs`               | ~140| 1 provider           |
| `src/providers/sse.rs`                | ~70 | shared parser        |

All under the 200 LOC/file, 30 LOC/fn budgets.
