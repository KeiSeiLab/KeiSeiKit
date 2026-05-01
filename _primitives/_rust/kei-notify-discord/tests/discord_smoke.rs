// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! REST-surface integration tests against a `wiremock` Discord webhook
//! stub. No live HTTP — every assertion is local to the test process.

use kei_notify_discord::DiscordChannel;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{DnaBuilder, HasDna};
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fixture(severity: NotifySeverity, subject: &str, body: &str) -> Notification {
    let dna = DnaBuilder::new("notification")
        .cap("ND")
        .scope("test")
        .body(b"n")
        .build()
        .unwrap();
    let parent = DnaBuilder::new("primitive")
        .cap("PR")
        .scope("test")
        .body(b"p")
        .build()
        .unwrap();
    Notification {
        dna,
        parent_dna: parent,
        subject: subject.into(),
        body_text: body.into(),
        body_html: None,
        severity,
        tags: vec![],
    }
}

#[tokio::test]
async fn send_204_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/webhooks/123/abc"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let url = format!("{}/api/webhooks/123/abc", server.uri());
    let channel = DiscordChannel::with_url(url, None).unwrap();
    let n = fixture(NotifySeverity::Info, "hello", "world");
    channel.send(&n).await.expect("send ok");
}

#[tokio::test]
async fn send_color_mapping_for_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/webhooks/123/abc"))
        .and(body_partial_json(json!({
            "embeds": [{"color": 15_158_332}]
        })))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let url = format!("{}/api/webhooks/123/abc", server.uri());
    let channel = DiscordChannel::with_url(url, None).unwrap();
    let n = fixture(NotifySeverity::Error, "boom", "stack");
    channel.send(&n).await.expect("send ok");
}

#[tokio::test]
async fn send_4xx_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/webhooks/123/abc"))
        .respond_with(ResponseTemplate::new(400).set_body_string("bad webhook"))
        .mount(&server)
        .await;

    let url = format!("{}/api/webhooks/123/abc", server.uri());
    let channel = DiscordChannel::with_url(url, None).unwrap();
    let n = fixture(NotifySeverity::Warn, "warn", "msg");
    let err = channel.send(&n).await.expect_err("must fail on 400");
    let s = err.to_string();
    assert!(s.contains("400"), "expected 400 in error, got: {s}");
}

#[tokio::test]
async fn dna_has_dc_cap() {
    let channel = DiscordChannel::with_url("http://localhost", None).unwrap();
    assert_eq!(channel.channel_name(), "discord");
    assert!(!channel.supports_batching());
    let caps = channel.dna().caps();
    assert!(caps.contains("DC"), "expected DC in caps, got {caps}");
    assert!(caps.contains("PR"));
    assert!(caps.contains("AP"));
}
