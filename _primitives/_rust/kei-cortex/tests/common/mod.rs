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
