// SPDX-License-Identifier: Apache-2.0
//! Axum handler for the Telegram webhook endpoint.
//!
//! Mount in your router with:
//! ```ignore
//! router.route("/telegram/webhook", axum::routing::post(handle_webhook::<MyState>))
//! ```

use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};
use tracing::debug;

use crate::{
    context::WebhookContext,
    event::classify,
    update::Update,
};

const TELEGRAM_TOKEN_HEADER: &str = "x-telegram-bot-api-secret-token";

/// Validate the secret token from the request headers.
///
/// Returns `Ok(())` on match, `Err(StatusCode::UNAUTHORIZED)` on mismatch or
/// absent header.
fn verify_token(headers: &HeaderMap, expected: &str) -> Result<(), StatusCode> {
    let provided = headers
        .get(TELEGRAM_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if provided == expected {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Axum POST handler for inbound Telegram `Update` payloads.
///
/// 1. Validates `X-Telegram-Bot-Api-Secret-Token` — returns 401 on mismatch.
/// 2. Parses the JSON body into [`Update`] — axum returns 400 on bad JSON.
/// 3. Calls [`classify`] and dispatches to [`WebhookContext::on_event`].
/// 4. Returns 200.
pub async fn handle_webhook<S>(
    State(state): State<S>,
    headers: HeaderMap,
    Json(update): Json<Update>,
) -> Result<StatusCode, StatusCode>
where
    S: WebhookContext,
{
    debug!(update_id = update.update_id, "received telegram update");

    verify_token(&headers, state.secret_token())?;

    let event = classify(update);
    state.on_event(event).await;

    Ok(StatusCode::OK)
}

// ──────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{self, Request},
        routing::post,
        Router,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::util::ServiceExt;

    use crate::event::WebhookEvent;

    #[derive(Clone)]
    struct MockCtx {
        token: String,
        captured: Arc<Mutex<Vec<WebhookEvent>>>,
    }

    impl MockCtx {
        fn new(token: &str) -> Self {
            Self {
                token: token.into(),
                captured: Arc::new(Mutex::new(vec![])),
            }
        }
    }

    #[async_trait]
    impl WebhookContext for MockCtx {
        fn secret_token(&self) -> &str {
            &self.token
        }
        async fn on_event(&self, event: WebhookEvent) {
            self.captured.lock().await.push(event);
        }
    }

    fn minimal_update_json() -> &'static str {
        r#"{"update_id":1,"message":{"message_id":1,"date":0,"chat":{"id":10},"text":"hi"}}"#
    }

    fn build_app(ctx: MockCtx) -> Router {
        Router::new()
            .route("/webhook", post(handle_webhook::<MockCtx>))
            .with_state(ctx)
    }

    #[tokio::test]
    async fn bad_secret_token_returns_401() {
        let ctx = MockCtx::new("RIGHT");
        let app = build_app(ctx);

        let req = Request::builder()
            .method(http::Method::POST)
            .uri("/webhook")
            .header(TELEGRAM_TOKEN_HEADER, "WRONG")
            .header("content-type", "application/json")
            .body(Body::from(minimal_update_json()))
            .expect("build request");

        let resp = app.oneshot(req).await.expect("call handler");
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn good_secret_token_returns_200() {
        let ctx = MockCtx::new("RIGHT");
        let app = build_app(ctx);

        let req = Request::builder()
            .method(http::Method::POST)
            .uri("/webhook")
            .header(TELEGRAM_TOKEN_HEADER, "RIGHT")
            .header("content-type", "application/json")
            .body(Body::from(minimal_update_json()))
            .expect("build request");

        let resp = app.oneshot(req).await.expect("call handler");
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
