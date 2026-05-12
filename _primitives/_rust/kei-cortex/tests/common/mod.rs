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
// wiremock unused-import guard — actual use is inside build_mock()
#[allow(unused_imports)]
use wiremock as _;

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

/// Handle to the process-wide mock Anthropic upstream.
///
/// (2026-05-12) Reimplemented on top of `wiremock` after the previous
/// hand-rolled axum + dedicated-thread implementation flaked under
/// macOS GitHub Actions runners — `error sending request for url
/// (http://127.0.0.1:PORT/v1/messages)` on `streaming_responses_runs_real_loop_not_stub`
/// + `sync_chat_completions_runs_real_loop_not_stub`. wiremock
/// production-grade HTTP mock removes the loopback / fd-limit races.
pub struct MockAnthropicServer {
    /// The owned `wiremock::MockServer` — its `Drop` shuts down the
    /// upstream listener. For singletons we leak it via `OnceLock` so
    /// it outlives every `#[tokio::test]` runtime in the binary.
    server: wiremock::MockServer,
    uri: String,
}

impl MockAnthropicServer {
    /// Base URL of the mock (`http://127.0.0.1:<port>/v1/messages`).
    /// Set this as `ANTHROPIC_ENDPOINT` to redirect upstream traffic.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Underlying wiremock server (rarely needed — exposed for tests
    /// that want to assert request shape via `received_requests`).
    #[allow(dead_code)]
    pub fn server(&self) -> &wiremock::MockServer {
        &self.server
    }
}

/// Spin up a wiremock server mounted with a canned `/v1/messages`
/// reply. Bind happens on `127.0.0.1:0` via wiremock's own listener,
/// which is reliable across macOS / Linux GitHub runners.
///
/// Includes a warm-up GET against the bound port so we only return
/// once the listener is actually accepting connections — this closes
/// the race where the first test under parallel `cargo test`
/// dispatches an HTTP request to the mock before its acceptor loop
/// is ready (manifests as `error sending request for url …`).
async fn build_mock(text: &str) -> MockAnthropicServer {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "content": [{"type": "text", "text": text}],
        "stop_reason": "end_turn",
        "usage": {"input_tokens": 1, "output_tokens": 1},
    });
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    // Warm-up probe: a HEAD against an unmounted route returns 404 but
    // confirms the listener is accepting. Retry briefly under cold CI.
    let probe_url = server.uri();
    for attempt in 0..20 {
        if reqwest::Client::new()
            .head(&probe_url)
            .timeout(std::time::Duration::from_millis(200))
            .send()
            .await
            .is_ok()
        {
            break;
        }
        if attempt == 19 {
            panic!("wiremock listener did not respond to warm-up probe after 1s");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    let uri = format!("{}/v1/messages", server.uri());
    MockAnthropicServer { server, uri }
}

/// Per-call mock variant. Spawns a fresh wiremock instance with the
/// given canned reply text. Each instance keeps its server alive for
/// the lifetime of the returned handle.
///
/// Implementation note: wiremock's hyper server is tied to the runtime
/// that called `MockServer::start().await`. We build a dedicated
/// **multi-thread** runtime on a helper OS-thread and keep it alive by
/// blocking on `pending()` — this guarantees the runtime keeps driving
/// async tasks (the previous `std::thread::park()` froze the worker
/// thread, starving wiremock of CPU and producing
/// `error sending request for url …` on CI under parallel tests).
pub fn mock_anthropic_responding_with(text: &'static str) -> MockAnthropicServer {
    let (tx, rx) = std::sync::mpsc::channel::<MockAnthropicServer>();
    let owned = text.to_string();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(2)
            .build()
            .expect("mock-runtime build");
        let mock = rt.block_on(async move { build_mock(&owned).await });
        tx.send(mock).expect("send mock back");
        // Hold the runtime open for the rest of the process — wiremock
        // needs it to accept incoming requests. `pending()` yields
        // forever; worker threads keep processing the hyper server.
        rt.block_on(std::future::pending::<()>());
    });
    rx.recv().expect("mock channel closed")
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
