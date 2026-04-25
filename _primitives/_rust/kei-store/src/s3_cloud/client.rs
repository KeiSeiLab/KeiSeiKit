//! aws-sdk-s3 client builder for the S3 cloud backend.
//!
//! Wraps `aws_config::defaults()` + optional endpoint override for
//! R2 / MinIO / Wasabi / any S3-compat provider. Credential chain is the
//! AWS default — env vars, `~/.aws/credentials`, IMDS — we do NOT invent
//! a new credential format (RULE 0.8 secrets-single-source honoured).
//!
//! Security invariants (v0.21.1):
//!
//!   * `validate_endpoint` rejects loopback / link-local / metadata URLs
//!     unless `KEI_STORE_S3_ALLOW_INTERNAL=1` is set, and plain-HTTP
//!     unless `KEI_STORE_S3_ALLOW_INSECURE=1` is set. Closes the SSRF /
//!     IMDS-leak surface where an operator-controlled `KEI_STORE_S3_ENDPOINT`
//!     pointed at `http://169.254.169.254` would cause the AWS default
//!     credential chain to sign requests against the instance metadata
//!     endpoint (and leak IMDS creds to the attacker's server).
//!
//!   * When the S3Cfg names `access_key_env` + `secret_key_env` env vars,
//!     we build an explicit `Credentials` provider and overlay it on the
//!     SDK builder. Without this wiring the two fields were silently dead
//!     (critic HIGH-2); with it, a user pointing the backend at MinIO can
//!     name a dedicated key pair via env instead of re-using the ambient
//!     AWS chain.
//!
//!   * For NON-AWS endpoints (anything with a custom endpoint URL) we
//!     REQUIRE explicit `access_key_env` + `secret_key_env`. Otherwise the
//!     default credential chain (which includes IMDS) would still fire —
//!     defeating the SSRF guard. Real-AWS paths (no endpoint override)
//!     keep the default chain.

use crate::config::S3Cfg;
use anyhow::{anyhow, bail, Result};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;

/// Resolve the effective endpoint URL:
///   1. `KEI_STORE_S3_ENDPOINT` env var (runtime override for tests / R2)
///   2. `cfg.endpoint` (TOML config)
///   3. None → aws-sdk-s3 default (real AWS)
pub fn effective_endpoint(cfg: &S3Cfg) -> Option<String> {
    if let Ok(url) = std::env::var("KEI_STORE_S3_ENDPOINT") {
        if !url.is_empty() {
            return Some(url);
        }
    }
    cfg.endpoint.clone()
}

/// SSRF / IMDS-leak guard. Rejects unsafe endpoint URLs unless the caller
/// has explicitly opted in via env.
pub fn validate_endpoint(endpoint: &str) -> Result<()> {
    let lower = endpoint.to_ascii_lowercase();
    let (scheme, rest) = lower
        .split_once("://")
        .ok_or_else(|| anyhow!(
            "endpoint rejected: {:?} — missing scheme (expected http:// or https://)",
            endpoint
        ))?;
    if scheme != "http" && scheme != "https" {
        bail!(
            "endpoint rejected: scheme {:?} not allowed (expected http or https)",
            scheme
        );
    }
    if scheme == "http" && std::env::var("KEI_STORE_S3_ALLOW_INSECURE").is_err() {
        bail!(
            "endpoint rejected: plain http is not permitted \
             (set KEI_STORE_S3_ALLOW_INSECURE=1 for MinIO local dev)"
        );
    }
    let host = extract_host(rest);
    if is_internal_host(host) && std::env::var("KEI_STORE_S3_ALLOW_INTERNAL").is_err() {
        bail!(
            "endpoint rejected: host {:?} is loopback / link-local / metadata \
             (set KEI_STORE_S3_ALLOW_INTERNAL=1 to allow local MinIO or test stubs)",
            host
        );
    }
    Ok(())
}

fn extract_host(rest: &str) -> &str {
    let host_port = rest.split('/').next().unwrap_or("");
    host_port
        .rsplit_once(':')
        .map(|(h, _)| h)
        .unwrap_or(host_port)
        .trim_start_matches('[')
        .trim_end_matches(']')
}

/// Is this host a loopback, link-local, or known metadata endpoint?
fn is_internal_host(host: &str) -> bool {
    if host == "localhost" || host == "127.0.0.1" || host == "::1" || host == "0.0.0.0" {
        return true;
    }
    if host.starts_with("127.") || host.starts_with("169.254.") {
        return true;
    }
    // IPv6 link-local fe80::/10 — matches fe80… through febf… prefixes.
    if host.len() >= 3 && &host[..2] == "fe" {
        let third = host.as_bytes()[2];
        if (b'8'..=b'b').contains(&third) {
            return true;
        }
    }
    matches!(
        host,
        "metadata.google.internal" | "metadata.azure.com" | "metadata"
    )
}

/// Resolve `access_key_env` + `secret_key_env` into a `Credentials` object
/// if both are set. Returns `Ok(None)` if neither is set. Errors if one is
/// set without the other, or if the resolved env var is empty.
pub(super) fn resolve_explicit_creds(cfg: &S3Cfg) -> Result<Option<Credentials>> {
    match (cfg.access_key_env.as_ref(), cfg.secret_key_env.as_ref()) {
        (None, None) => Ok(None),
        (Some(_), None) | (None, Some(_)) => bail!(
            "config invalid: access_key_env and secret_key_env must both be set or both absent"
        ),
        (Some(a_var), Some(s_var)) => {
            let access = std::env::var(a_var)
                .map_err(|_| anyhow!("env var {:?} (access_key_env) is not set", a_var))?;
            let secret = std::env::var(s_var)
                .map_err(|_| anyhow!("env var {:?} (secret_key_env) is not set", s_var))?;
            if access.is_empty() || secret.is_empty() {
                bail!(
                    "env vars {:?} / {:?} resolved to empty string — refusing to sign with empty creds",
                    a_var,
                    s_var
                );
            }
            Ok(Some(Credentials::new(
                access,
                secret,
                None,
                None,
                "keisei-s3-cfg",
            )))
        }
    }
}

/// Build the aws-sdk-s3 client with optional endpoint + region overrides.
pub async fn build_client(cfg: &S3Cfg) -> Result<Client> {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
    if let Some(region) = cfg.region.clone() {
        loader = loader.region(aws_config::Region::new(region));
    }

    let endpoint = effective_endpoint(cfg);
    let explicit_creds = resolve_explicit_creds(cfg)?;

    // If a custom endpoint is set (non-AWS path) we REQUIRE explicit creds.
    // Falling back to the AWS default chain against a non-AWS endpoint can
    // leak IMDS-sourced creds to an attacker-controlled server.
    if endpoint.is_some() && explicit_creds.is_none() {
        bail!(
            "custom endpoint {:?} set without access_key_env + secret_key_env — \
             refusing to use AWS default credential chain (IMDS leak risk). \
             Configure access_key_env and secret_key_env in [s3] TOML, or \
             unset KEI_STORE_S3_ENDPOINT to use real AWS",
            endpoint.as_deref().unwrap_or("<none>")
        );
    }

    // If the caller supplied explicit creds, overlay them BEFORE loading the
    // default chain so we never hit IMDS at all.
    if let Some(creds) = explicit_creds.clone() {
        loader = loader.credentials_provider(creds);
    }

    let shared = loader.load().await;
    let mut s3_builder = aws_sdk_s3::config::Builder::from(&shared);
    if let Some(endpoint) = endpoint {
        validate_endpoint(&endpoint)?;
        s3_builder = s3_builder.endpoint_url(endpoint).force_path_style(true);
    }
    if let Some(creds) = explicit_creds {
        s3_builder = s3_builder.credentials_provider(creds);
    }
    Ok(Client::from_conf(s3_builder.build()))
}
