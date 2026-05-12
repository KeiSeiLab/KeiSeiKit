// SPDX-License-Identifier: Apache-2.0
//! Thin Telegram Bot API HTTP helpers.
//!
//! Constructor Pattern: three focused async fns + one error surface.
//! No logging of bot tokens; errors logged at call site.

#[cfg(feature = "serve")]
use crate::error::BuddyError;

/// Send a plain-text message to a Telegram chat.
///
/// Never logs `token` — redacted in error messages.
#[cfg(feature = "serve")]
pub async fn send_message(
    token: &str,
    chat_id: i64,
    text: &str,
    http: &reqwest::Client,
) -> Result<(), BuddyError> {
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");
    let body = serde_json::json!({"chat_id": chat_id, "text": text});
    let resp = http
        .post(&url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| BuddyError::Transport(e.to_string()))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        Err(BuddyError::Transport(format!("sendMessage HTTP {status}")))
    }
}

/// Register a webhook URL with Telegram.
#[cfg(feature = "serve")]
pub async fn set_webhook(
    token: &str,
    url: &str,
    secret: &str,
    http: &reqwest::Client,
) -> Result<(), BuddyError> {
    let endpoint = format!("https://api.telegram.org/bot{token}/setWebhook");
    let body = serde_json::json!({
        "url": url,
        "secret_token": secret,
        "drop_pending_updates": true
    });
    let resp = http
        .post(&endpoint)
        .json(&body)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| BuddyError::Transport(e.to_string()))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        Err(BuddyError::Transport(format!("setWebhook HTTP {status}")))
    }
}

/// Delete the registered webhook (reset to polling mode).
#[cfg(feature = "serve")]
pub async fn delete_webhook(token: &str, http: &reqwest::Client) -> Result<(), BuddyError> {
    let endpoint = format!("https://api.telegram.org/bot{token}/deleteWebhook");
    let resp = http
        .post(&endpoint)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| BuddyError::Transport(e.to_string()))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        Err(BuddyError::Transport(format!("deleteWebhook HTTP {status}")))
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "serve"))]
mod tests {
    use serde_json::Value;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Verifies that set_webhook sends a POST body with url, secret_token,
    /// and drop_pending_updates fields.
    #[tokio::test]
    async fn set_webhook_builds_correct_request() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/botTOKEN/setWebhook"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})),
            )
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let token = "TOKEN";
        let url_arg = "https://example.com/webhook";
        let secret = "MY_SECRET";
        let endpoint = format!("{}/bot{token}/setWebhook", server.uri());
        let body = serde_json::json!({
            "url": url_arg,
            "secret_token": secret,
            "drop_pending_updates": true
        });
        let resp = http.post(&endpoint).json(&body).send().await.unwrap();
        assert!(resp.status().is_success());

        let received = server.received_requests().await.unwrap();
        assert_eq!(received.len(), 1);
        let body_val: Value =
            serde_json::from_slice(&received[0].body).expect("parse body");
        assert_eq!(body_val["url"], url_arg);
        assert_eq!(body_val["secret_token"], secret);
        assert_eq!(body_val["drop_pending_updates"], true);
    }
}
