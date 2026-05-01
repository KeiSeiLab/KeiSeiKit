//! Runtime configuration for the cortex daemon.
//!
//! `AppConfig` is assembled once at startup from CLI arguments and handed to
//! the router via `AppState`. All paths are resolved to absolute at construct
//! time so handlers never have to re-resolve `~` or cwd.

use axum::http::HeaderValue;
use std::path::{Path, PathBuf};

/// Default listen port when `--port` is not provided.
pub const DEFAULT_PORT: u16 = 9797;

/// Default CORS origin when `--cors-origin` is not provided.
pub const DEFAULT_CORS_ORIGIN: &str = "https://keisei.app";

/// Default LLM provider when `--default-provider` is not provided.
pub const DEFAULT_PROVIDER: &str = "anthropic";

/// Errors from config assembly — surface to `main.rs` as non-zero exit.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cors_origin {0:?} is not a valid HTTP header value: {1}")]
    BadCorsOrigin(String, String),
}

/// Runtime configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// TCP port for the local HTTP listener. Bound to 127.0.0.1 only.
    pub port: u16,

    /// Single CORS origin the daemon will echo back. Exact-match; no wildcards.
    pub cors_origin: String,

    /// Path to the bearer-token file. Read once at startup.
    pub token_path: PathBuf,

    /// SQLite database holding the agent ledger (kei-ledger schema).
    pub ledger_path: PathBuf,

    /// Root directory holding `<user_id>.toml` pet manifests.
    pub pet_root: PathBuf,

    /// SQLite database holding pet conversation memory (kei-pet schema).
    pub memory_db: PathBuf,

    /// Directory containing bundled Cubism sample rigs (`haru/`, `mao/`,
    /// `hiyori/`, `mark/`). The portrait-stylize handler clones one of these
    /// subdirectories into `<live2d_samples_dir>/custom-<user_id>/` and swaps
    /// `texture_00.png` with the Flux-generated image.
    pub live2d_samples_dir: PathBuf,

    /// Working directory used by the chat handler to discover CLAUDE.md /
    /// AGENTS.md / SOUL.md context files (walked upward).
    pub cwd: PathBuf,

    /// Project root used for skill resolution (`<root>/.claude/skills/<name>/`)
    /// and as the chroot for `/fs/list` + `/term`. Defaults to `cwd`.
    pub project_root: PathBuf,

    /// Default LLM provider name when the request lacks `?provider=`.
    pub default_provider: String,

    /// SQLite database for per-turn token-event telemetry (kei-token-tracker
    /// schema). Defaults to `~/.keisei/token-events.sqlite`. Each successful
    /// chat / response / run handler records one [`TokenEvent`] here for
    /// nightly sleep-report aggregation.
    pub token_tracker_db: PathBuf,
}

impl AppConfig {
    /// Legacy 7-arg builder kept for tests. Panics on bad CORS.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: Option<u16>, cors_origin: Option<String>, token_path: Option<PathBuf>,
        ledger_path: Option<PathBuf>, pet_root: Option<PathBuf>, memory_db: Option<PathBuf>,
        live2d_samples_dir: Option<PathBuf>,
    ) -> Self {
        Self::try_new(
            port, cors_origin, token_path, ledger_path, pet_root, memory_db,
            live2d_samples_dir, None, None, None, None,
        )
        .expect("valid CORS origin")
    }

    /// 11-arg constructor; returns a `Result` so `main` can render clean errors.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        port: Option<u16>, cors_origin: Option<String>, token_path: Option<PathBuf>,
        ledger_path: Option<PathBuf>, pet_root: Option<PathBuf>, memory_db: Option<PathBuf>,
        live2d_samples_dir: Option<PathBuf>, cwd: Option<PathBuf>,
        project_root: Option<PathBuf>, default_provider: Option<String>,
        token_tracker_db: Option<PathBuf>,
    ) -> Result<Self, ConfigError> {
        let base = home_dir().join(".keisei");
        let cors = cors_origin.unwrap_or_else(|| DEFAULT_CORS_ORIGIN.to_string());
        validate_cors(&cors)?;
        let resolved_cwd = cwd.unwrap_or_else(default_cwd);
        let resolved_root = project_root.unwrap_or_else(|| resolved_cwd.clone());
        Ok(Self {
            port: port.unwrap_or(DEFAULT_PORT),
            cors_origin: cors,
            token_path: token_path.unwrap_or_else(|| base.join("cortex.token")),
            ledger_path: ledger_path.unwrap_or_else(|| base.join("ledger.sqlite")),
            pet_root: pet_root.unwrap_or_else(|| base.join("pets")),
            memory_db: memory_db.unwrap_or_else(|| base.join("pet-memory.sqlite")),
            live2d_samples_dir: live2d_samples_dir.unwrap_or_else(default_live2d_samples_dir),
            cwd: resolved_cwd,
            project_root: resolved_root,
            default_provider: default_provider.unwrap_or_else(|| DEFAULT_PROVIDER.to_string()),
            token_tracker_db: token_tracker_db
                .unwrap_or_else(|| base.join("token-events.sqlite")),
        })
    }
}

/// Validate that `cors_origin` parses as a `HeaderValue` — we crash early
/// rather than propagating an `expect` into `routes::build_router`.
fn validate_cors(origin: &str) -> Result<(), ConfigError> {
    match origin.parse::<HeaderValue>() {
        Ok(_) => Ok(()),
        Err(e) => Err(ConfigError::BadCorsOrigin(origin.to_string(), e.to_string())),
    }
}

/// Resolve the default live2d samples directory to an absolute path anchored
/// at the crate's `CARGO_MANIFEST_DIR` so the daemon works regardless of
/// cwd. If the resolved directory does not exist, we still return the path
/// — `main.rs` logs a warning and portrait uploads will 500 later with a
/// clear message (rather than a cryptic cwd-relative ENOENT).
fn default_live2d_samples_dir() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent());
    match repo_root {
        Some(root) => root.join("_ts_packages/packages/cortex-ui/public/live2d-models"),
        None => PathBuf::from("_ts_packages/packages/cortex-ui/public/live2d-models"),
    }
}

/// `HOME` with a plain-`.` fallback so tests that unset `HOME` still work.
fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Resolve the default cwd. Falls back to `.` when the OS refuses to give
/// one (rare; only on chroot setups without `/proc/self/cwd`).
fn default_cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Result<AppConfig, ConfigError> {
        AppConfig::try_new(None, None, None, None, None, None, None, None, None, None, None)
    }

    #[test]
    fn try_new_accepts_default_cors() {
        assert!(defaults().is_ok());
    }

    #[test]
    fn try_new_rejects_bad_cors() {
        let r = AppConfig::try_new(
            None, Some("line\nbreak".into()), None, None, None, None, None, None, None, None,
            None,
        );
        assert!(matches!(r, Err(ConfigError::BadCorsOrigin(_, _))));
    }

    #[test]
    fn default_live2d_dir_is_populated() {
        assert!(!defaults().unwrap().live2d_samples_dir.as_os_str().is_empty());
    }

    #[test]
    fn default_provider_is_anthropic() {
        assert_eq!(defaults().unwrap().default_provider, "anthropic");
    }

    #[test]
    fn project_root_defaults_to_cwd() {
        let cfg = AppConfig::try_new(
            None, None, None, None, None, None, None,
            Some(PathBuf::from("/tmp/xyz")), None, None, None,
        )
        .unwrap();
        assert_eq!(cfg.cwd, PathBuf::from("/tmp/xyz"));
        assert_eq!(cfg.project_root, PathBuf::from("/tmp/xyz"));
    }

    #[test]
    fn default_token_tracker_db_is_under_keisei_home() {
        let cfg = defaults().unwrap();
        assert!(cfg.token_tracker_db.ends_with("token-events.sqlite"));
    }
}
