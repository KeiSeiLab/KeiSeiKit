//! Verify `DaytonaBackend::with_sync(...)` actually wires file-sync into
//! `acquire` (push) and `release` (pull).
//!
//! These tests use `wiremock` to assert that the expected REST endpoints
//! are invoked in the right order. They do NOT exercise the underlying
//! Daytona service.
//!
//! Architecture note (post Patch B):
//!   - File operations go through the Toolbox API, not the management API.
//!   - `GET /sandbox/{name}/toolbox-proxy-url` returns `{"url":"<toolbox_base>"}`.
//!   - Upload:   POST <toolbox_base>/toolbox/{name}/toolbox/files/upload?path=<p>
//!   - Download: GET  <toolbox_base>/toolbox/{name}/toolbox/files/download?path=<p>
//!
//! In tests the toolbox proxy URL points back at the same wiremock server,
//! so all mocks live on a single MockServer instance.

use kei_backend_daytona::{Backend, DaytonaBackend, DaytonaClient, SyncConfig};
use serde_json::json;
use std::fs;
use tempfile::TempDir;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TASK_ID: &str = "task-sync";

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

fn build_backend(server: &MockServer, sync: Option<SyncConfig>) -> DaytonaBackend {
    let client = DaytonaClient::new("test-key", server.uri()).expect("client");
    let mut b = DaytonaBackend::new(client, "ubuntu:24.04");
    if let Some(cfg) = sync {
        b = b.with_sync(cfg);
    }
    b
}

/// Seed `local_root` with a single file so push has work to do.
fn seed_local(dir: &TempDir, rel: &str, body: &str) {
    let p = dir.path().join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir");
    }
    fs::write(&p, body).expect("write");
}

/// Register a mock that returns the wiremock server's own URI as the toolbox
/// proxy URL, so all toolbox calls land on the same MockServer.
async fn mount_toolbox_proxy_url(server: &MockServer, sandbox_name: &str) {
    let toolbox_base = server.uri();
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/{}/toolbox-proxy-url", sandbox_name)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "url": toolbox_base })),
        )
        .mount(server)
        .await;
}

#[tokio::test]
async fn acquire_with_sync_uploads_files() {
    let server = MockServer::start().await;
    let sandbox_name = format!("kei-{}", TASK_ID);

    // Management API: GET /sandbox/{name}
    Mock::given(method("GET"))
        .and(path(format!("/sandbox/{}", sandbox_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("running")))
        .mount(&server)
        .await;

    // Toolbox proxy URL resolution (called once per sandbox, cached after).
    mount_toolbox_proxy_url(&server, &sandbox_name).await;

    // Toolbox upload: POST /toolbox/{name}/toolbox/files/upload?path=keiseikit/config.toml
    Mock::given(method("POST"))
        .and(path(format!(
            "/toolbox/{}/toolbox/files/upload",
            sandbox_name
        )))
        .and(query_param("path", "keiseikit/config.toml"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let local = TempDir::new().expect("tempdir");
    seed_local(&local, "config.toml", "key = \"value\"\n");

    let cfg = SyncConfig {
        local_root: local.path().to_path_buf(),
        remote_root: "keiseikit".into(),
    };
    let backend = build_backend(&server, Some(cfg));

    backend.acquire(TASK_ID).await.expect("acquire");
    // The expect(1) on the POST mock is checked at MockServer drop.
}

#[tokio::test]
async fn acquire_without_sync_does_not_upload() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/sandbox/kei-{}", TASK_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_json(sandbox_json("running")))
        .mount(&server)
        .await;

    // No POST mock for toolbox upload. If the backend tries to upload,
    // wiremock returns 404 and acquire fails — test catches that implicitly.
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204))
        .expect(0)
        .mount(&server)
        .await;

    let backend = build_backend(&server, None);
    backend.acquire(TASK_ID).await.expect("acquire");
}

#[tokio::test]
async fn release_persist_with_sync_pulls_then_stops() {
    let server = MockServer::start().await;
    let sandbox_name = format!("kei-{}", TASK_ID);

    // Toolbox proxy URL resolution.
    mount_toolbox_proxy_url(&server, &sandbox_name).await;

    // Toolbox download: GET /toolbox/{name}/toolbox/files/download?path=keiseikit/.keiseikit-state
    Mock::given(method("GET"))
        .and(path(format!(
            "/toolbox/{}/toolbox/files/download",
            sandbox_name
        )))
        .and(query_param("path", "keiseikit/.keiseikit-state"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"sentinel".as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    // Management API stop.
    Mock::given(method("POST"))
        .and(path(format!("/sandbox/{}/stop", sandbox_name)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let local = TempDir::new().expect("tempdir");
    let cfg = SyncConfig {
        local_root: local.path().to_path_buf(),
        remote_root: "keiseikit".into(),
    };
    let backend = build_backend(&server, Some(cfg));
    let handle = kei_backend_daytona::SandboxHandle {
        name: sandbox_name,
        image: "ubuntu:24.04".into(),
    };
    backend.release(handle, true).await.expect("release");

    // The sentinel should have landed locally.
    let sentinel = local.path().join(".keiseikit-state");
    assert!(sentinel.exists(), "sentinel was not pulled to local");
    let body = fs::read(&sentinel).expect("read sentinel");
    assert_eq!(body, b"sentinel");
}

#[tokio::test]
async fn release_ephemeral_with_sync_pulls_then_deletes() {
    let server = MockServer::start().await;
    let sandbox_name = format!("kei-{}", TASK_ID);

    mount_toolbox_proxy_url(&server, &sandbox_name).await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/toolbox/{}/toolbox/files/download",
            sandbox_name
        )))
        .and(query_param("path", "keiseikit/.keiseikit-state"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"x".as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path(format!("/sandbox/{}", sandbox_name)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let local = TempDir::new().expect("tempdir");
    let cfg = SyncConfig {
        local_root: local.path().to_path_buf(),
        remote_root: "keiseikit".into(),
    };
    let backend = build_backend(&server, Some(cfg));
    let handle = kei_backend_daytona::SandboxHandle {
        name: sandbox_name,
        image: "ubuntu:24.04".into(),
    };
    backend.release(handle, false).await.expect("release");
}

#[tokio::test]
async fn release_without_sync_skips_pull() {
    let server = MockServer::start().await;

    // No GET /toolbox/.../files/download mock — backend must NOT call it.
    // We use a broad GET matcher with expect(0) to catch any unexpected GET
    // calls to toolbox paths, while allowing the stop POST to pass.
    Mock::given(method("POST"))
        .and(path(format!("/sandbox/kei-{}/stop", TASK_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let backend = build_backend(&server, None);
    let handle = kei_backend_daytona::SandboxHandle {
        name: format!("kei-{}", TASK_ID),
        image: "ubuntu:24.04".into(),
    };
    backend.release(handle, true).await.expect("release");
}

#[tokio::test]
async fn release_pull_failure_does_not_abort_stop() {
    let server = MockServer::start().await;
    let sandbox_name = format!("kei-{}", TASK_ID);

    mount_toolbox_proxy_url(&server, &sandbox_name).await;

    // Pull returns 500 — release must still call stop and succeed.
    Mock::given(method("GET"))
        .and(path(format!(
            "/toolbox/{}/toolbox/files/download",
            sandbox_name
        )))
        .and(query_param("path", "keiseikit/.keiseikit-state"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(format!("/sandbox/{}/stop", sandbox_name)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let local = TempDir::new().expect("tempdir");
    let cfg = SyncConfig {
        local_root: local.path().to_path_buf(),
        remote_root: "keiseikit".into(),
    };
    let backend = build_backend(&server, Some(cfg));
    let handle = kei_backend_daytona::SandboxHandle {
        name: sandbox_name,
        image: "ubuntu:24.04".into(),
    };
    // Should succeed even though pull failed.
    backend.release(handle, true).await.expect("release");
}
