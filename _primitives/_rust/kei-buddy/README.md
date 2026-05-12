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

## Roadmap

- **State-machine transition logic** — port `handleStep` from
  `chat-onboard.ts`; wire per-state LLM-extract calls through kei-cortex.
- **Memory binding** — persist scratchpad and finalised persona via
  kei-memory-sqlite; implement `getChatState` / `setChatStep` equivalents.
- **Persona binding** — read persona manifest via `kei-pet`; apply tone
  overlay to outgoing replies.
- **Transport binding** — wire kei-notify-telegram for outbound messages;
  add a real Telegram webhook server (or kei-gateway adapter) for inbound.
