//! Server — spawn `llama-server` and return a managed handle.
//!
//! Default --host 127.0.0.1 ALWAYS. Non-localhost host strings are
//! rejected with `Error::InvalidHost`; this primitive is a daemon
//! spawner, never a remote-exposure tool.

use crate::error::{Error, Result};
use crate::runner::{Runner, ServerHandle};
use serde::{Deserialize, Serialize};
use std::path::Path;

const DEFAULT_HOST: &str = "127.0.0.1";
const ALLOWED_HOSTS: &[&str] = &["127.0.0.1", "localhost", "::1"];

/// Inputs to a `server` invocation.
#[derive(Debug, Clone)]
pub struct ServerOpts {
    pub host: String,
    pub port: u16,
}

impl Default for ServerOpts {
    fn default() -> Self {
        Self { host: DEFAULT_HOST.into(), port: 8080 }
    }
}

/// JSON-friendly summary returned by the CLI on spawn.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerInfo {
    pub pid: u32,
    pub port: u16,
    pub host: String,
    pub openai_compat_url: String,
}

/// Reject anything that isn't an allow-listed loopback host.
/// Pure fn — exercised directly by tests.
pub fn validate_host(host: &str) -> Result<()> {
    if ALLOWED_HOSTS.iter().any(|h| h.eq_ignore_ascii_case(host)) {
        Ok(())
    } else {
        Err(Error::InvalidHost { host: host.to_string() })
    }
}

/// Build the argv for `llama-server -m <model> --host <host> --port <port>`.
pub fn build_server_args(model: &Path, opts: &ServerOpts) -> Vec<String> {
    vec![
        "-m".into(),
        model.to_string_lossy().into_owned(),
        "--host".into(),
        opts.host.clone(),
        "--port".into(),
        opts.port.to_string(),
    ]
}

/// Build a ServerInfo from a handle + opts.
pub fn info_from_handle(handle: &ServerHandle, opts: &ServerOpts) -> ServerInfo {
    ServerInfo {
        pid: handle.pid,
        port: handle.port,
        host: opts.host.clone(),
        openai_compat_url: format!("http://{}:{}/v1", opts.host, opts.port),
    }
}

/// Validate host then spawn `llama-server`. Caller owns the handle;
/// dropping it kills the child.
pub async fn start_server<R: Runner + ?Sized>(
    runner: &R,
    bin: &str,
    model: &Path,
    opts: &ServerOpts,
) -> Result<ServerHandle> {
    validate_host(&opts.host)?;
    if !model.exists() {
        return Err(Error::ModelNotFound { path: model.to_path_buf() });
    }
    let args = build_server_args(model, opts);
    runner.spawn_server(bin, &args, opts.port).await
}
