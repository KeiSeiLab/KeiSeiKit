//! State-machine tests for `DaytonaBackend::acquire`.
//!
//! Each test stands up a wiremock server, configures the responses Daytona
//! would emit for one of the resume-or-create branches, and asserts that
//! the right HTTP calls were made.

use kei_backend_daytona::{
    Backend, CreateSandboxSpec, DaytonaBackend, DaytonaClient, Resources, Sandbox, SandboxState,
};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TASK_ID: &str = "task-abc";

fn sandbox_json(state: &str) -> serde_json::Value {
    json!({
        "id": "sb-id-1",
        "name": format!("kei-{}", TASK_ID),
        "state": state,
        "image": "ubuntu:24.04",
        "resources": { "cpu": 1, "memory": 5, "disk": 10 },
        "labels": { "kei_task_id": TASK_ID }
    })
}

async fn build(server: &MockServer) -> DaytonaBackend {
    let client = DaytonaClient::new("test-key", server.uri()).expect("build client");
    DaytonaBackend::new(client, "ubuntu:24.04")
}

#[tokio::test]
async fn acquire_running_sandbox_skips_start() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("running")))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;

    let handle = backend.acquire(TASK_ID).await.expect("acquire");
    assert_eq!(handle.name, format!("kei-{}", TASK_ID));
    // No /start mock registered → if backend called it, the test fails on
    // the `expect(1)` GET assertion or on a 404 from wiremock.
}

#[tokio::test]
async fn acquire_hibernated_sandbox_calls_start() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("hibernated")))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("/sandbox/kei-{}/start", TASK_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;

    backend.acquire(TASK_ID).await.expect("acquire");
}

#[tokio::test]
async fn acquire_stopped_sandbox_calls_start() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("stopped")))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("/sandbox/kei-{}/start", TASK_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;
    backend.acquire(TASK_ID).await.expect("acquire");
}

#[tokio::test]
async fn acquire_missing_sandbox_creates_new() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/sandbox"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("running")))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;

    let handle = backend.acquire(TASK_ID).await.expect("acquire");
    assert!(handle.name.contains(TASK_ID));
}

#[tokio::test]
async fn acquire_error_state_fails() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("error")))
        .mount(&server)
        .await;
    let backend = build(&server).await;
    let err = backend.acquire(TASK_ID).await.expect_err("should fail");
    let msg = format!("{err}");
    assert!(msg.contains("Error state"), "msg={msg}");
}

#[tokio::test]
async fn release_persistent_calls_stop() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/sandbox/kei-{}/stop", TASK_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;
    let handle = kei_backend_daytona::SandboxHandle {
        name: format!("kei-{}", TASK_ID),
        image: "ubuntu:24.04".into(),
    };
    backend.release(handle, true).await.expect("release");
}

#[tokio::test]
async fn release_ephemeral_calls_delete() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let backend = build(&server).await;
    let handle = kei_backend_daytona::SandboxHandle {
        name: format!("kei-{}", TASK_ID),
        image: "ubuntu:24.04".into(),
    };
    backend.release(handle, false).await.expect("release");
}

#[test]
fn create_spec_builder_sets_label_and_persistent() {
    let spec = CreateSandboxSpec::new("ubuntu:24.04", "kei-x")
        .with_resources(Resources { cpu: 2, memory: 8, disk: 10 })
        .with_label("kei_task_id", "x")
        .with_persistent();
    assert_eq!(spec.auto_stop_interval, 0);
    assert_eq!(spec.resources.cpu, 2);
    assert_eq!(spec.labels.get("kei_task_id"), Some(&"x".to_string()));
}

#[test]
fn sandbox_state_deserializes_unknown_as_unknown() {
    let s: Sandbox = serde_json::from_value(json!({
        "id": "x", "name": "y", "state": "future-value-we-do-not-know",
    }))
    .expect("deserialize");
    assert_eq!(s.state, SandboxState::Unknown);
}
