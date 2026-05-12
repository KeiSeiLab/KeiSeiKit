# kei-telegram-webhook

Inbound Telegram Bot API webhook handler.  
Sibling to `kei-notify-telegram` (outbound). This crate is the **inbound** half.

## Purpose

Parse Telegram `Update` payloads arriving via HTTPS POST into typed
`WebhookEvent` values. Secret-token verification included.

## Architecture

The crate exposes a single axum **handler function** and the parsed types.
It does **not** own an `axum::Server` — that is the consumer's job.
Mount `handle_webhook` into your existing `Router`.

## Usage

```rust
use axum::{routing::post, Router};
use kei_telegram_webhook::handle_webhook;

#[derive(Clone)]
struct AppState { token: String }

#[async_trait::async_trait]
impl kei_telegram_webhook::WebhookContext for AppState {
    fn secret_token(&self) -> &str { &self.token }
    async fn on_event(&self, event: kei_telegram_webhook::WebhookEvent) {
        println!("{event:?}");
    }
}

let state = AppState { token: "MY_SECRET".into() };
let app = Router::new()
    .route("/telegram/webhook", post(handle_webhook::<AppState>))
    .with_state(state);
// pass `app` to your axum::serve call
```

## Status

Alpha — handler logic and unit tests pass; real Telegram POST integration
verified by the consumer (KeiBuddy).
