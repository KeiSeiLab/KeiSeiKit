//! Hetzner Cloud adapter. Shells out to `hcloud server …`.
//!
//! JSON shape (hcloud v1.44):
//!   describe → `{ "id": u64, "name": str, "status": str,
//!                 "public_net": { "ipv4": { "ip": str } }, ... }`
//!   list     → `[ { same shape } ]`
//!   create   → `{ "server": { same shape as describe } }`

use crate::backend::{Backend, CreateOpts, ServerInfo};
use crate::exec::{require_cli, require_env, run_json, run_json_strict, run_void};
use anyhow::{anyhow, Result};
use serde_json::Value;

const BIN: &str = "hcloud";
const INSTALL_HINT: &str =
    "brew install hcloud (macOS) | https://github.com/hetznercloud/cli/releases";
const ENV_TOKEN: &str = "HCLOUD_TOKEN";

pub struct HetznerBackend;

impl HetznerBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_ready(&self) -> Result<()> {
        require_cli(BIN, INSTALL_HINT)?;
        require_env(ENV_TOKEN)?;
        Ok(())
    }

    fn describe(&self, name: &str) -> Result<Option<Value>> {
        run_json(BIN, &["server", "describe", name, "-o", "json"])
    }
}

impl Default for HetznerBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for HetznerBackend {
    fn name(&self) -> &'static str {
        "hetzner"
    }

    fn create(&self, name: &str, opts: &CreateOpts) -> Result<ServerInfo> {
        self.ensure_ready()?;
        if let Some(v) = self.describe(name)? {
            return Ok(parse_server(&v));
        }
        let args = build_create_args(name, opts)?;
        let argrefs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let v = run_json_strict(BIN, &argrefs)?
            .ok_or_else(|| anyhow!("hcloud create emitted no JSON"))?;
        let server = v.get("server").cloned().unwrap_or(v);
        Ok(parse_server(&server))
    }

    fn status(&self, name: &str) -> Result<Option<ServerInfo>> {
        self.ensure_ready()?;
        Ok(self.describe(name)?.map(|v| parse_server(&v)))
    }

    fn destroy(&self, name: &str, _force: bool) -> Result<()> {
        self.ensure_ready()?;
        if self.describe(name)?.is_none() {
            return Ok(()); // idempotent absent
        }
        run_void(BIN, &["server", "delete", name])
    }

    fn list(&self) -> Result<Vec<ServerInfo>> {
        self.ensure_ready()?;
        let v = run_json_strict(BIN, &["server", "list", "-o", "json"])?
            .ok_or_else(|| anyhow!("hcloud list emitted no JSON"))?;
        let arr = v
            .as_array()
            .ok_or_else(|| anyhow!("hcloud list: expected array, got {v:?}"))?;
        Ok(arr.iter().map(parse_server).collect())
    }
}

fn build_create_args(name: &str, opts: &CreateOpts) -> Result<Vec<String>> {
    let mut args: Vec<String> = vec![
        "server".into(),
        "create".into(),
        "--name".into(),
        name.into(),
        "--type".into(),
        opts.server_type.clone().unwrap_or_else(|| "cx22".into()),
        "--image".into(),
        opts.image.clone().unwrap_or_else(|| "debian-12".into()),
        "--location".into(),
        opts.location.clone().unwrap_or_else(|| "fsn1".into()),
        "--label".into(),
        "project=kei".into(),
    ];
    if let Some(k) = &opts.ssh_key {
        args.extend(["--ssh-key".into(), k.clone()]);
    }
    if let Some(f) = &opts.firewall {
        args.extend(["--firewall".into(), f.clone()]);
    }
    if let Some(p) = &opts.user_data_path {
        if !p.is_file() {
            return Err(anyhow!("user-data not readable: {}", p.display()));
        }
        args.extend(["--user-data-from-file".into(), p.display().to_string()]);
    }
    args.extend(["-o".into(), "json".into()]);
    Ok(args)
}

fn parse_server(v: &Value) -> ServerInfo {
    let id = v
        .get("id")
        .map(|x| x.to_string().trim_matches('"').to_string())
        .unwrap_or_default();
    let name = v
        .get("name")
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string();
    let status = v
        .get("status")
        .and_then(|x| x.as_str())
        .unwrap_or("unknown")
        .to_string();
    let ipv4 = v
        .pointer("/public_net/ipv4/ip")
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty() && *s != "-")
        .map(|s| s.to_string());
    ServerInfo {
        id,
        name,
        ipv4,
        status,
        backend_specific: v.clone(),
    }
}
