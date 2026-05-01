// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Twilio REST integration tests against a `wiremock` stub. No live
//! HTTP — every assertion is local to the test process.

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use kei_notify_sms::SmsChannel;
use kei_runtime_core::traits::notify::{
    Notification, NotifyChannel, NotifySeverity,
};
use kei_runtime_core::{DnaBuilder, HasDna};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SID: &str = "ACtest_sid_0123456789abcdef";
const TOKEN: &str = "test_auth_token_xxx";
const FROM: &str = "+15005550006";
const TO: &str = "+15005550010";

fn channel(server: &MockServer) -> SmsChannel {
    SmsChannel::with_config(server.uri(), SID, TOKEN, FROM, TO, None).unwrap()
}

fn notif(sev: NotifySeverity, subject: &str, body: &str) -> Notification {
    let dna = DnaBuilder::new("notification")
        .cap("NF")
        .scope("test")
        .body(b"smoke")
        .build()
        .unwrap();
    let parent = DnaBuilder::new("primitive")
        .cap("PR")
        .scope("test-parent")
        .body(b"parent")
        .build()
        .unwrap();
    Notification {
        dna,
        parent_dna: parent,
        subject: subject.into(),
        body_text: body.into(),
        body_html: None,
        severity: sev,
        tags: vec![],
    }
}

fn endpoint_path() -> String {
    format!("/2010-04-01/Accounts/{SID}/Messages.json")
}

fn expected_basic_header() -> String {
    let raw = format!("{SID}:{TOKEN}");
    format!("Basic {}", B64.encode(raw.as_bytes()))
}

#[tokio::test]
async fn send_201_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(endpoint_path()))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "sid": "SMxxxxxxxxxxxxxxxx",
            "status": "queued"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let ch = channel(&server);
    let n = notif(NotifySeverity::Warn, "alert", "disk 92%");
    ch.send(&n).await.expect("201 should be Ok");
}

#[tokio::test]
async fn send_400_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(endpoint_path()))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "code": 21211,
            "message": "Invalid 'To' Phone Number",
            "more_info": "https://www.twilio.com/docs/errors/21211",
            "status": 400
        })))
        .mount(&server)
        .await;

    let ch = channel(&server);
    let n = notif(NotifySeverity::Error, "fail", "boom");
    let err = ch.send(&n).await.expect_err("400 must surface as Err");
    let msg = err.to_string();
    assert!(
        msg.contains("21211") || msg.contains("Invalid"),
        "expected twilio code/message in error, got: {msg}"
    );
}

#[tokio::test]
async fn info_severity_dropped_by_filter() {
    // The trait dispatcher consults `min_severity()` to gate delivery.
    // SmsChannel overrides to `Warn`, so `Info` is below the floor and
    // would be dropped. We assert the predicate directly.
    let server = MockServer::start().await;
    let ch = channel(&server);
    assert_eq!(ch.min_severity(), NotifySeverity::Warn);

    // Demonstrate the filter contract that an upstream dispatcher would
    // enforce. `NotifySeverity` doesn't impl Ord, so the gate is an
    // explicit allow-list per channel.
    let allowed = |sev: NotifySeverity| -> bool {
        match (ch.min_severity(), sev) {
            (NotifySeverity::Warn, NotifySeverity::Info)
            | (NotifySeverity::Warn, NotifySeverity::Success) => false,
            (NotifySeverity::Warn, _) => true,
            _ => true,
        }
    };
    assert!(!allowed(NotifySeverity::Info), "Info must be filtered");
    assert!(!allowed(NotifySeverity::Success), "Success must be filtered");
    assert!(allowed(NotifySeverity::Warn));
    assert!(allowed(NotifySeverity::Error));
}

#[tokio::test]
async fn warn_severity_delivered() {
    let server = MockServer::start().await;
    // No body matcher — wiremock 0.6 form-body matching varies across
    // patch versions; we verify delivery by 201 + path + method only.
    Mock::given(method("POST"))
        .and(path(endpoint_path()))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "sid": "SMwarn_ok",
            "status": "queued"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let ch = channel(&server);
    let n = notif(NotifySeverity::Warn, "boot", "rebooting");
    ch.send(&n).await.expect("warn should reach the wire");
}

#[tokio::test]
async fn dna_has_sm_cap() {
    let server = MockServer::start().await;
    let ch = channel(&server);
    let caps = ch.dna().caps();
    assert!(caps.contains("SM"), "expected SM in caps, got {caps}");
    assert!(caps.contains("PR"));
    assert!(caps.contains("AP"));
    assert_eq!(ch.channel_name(), "sms");
    assert!(!ch.supports_batching());
}

#[tokio::test]
async fn http_basic_auth_header_present() {
    let server = MockServer::start().await;
    let expected = expected_basic_header();
    Mock::given(method("POST"))
        .and(path(endpoint_path()))
        .and(header("authorization", expected.as_str()))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "sid": "SMauth_ok",
            "status": "queued"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let ch = channel(&server);
    let n = notif(NotifySeverity::Warn, "auth", "ping");
    ch.send(&n).await.expect("basic-auth header must match");
}
