//! End-to-end test for the CLI adapter wiring.
//!
//! Verifies the adapter constructs a [`MessageEvent`] with the right shape
//! and that an `OutboundMessage` can be sent without error. Full stdin/stdout
//! piping is platform-specific and exercised in integration tests; here we
//! cover the in-process contract.

use kei_gateway::adapters::base::{OutboundMessage, PlatformAdapter};
use kei_gateway::message::Platform;

#[cfg(feature = "cli")]
use kei_gateway::adapters::cli::CliAdapter;

#[cfg(feature = "cli")]
#[tokio::test]
async fn cli_adapter_reports_correct_platform() {
    let adapter = CliAdapter::default();
    assert_eq!(adapter.platform(), Platform::Cli);
}

#[cfg(feature = "cli")]
#[tokio::test]
async fn cli_adapter_connect_is_noop() {
    let adapter = CliAdapter::default();
    adapter.connect().await.expect("connect should never fail");
}

#[cfg(feature = "cli")]
#[tokio::test]
async fn cli_adapter_send_writes_to_stdout_without_error() {
    // The actual stdout output is captured by the harness; we just assert no
    // I/O error and a successful SendResult.
    let adapter = CliAdapter::default();
    let res = adapter
        .send(OutboundMessage::text("hello from kei-gateway"))
        .await
        .expect("send should succeed");
    assert!(res.success);
    assert!(res.error.is_none());
}

#[cfg(feature = "cli")]
#[tokio::test]
async fn cli_adapter_with_custom_chat_id() {
    let adapter = CliAdapter::new("custom-stdin");
    assert_eq!(adapter.chat_id, "custom-stdin");
    assert_eq!(adapter.platform(), Platform::Cli);
}

#[cfg(feature = "cli")]
#[tokio::test]
async fn outbound_message_target_binding() {
    let msg = OutboundMessage::text("hi").with_target("chat-9".into(), Some("thread-1".into()));
    assert_eq!(msg.chat_id.as_deref(), Some("chat-9"));
    assert_eq!(msg.thread_id.as_deref(), Some("thread-1"));
    assert_eq!(msg.text, "hi");
}

#[cfg(not(feature = "cli"))]
#[test]
fn cli_disabled_compiles() {
    // Sanity test: when the `cli` feature is off, the test suite still builds.
}
