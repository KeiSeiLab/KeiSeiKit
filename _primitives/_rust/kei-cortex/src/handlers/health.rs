//! Unauthenticated liveness probe.

/// `GET /healthz` → `"ok"` (text/plain). Always returns 200 OK.
pub async fn healthz() -> &'static str {
    "ok"
}
