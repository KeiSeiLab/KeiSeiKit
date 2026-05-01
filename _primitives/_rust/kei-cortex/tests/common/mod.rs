//! Shared test harness: spins up the router on an ephemeral port and hands
//! back the base URL + bearer token + config to the test body.
//!
//! Every integration-test file includes this module with `mod common;`, so
//! items unused by one file still count as live via the others. The
//! `#![allow(dead_code)]` silences per-file false positives.

#![allow(dead_code)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use kei_cortex::{auth, build_router, AppConfig, AppState};
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// Minimal valid pet.toml used by multiple tests.
pub const MINIMAL_PET_TOML: &str = r#"
schema = 1

[identity]
pet_name    = "Kei"
user_name   = "Alex"
addressing  = "by-name"
languages   = ["en"]

[voice]
tone_primary    = "neutral"
tone_secondary  = []
humor_style     = "none"
humor_frequency = "rare"

[edge]
profanity            = "never"
profanity_languages  = []
directness           = "balanced"
initiative           = "wait"

[forbidden]
topics        = []
tone_patterns = []

[meta]
schema_version_written_by = "kei-pet 0.1.0"
created_at                = "2026-04-23T12:30:00Z"
last_tuned                = "2026-04-23T12:30:00Z"
tune_count                = 0
"#;

/// Handle returned to each test; dropping stops the server.
pub struct TestServer {
    pub base_url: String,
    pub token: String,
    pub config: AppConfig,
    pub _tmp: TempDir,
    handle: Option<JoinHandle<()>>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(h) = self.handle.take() {
            h.abort();
        }
    }
}

/// Spin up the router on 127.0.0.1 with a random port.
pub async fn spawn() -> TestServer {
    let tmp = tempfile::tempdir().expect("tempdir");
    let base = tmp.path().to_path_buf();
    let config = AppConfig::new(
        Some(0),
        Some("https://keisei.app".to_string()),
        Some(base.join("cortex.token")),
        Some(base.join("ledger.sqlite")),
        Some(base.join("pets")),
        Some(base.join("pet-memory.sqlite")),
        Some(base.join("live2d-samples")),
    );
    std::fs::create_dir_all(&config.pet_root).unwrap();
    let token = auth::generate_token();
    auth::save_token(&config.token_path, &token).unwrap();

    let state = AppState::new(config.clone(), token.clone());
    let router = build_router(state);
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .await
        .unwrap();
    let actual = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    // Give axum a tick to start accepting connections.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    TestServer {
        base_url: format!("http://{}", actual),
        token,
        config,
        _tmp: tmp,
        handle: Some(handle),
    }
}

/// Write a minimal pet.toml for `user_id` under `<pet_root>/<user_id>.toml`.
pub fn write_minimal_pet(pet_root: &PathBuf, user_id: &str) {
    let path = pet_root.join(format!("{user_id}.toml"));
    std::fs::write(&path, MINIMAL_PET_TOML).unwrap();
}

/// Build an async reqwest client.
pub fn async_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
}

/// Handle to the process-wide mock Anthropic upstream. The server runs
/// on a dedicated OS-thread runtime that outlives every `#[tokio::test]`
/// runtime in the binary, so the listener never closes between tests.
pub struct MockAnthropicServer {
    uri: String,
}

impl MockAnthropicServer {
    /// Base URL of the mock (`http://127.0.0.1:<port>/v1/messages`).
    /// Set this as `ANTHROPIC_ENDPOINT` to redirect upstream traffic.
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// Build the canned-reply axum router used by the mock. Same body for
/// every POST so concurrent tests can share one server safely.
fn build_mock_router(text: &str) -> axum::Router {
    use axum::{routing::post, Json, Router};
    let body = serde_json::json!({
        "content": [{"type": "text", "text": text}],
        "stop_reason": "end_turn",
        "usage": {"input_tokens": 1, "output_tokens": 1},
    });
    Router::new().route(
        "/v1/messages",
        post(move || {
            let body = body.clone();
            async move { Json(body) }
        }),
    )
}

/// Spin up the mock listener on a dedicated thread+runtime, return the
/// resolved URI once it is bound and accepting. Kept private — tests
/// reach it through `mock_anthropic_responding_with` (per-call wrapper)
/// or `shared_mock_anthropic` (lazy singleton).
fn spawn_mock_on_dedicated_thread(text: &'static str) -> String {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let owned_text = text.to_string();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("mock-runtime build");
        rt.block_on(async move {
            let app = build_mock_router(&owned_text);
            let listener =
                TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
                    .await
                    .expect("bind mock listener");
            let actual = listener.local_addr().expect("local_addr");
            let uri = format!("http://{actual}/v1/messages");
            tx.send(uri).expect("send mock uri");
            // Server runs forever on this thread's runtime.
            let _ = axum::serve(listener, app).await;
        });
    });
    rx.recv().expect("mock uri channel closed")
}

/// Per-call mock variant. Spawns a fresh dedicated-thread mock so every
/// invocation gets a unique URI and reply text. Useful for tests that
/// want to vary the canned content; tests that just need any `200 OK`
/// envelope should prefer `shared_mock_anthropic`.
pub fn mock_anthropic_responding_with(text: &'static str) -> MockAnthropicServer {
    let uri = spawn_mock_on_dedicated_thread(text);
    MockAnthropicServer { uri }
}

/// Process-wide shared mock Anthropic server. Initialised on first call
/// and kept alive for the rest of the test binary so concurrent tests
/// can share one upstream URI without racing through the global
/// `ANTHROPIC_ENDPOINT` env var. All tests get the same canned `"hi"`
/// reply, which is enough for shape-only assertions.
pub fn shared_mock_anthropic() -> &'static MockAnthropicServer {
    static SHARED: std::sync::OnceLock<MockAnthropicServer> = std::sync::OnceLock::new();
    SHARED.get_or_init(|| mock_anthropic_responding_with("hi"))
}
