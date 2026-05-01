//! Phase 2 token-tracker wiring integration witnesses.
//!
//! Drives `/v1/chat/completions` (sync) against a mock Anthropic that
//! returns `input_tokens=10, output_tokens=5`, with the AppState wired
//! to an in-memory [`kei_token_tracker::Store`]. Asserts that exactly
//! one [`TokenEvent`] is recorded after the call returns, with the
//! expected token counts.
//!
//! Companion to `openai_loop_wiring.rs` — that file proves the loop is
//! the production path; this one proves the loop's per-turn telemetry
//! reaches the tracker store.

mod common;

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use kei_cortex::routes::openai::openai_router;
use kei_cortex::state::{AppState, InvokerFactory};
use kei_cortex::AppConfig;
use kei_router::LlmRouter;
use kei_token_tracker::Store as TokenTracker;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

fn dummy_config() -> AppConfig {
    AppConfig::new(
        Some(0),
        Some("http://127.0.0.1".into()),
        Some(PathBuf::from("/tmp/kc-tok-tracking")),
        Some(PathBuf::from("/tmp/kc-led-tracking")),
        Some(PathBuf::from("/tmp/kc-pets-tracking")),
        Some(PathBuf::from("/tmp/kc-mem-tracking.sqlite")),
        Some(PathBuf::from("/tmp/kc-live2d-tracking")),
    )
}

/// Build a mock Anthropic server that returns input_tokens=10,
/// output_tokens=5 so the tracker row's counts are unambiguously
/// caused by THIS test (not the shared `"hi"` mock that returns 1/1).
fn spawn_custom_usage_mock() -> String {
    use axum::{routing::post, Json, Router};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tokio::net::TcpListener;
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("mock runtime");
        rt.block_on(async move {
            let body = serde_json::json!({
                "content": [{"type": "text", "text": "ack"}],
                "stop_reason": "end_turn",
                "usage": {"input_tokens": 10, "output_tokens": 5},
            });
            let app: Router = Router::new().route(
                "/v1/messages",
                post(move || {
                    let body = body.clone();
                    async move { Json(body) }
                }),
            );
            let listener =
                TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
                    .await
                    .expect("bind mock");
            let addr = listener.local_addr().expect("local_addr");
            tx.send(format!("http://{addr}/v1/messages")).unwrap();
            let _ = axum::serve(listener, app).await;
        });
    });
    rx.recv().expect("mock uri channel")
}

fn auth_request(method: &str, uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", "Bearer test-key")
        .body(body)
        .unwrap()
}

/// AppState wired to an explicit in-memory tracker handle so the test
/// body can read out recorded events directly. `default_invoker_factory`
/// is unused on the chat-completions path (HTTP invoker is built fresh
/// per call); we wire any factory to satisfy the constructor.
fn state_with_tracker(
    cfg: AppConfig,
    tracker: Arc<Mutex<TokenTracker>>,
) -> AppState {
    let factory: InvokerFactory = Arc::new(|| {
        Arc::new(NoopInvoker)
    });
    AppState::with_router_factory_and_tracker(
        cfg,
        "test-key".into(),
        Arc::new(LlmRouter::new()),
        factory,
        Some(tracker),
    )
}

/// Stand-in for the memory-review invoker factory — never called by
/// the chat-completions sync path. Returning `"Nothing to save."` keeps
/// the Invoker contract; the test does not exercise this code path.
struct NoopInvoker;

impl kei_cortex::agent::memory_review_task::Invoker for NoopInvoker {
    fn invoke(
        &self,
        _s: Vec<kei_cortex::agent::memory_nudge::Turn>,
        _p: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        Box::pin(async move { "Nothing to save.".into() })
    }
}

/// End-to-end: sync /v1/chat/completions records exactly one TokenEvent
/// with matching tokens (10/5) into the AppState's in-memory tracker.
#[tokio::test]
async fn sync_chat_completion_records_one_token_event() {
    let mock_uri = spawn_custom_usage_mock();
    std::env::set_var("ANTHROPIC_ENDPOINT", &mock_uri);
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
    std::env::set_var("KEI_API_KEY", "test-key");

    let store = TokenTracker::open_in_memory().expect("open in-memory tracker");
    let tracker = Arc::new(Mutex::new(store));
    let state = state_with_tracker(dummy_config(), tracker.clone());

    let app = openai_router().with_state(state);
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/chat/completions",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let _ = to_bytes(resp.into_body(), 65536).await.unwrap();

    // The record_event call is dispatched via `spawn_blocking` from the
    // sync collect_reply path — it returns BEFORE the spawn_blocking
    // future runs. Yield repeatedly until the tracker reports the row
    // (or fail the test on a generous timeout).
    let mut attempts = 0;
    let count = loop {
        let n = {
            let g = tracker.lock().unwrap();
            g.count().expect("count")
        };
        if n >= 1 {
            break n;
        }
        attempts += 1;
        if attempts > 200 {
            panic!("tracker never received an event (count stayed 0)");
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    };
    assert_eq!(count, 1, "expected exactly one TokenEvent, got {count}");

    let recent = {
        let g = tracker.lock().unwrap();
        g.list_recent(10).expect("list_recent")
    };
    assert_eq!(recent.len(), 1);
    let ev = &recent[0];
    assert_eq!(ev.input_tokens, 10, "input_tokens not captured");
    assert_eq!(ev.output_tokens, 5, "output_tokens not captured");
    assert!(
        ev.agent_id.starts_with("openai-chat-"),
        "agent_id format: {}",
        ev.agent_id
    );
    assert_eq!(ev.role, "kei-cortex-chat");
    assert_eq!(ev.source_kind.as_deref(), Some("chat-completions"));
}
