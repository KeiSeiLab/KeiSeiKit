//! Model-id and endpoint resolution for the Anthropic client.
//!
//! Three-tier model fallback (env → kei-model registry → literal) and the
//! companion endpoint resolver live here so the HTTP client (`anthropic.rs`)
//! and the tool-use invoker (`anthropic_invoker.rs`) consume a single
//! resolver surface. Stateless: every call re-reads the underlying env
//! var so rotation works without restarting the process.

use std::borrow::Cow;

/// Literal model-id fallback. Used when `ANTHROPIC_MODEL` is unset and the
/// kei-model registry cannot resolve the `kei-cortex-default` role. Stays
/// in sync with `selectors.toml [defaults] kei-cortex-default` at Wave 55.
pub const MODEL_FALLBACK: &str = "claude-haiku-4-5-20251001";

/// Selector role consulted when no env override is set. Matches the
/// `selectors.toml [defaults]` key the W55 SSoT pins for cortex-default.
const RESOLVE_ROLE: &str = "kei-cortex-default";

/// Anthropic API endpoint (v1 messages). Compile-time default; the
/// runtime endpoint resolves through `endpoint()` so tests can divert
/// upstream traffic to a mock server via `ANTHROPIC_ENDPOINT`.
pub const ENDPOINT: &str = "https://api.anthropic.com/v1/messages";

/// Anthropic API version header value.
pub const API_VERSION: &str = "2023-06-01";

/// Resolve the Claude model id (W55 Stage 2, mirrors W55b kei-spawn).
///
/// Three-tier fallback: `ANTHROPIC_MODEL` env → kei-model registry role
/// `kei-cortex-default` → literal `MODEL_FALLBACK`. Stateless re-read on
/// every call; callers MUST NOT cache the result.
pub fn default_model() -> Cow<'static, str> {
    if let Ok(env_val) = std::env::var("ANTHROPIC_MODEL") {
        if !env_val.is_empty() {
            return Cow::Owned(env_val);
        }
    }
    if let Some(id) = try_registry_default() {
        return Cow::Owned(id);
    }
    Cow::Borrowed(MODEL_FALLBACK)
}

/// Inner registry resolve — returns `None` on any failure (missing TOML,
/// parse error, no role match). Kept silent: the public `default_model`
/// owns the fallback decision and never panics.
fn try_registry_default() -> Option<String> {
    let no_caps: &[kei_model::Capability] = &[];
    let path = kei_model::Registry::resolve_path(None).ok()?;
    let registry = kei_model::Registry::load(&path).ok()?;
    let res = kei_model::resolve(RESOLVE_ROLE, None, no_caps, &registry, None).ok()?;
    Some(res.model.id)
}

/// Resolve the live Anthropic endpoint. Reads `ANTHROPIC_ENDPOINT` at
/// every call (env-rotation friendly, mirrors the `ANTHROPIC_API_KEY`
/// read path in `open_stream`); falls back to the const `ENDPOINT`
/// when the env var is unset or empty so production behaviour is
/// unchanged when nothing is exported.
pub fn endpoint() -> Cow<'static, str> {
    match std::env::var("ANTHROPIC_ENDPOINT") {
        Ok(v) if !v.is_empty() => Cow::Owned(v),
        _ => Cow::Borrowed(ENDPOINT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Serializes env-var mutation across `default_model_*` tests so the
    /// process-wide `ANTHROPIC_MODEL` slot is not raced when cargo test
    /// runs threads in parallel.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn default_model_uses_env_when_set() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::set_var("ANTHROPIC_MODEL", "claude-test-env-pin");
        let got = default_model();
        std::env::remove_var("ANTHROPIC_MODEL");
        assert_eq!(got.as_ref(), "claude-test-env-pin");
    }

    #[test]
    fn default_model_falls_back_to_literal() {
        let _g = ENV_LOCK.lock().unwrap();
        std::env::remove_var("ANTHROPIC_MODEL");
        std::env::set_var("KEI_MODEL_REGISTRY", "/nonexistent/models.toml");
        let got = default_model();
        std::env::remove_var("KEI_MODEL_REGISTRY");
        assert_eq!(got.as_ref(), MODEL_FALLBACK);
    }
}
