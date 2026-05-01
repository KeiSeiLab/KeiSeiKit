//! End-to-end smoke test against the real Daytona service.
//!
//! Skipped by default. To run:
//!
//! ```bash
//! export DAYTONA_API_KEY=...
//! export DAYTONA_BASE_URL=https://app.daytona.io/api
//! cargo test -p kei-backend-daytona --test integration_smoke -- --ignored --nocapture
//! ```
//!
//! The test acquires a sandbox keyed by a fixed task id, runs `echo hi`,
//! then **stops** (does NOT delete) so that the next run exercises the
//! resume-from-hibernated branch.

use kei_backend_daytona::{Backend, DaytonaBackend, DaytonaClient};

const SMOKE_TASK_ID: &str = "kei-backend-daytona-smoke";
const DEFAULT_IMAGE: &str = "ubuntu:24.04";

fn maybe_skip() -> Option<(String, String)> {
    let key = std::env::var("DAYTONA_API_KEY").ok()?;
    let url = std::env::var("DAYTONA_BASE_URL")
        .unwrap_or_else(|_| "https://app.daytona.io/api".into());
    Some((key, url))
}

#[tokio::test]
#[ignore]
async fn smoke_acquire_exec_release() {
    let (key, base) = match maybe_skip() {
        Some(v) => v,
        None => {
            eprintln!("DAYTONA_API_KEY not set; skipping smoke test");
            return;
        }
    };
    let client = DaytonaClient::new(key, base).expect("client");
    let backend = DaytonaBackend::new(client, DEFAULT_IMAGE);

    let handle = backend
        .acquire(SMOKE_TASK_ID)
        .await
        .expect("acquire failed");
    eprintln!("smoke: acquired {}", handle.name);

    let out = backend
        .exec(&handle, "echo hi")
        .await
        .expect("exec failed");
    eprintln!("smoke: exec exit={} stdout={:?}", out.exit_code, out.stdout);
    assert_eq!(out.exit_code, 0);
    assert!(out.stdout.contains("hi"), "stdout was {:?}", out.stdout);

    backend
        .release(handle, /* persist */ true)
        .await
        .expect("release failed");
    eprintln!("smoke: released (persisted)");
}

#[tokio::test]
#[ignore]
async fn smoke_resume_existing_after_stop() {
    let (key, base) = match maybe_skip() {
        Some(v) => v,
        None => {
            eprintln!("DAYTONA_API_KEY not set; skipping resume test");
            return;
        }
    };
    let client = DaytonaClient::new(key, base).expect("client");
    let backend = DaytonaBackend::new(client, DEFAULT_IMAGE);

    // First acquire (creates or resumes).
    let h1 = backend.acquire(SMOKE_TASK_ID).await.expect("first acquire");
    backend
        .release(h1, /* persist */ true)
        .await
        .expect("first release");

    // Second acquire on same task_id MUST resume the same sandbox.
    let h2 = backend
        .acquire(SMOKE_TASK_ID)
        .await
        .expect("second acquire");
    let out = backend.exec(&h2, "echo resumed").await.expect("exec");
    assert_eq!(out.exit_code, 0);
    backend.release(h2, true).await.expect("second release");
}
