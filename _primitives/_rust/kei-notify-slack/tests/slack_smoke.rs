// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `wiremock`-driven smoke tests for [`SlackChannel`]. No live Slack calls.

use kei_notify_slack::{Error, SlackChannel};
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{DnaBuilder, HasDna};
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn make_notification(severity: NotifySeverity, subject: &str, body: &str) -> Notification {
    let dna = DnaBuilder::new("notification")
        .cap("NT")
        .scope("test/slack")
        .body(b"smoke")
        .build()
        .unwrap();
    let parent = DnaBuilder::new("primitive")
        .cap("PR")
        .scope("test/slack-parent")
        .body(b"parent")
        .build()
        .unwrap();
    Notification {
        dna,
        parent_dna: parent,
        subject: subject.into(),
        body_text: body.into(),
        body_html: None,
        severity,
        tags: Vec::new(),
    }
}

#[tokio::test]
async fn send_info_200() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/T1/B1/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;
    let url = format!("{}/services/T1/B1/abc", server.uri());
    let ch = SlackChannel::with_url(None, url).unwrap();
    let n = make_notification(NotifySeverity::Info, "hello", "world");
    ch.send(&n).await.expect("send ok");
    assert_eq!(ch.channel_name(), "slack");
}

#[tokio::test]
async fn send_error_includes_danger_color() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/T2/B2/xyz"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;
    let url = format!("{}/services/T2/B2/xyz", server.uri());
    let ch = SlackChannel::with_url(None, url).unwrap();
    let n = make_notification(NotifySeverity::Error, "boom", "fatal");
    ch.send(&n).await.expect("send ok");

    // Pull the captured request and assert its JSON body shape.
    let received = server.received_requests().await.unwrap();
    assert_eq!(received.len(), 1);
    let req: &Request = &received[0];
    let body: Value = serde_json::from_slice(&req.body).unwrap();
    assert_eq!(body["text"], "boom");
    assert_eq!(body["attachments"][0]["color"], "danger");
    assert_eq!(body["attachments"][0]["title"], "boom");
    assert_eq!(body["attachments"][0]["text"], "fatal");
}

#[tokio::test]
async fn send_500_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/services/T3/B3/fail"))
        .respond_with(ResponseTemplate::new(500).set_body_string("server burning"))
        .mount(&server)
        .await;
    let url = format!("{}/services/T3/B3/fail", server.uri());
    let ch = SlackChannel::with_url(None, url).unwrap();
    let n = make_notification(NotifySeverity::Warn, "hot", "smoke");
    let err = ch.send_raw(&n).await.expect_err("must fail");
    match err {
        Error::Api(msg) => {
            assert!(msg.contains("500"), "msg should include status: {msg}");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn dna_has_sk_cap() {
    let ch = SlackChannel::with_url(None, "http://example.invalid/services/x/y/z").unwrap();
    let caps = ch.dna().caps();
    assert!(caps.contains("SK"), "DNA caps must include SK, got {caps}");
    assert!(caps.contains("PR"), "DNA caps must include PR, got {caps}");
    assert!(caps.contains("AP"), "DNA caps must include AP, got {caps}");
    assert_eq!(ch.dna().role(), "primitive");
}
