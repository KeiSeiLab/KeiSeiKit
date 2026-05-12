# kei-buddy

**Maturity:** concept / scaffold — no business logic yet.

## Purpose

`kei-buddy` is the runtime crate that composes existing KeiSeiKit
primitives (`kei-pet`, `kei-memory-sqlite`, `kei-cortex`,
`kei-notify-telegram`) into a personal-assistant Telegram bot called
KeiBuddy.

On first contact the bot walks the user through an 11-state onboarding
flow: name, tone, interests, hobbies, per-topic decomposition (specifics
→ now-or-later → research preference → source selection), and digest
schedule. After onboarding the bot enters ongoing conversation mode,
drawing on the stored persona and memory.

This crate provides the state-machine enum and skeleton driver. The
onboarding FSM is ported from
`keisei-marketplace/src/lib/keibuddy/chat-onboard.ts`.

## Status

Scaffold only. The `OnboardState` enum and `TransitionInput` struct are
defined. All transition logic is stubbed (`next()` returns `self.clone()`).
The binary entry point prints a placeholder message and exits 0.

## Running

### Environment variables

| Variable | Required | Default | Description |
|---|---|---|---|
| `TELEGRAM_BOT_TOKEN` | yes (serve) | — | Bot token from @BotFather |
| `TELEGRAM_WEBHOOK_SECRET` | yes (serve) | — | Secret token for webhook verification |
| `KEI_BUDDY_PORT` | no | `8080` | HTTP port to bind |
| `KEI_BUDDY_DB_PATH` | no | `./kei-buddy.db` | SQLite database path |
| `OPENAI_API_KEY` | no | — | Enables OpenAiExtractor when set (requires `extractor-openai` feature) |

### Subcommands

```sh
# Apply schema (idempotent; run once before first serve)
kei-buddy migrate

# Register the webhook URL with Telegram
kei-buddy webhook-set https://your-domain.com/webhook

# Start the HTTP server
kei-buddy serve

# Remove the registered webhook (revert to polling)
kei-buddy webhook-delete
```

### Example systemd unit

```ini
[Unit]
Description=KeiBuddy Telegram bot
After=network.target

[Service]
EnvironmentFile=/etc/kei-buddy/env
ExecStart=/usr/local/bin/kei-buddy serve
Restart=on-failure
User=keisei

[Install]
WantedBy=multi-user.target
```

## Roadmap

- **OpenAiExtractor wiring** — pass real OPENAI_API_KEY to OpenAiExtractor in serve.rs when feature enabled.
- **Persona binding** — read persona manifest via `kei-pet`; apply tone overlay to outgoing replies.
- **Digest scheduling** — wire `kei-cron-scheduler` for morning/evening digest delivery.
