// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock-only integration tests. No live HTTP, no Telegram API
//! calls. Covers the Bot API `sendMessage` happy path, the
//! `{"ok":false}` rejection path, the 5xx transport error, the
//! `parse_mode=HTML` body invariant, the HTML-escape invariant, and
//! the DNA cap surface.

use kei_notify_telegram::TelegramChannel;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use serde_json::{json, Value};
use wiremock::matchers::{body_partial_json, method, path_regex};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn dna() -> Dna {
    DnaBuilder::new("test")
        .cap("TG")
        .scope("test/scope")
        .body(b"test")
        .build()
        .unwrap()
}

fn make_notif(sev: NotifySeverity, subject: &str, body: &str) -> Notification {
    Notification {
        dna: dna(),
        parent_dna: dna(),
        subject: subject.into(),
        body_text: body.into(),
        body_html: None,
        severity: sev,
        tags: vec![],
    }
}

fn channel_for(server: &MockServer) -> TelegramChannel {
    TelegramChannel::with_config(server.uri(), "TEST_TOKEN", "12345", None)
        .expect("channel ctor")
}

#[tokio::test]
async fn send_ok_true_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": { "message_id": 7 }
        })))
        .mount(&server)
        .await;

    let ch = channel_for(&server);
    ch.send(&make_notif(NotifySeverity::Info, "hello", "world"))
        .await
        .expect("send ok");
}

#[tokio::test]
async fn send_ok_false_returns_api_error_with_description() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": false,
            "description": "Bad Request: chat not found"
        })))
        .mount(&server)
        .await;

    let ch = channel_for(&server);
    let err = ch
        .send(&make_notif(NotifySeverity::Warn, "subj", "body"))
        .await
        .expect_err("send must fail");
    let s = err.to_string();
    assert!(
        s.contains("Bad Request: chat not found"),
        "expected description in error, got: {s}"
    );
    // Routed through provider variant (see error.rs From bridge).
    assert!(s.contains("provider"), "must surface as provider error: {s}");
}

#[tokio::test]
async fn send_5xx_returns_http_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream down"))
        .mount(&server)
        .await;

    let ch = channel_for(&server);
    let err = ch
        .send(&make_notif(NotifySeverity::Error, "x", "y"))
        .await
        .expect_err("must fail on 5xx");
    let s = err.to_string();
    assert!(s.contains("503"), "expected status code in error: {s}");
}

#[tokio::test]
async fn dna_has_tg_cap() {
    let server = MockServer::start().await;
    let ch = channel_for(&server);
    let caps = ch.dna().caps();
    assert!(caps.contains("TG"), "DNA caps must include TG: {caps}");
    assert!(caps.contains("PR"), "DNA caps must include PR: {caps}");
    assert!(caps.contains("AP"), "DNA caps must include AP: {caps}");
    assert_eq!(ch.channel_name(), "telegram");
    assert!(!ch.supports_batching());
}

#[tokio::test]
async fn parse_mode_html_in_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .and(body_partial_json(json!({"parse_mode": "HTML"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true, "result": {}
        })))
        .mount(&server)
        .await;

    let ch = channel_for(&server);
    // If the body lacks parse_mode=HTML, wiremock returns 404 and send fails.
    ch.send(&make_notif(NotifySeverity::Success, "subj", "body"))
        .await
        .expect("parse_mode=HTML must be in request body");
}

#[tokio::test]
async fn escapes_html_special_chars() {
    let server = MockServer::start().await;
    // Capture matcher: any POST to sendMessage. We then read the recorded
    // requests and assert on the JSON body content.
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true, "result": {}
        })))
        .mount(&server)
        .await;

    let ch = channel_for(&server);
    ch.send(&make_notif(
        NotifySeverity::Error,
        "alert <prod>",
        "value & x > y",
    ))
    .await
    .expect("send ok");

    let recorded: Vec<Request> = server.received_requests().await.unwrap();
    assert_eq!(recorded.len(), 1, "exactly one request expected");
    let body: Value = serde_json::from_slice(&recorded[0].body).expect("json body");
    let text = body
        .get("text")
        .and_then(|v| v.as_str())
        .expect("text field");
    assert!(text.contains("&lt;prod&gt;"), "subject lt/gt escape: {text}");
    assert!(text.contains("&amp;"), "amp escape: {text}");
    assert!(text.contains("&gt; y"), "body gt escape: {text}");
    // Severity emoji for Error must be present.
    assert!(text.contains("🚨"), "severity emoji must be present: {text}");
    // Bold wrapper for subject must remain literal.
    assert!(text.contains("<b>"), "bold open tag: {text}");
    assert!(text.contains("</b>"), "bold close tag: {text}");

    // chat_id was numeric — verify it serialized as i64, not a quoted string.
    let chat_id = body.get("chat_id").expect("chat_id field");
    assert!(chat_id.is_i64(), "chat_id should serialize as i64: {chat_id}");
    assert_eq!(chat_id.as_i64(), Some(12345));
}

#[tokio::test]
async fn channel_username_chat_id_serializes_as_string() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r"^/botTEST_TOKEN/sendMessage$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true, "result": {}
        })))
        .mount(&server)
        .await;

    let ch = TelegramChannel::with_config(server.uri(), "TEST_TOKEN", "@my_channel", None)
        .expect("ctor");
    ch.send(&make_notif(NotifySeverity::Info, "s", "b"))
        .await
        .expect("send ok");

    let recorded = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&recorded[0].body).unwrap();
    let chat_id = body.get("chat_id").unwrap();
    assert_eq!(chat_id.as_str(), Some("@my_channel"));
}
