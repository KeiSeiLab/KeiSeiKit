//! Vultr adapter. Shells out to `vultr-cli instance …` (v3 CLI).
//!
//! JSON shape (vultr-cli v3):
//!   instance list → `{ "instances": [ { "id": str, "label": str,
//!                      "main_ip": str, "status": str, "region": str,
//!                      "plan": str, "power_status": str, ... } ] }`
//!   instance get <id> → `{ "instance": {...} }` (id required, not label)
//!   instance create → `{ "instance": {...} }`
//!   os list → `{ "os": [ { "id": int, "name": str, ... } ] }`

use crate::b64;
use crate::backend::{Backend, CreateOpts, ServerInfo};
use crate::exec::{require_cli, require_env, run_json_strict, run_void};
use anyhow::{anyhow, Context, Result};
use serde_json::Value;

const BIN: &str = "vultr-cli";
const INSTALL_HINT: &str =
    "brew install vultr/vultr-cli/vultr-cli | https://github.com/vultr/vultr-cli";
const ENV_TOKEN: &str = "VULTR_API_KEY";

pub struct VultrBackend;

impl VultrBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_ready(&self) -> Result<()> {
        require_cli(BIN, INSTALL_HINT)?;
        require_env(ENV_TOKEN)?;
        Ok(())
    }

    fn list_raw(&self) -> Result<Vec<Value>> {
        let v = run_json_strict(BIN, &["instance", "list", "-o", "json"])?
            .ok_or_else(|| anyhow!("vultr-cli list emitted no JSON"))?;
        let arr = v
            .get("instances")
            .and_then(|x| x.as_array())
            .cloned()
            .ok_or_else(|| anyhow!("vultr-cli list: missing .instances array"))?;
        Ok(arr)
    }

    fn find_by_label(&self, label: &str) -> Result<Option<Value>> {
        for inst in self.list_raw()? {
            if inst.get("label").and_then(|x| x.as_str()) == Some(label) {
                return Ok(Some(inst));
            }
        }
        Ok(None)
    }

    fn resolve_debian_12(&self) -> Result<String> {
        let v = run_json_strict(BIN, &["os", "list", "-o", "json"])?
            .ok_or_else(|| anyhow!("vultr-cli os list emitted no JSON"))?;
        let arr = v
            .get("os")
            .and_then(|x| x.as_array())
            .ok_or_else(|| anyhow!("vultr-cli os list: missing .os array"))?;
        for os in arr {
            let name = os.get("name").and_then(|x| x.as_str()).unwrap_or("");
            if name.to_lowercase().contains("debian 12")
                && name.to_lowercase().contains("x64")
            {
                let id = os.get("id").ok_or_else(|| anyhow!("os.id missing"))?;
                return Ok(id.to_string().trim_matches('"').to_string());
            }
        }
        Err(anyhow!(
            "cannot resolve Debian 12 x64 OS id. Pass --image explicitly."
        ))
    }
}

impl Default for VultrBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for VultrBackend {
    fn name(&self) -> &'static str {
        "vultr"
    }

    fn create(&self, name: &str, opts: &CreateOpts) -> Result<ServerInfo> {
        self.ensure_ready()?;
        if let Some(v) = self.find_by_label(name)? {
            return Ok(parse_server(&v));
        }
        let os_id = match &opts.image {
            Some(s) => s.clone(),
            None => self.resolve_debian_12()?,
        };
        let args = build_create_args(name, opts, &os_id)?;
        let argrefs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let v = run_json_strict(BIN, &argrefs)?
            .ok_or_else(|| anyhow!("vultr-cli create emitted no JSON"))?;
        let inst = v.get("instance").cloned().unwrap_or(v);
        Ok(parse_server(&inst))
    }

    fn status(&self, name: &str) -> Result<Option<ServerInfo>> {
        self.ensure_ready()?;
        Ok(self.find_by_label(name)?.as_ref().map(parse_server))
    }

    fn destroy(&self, name: &str, _force: bool) -> Result<()> {
        self.ensure_ready()?;
        let Some(inst) = self.find_by_label(name)? else {
            return Ok(()); // idempotent absent
        };
        let id = inst
            .get("id")
            .and_then(|x| x.as_str())
            .context("instance.id missing")?;
        run_void(BIN, &["instance", "delete", id])
    }

    fn list(&self) -> Result<Vec<ServerInfo>> {
        self.ensure_ready()?;
        Ok(self.list_raw()?.iter().map(parse_server).collect())
    }
}

fn build_create_args(label: &str, opts: &CreateOpts, os_id: &str) -> Result<Vec<String>> {
    let mut args: Vec<String> = vec![
        "instance".into(),
        "create".into(),
        "--region".into(),
        opts.location.clone().unwrap_or_else(|| "ams".into()),
        "--plan".into(),
        opts.server_type.clone().unwrap_or_else(|| "vc2-1c-1gb".into()),
        "--os".into(),
        os_id.into(),
        "--label".into(),
        label.into(),
        "--tags".into(),
        "project=kei".into(),
    ];
    if let Some(k) = &opts.ssh_key {
        args.extend(["--ssh-keys".into(), k.clone()]);
    }
    if let Some(f) = &opts.firewall {
        args.extend(["--firewall-group-id".into(), f.clone()]);
    }
    if let Some(p) = &opts.user_data_path {
        if !p.is_file() {
            return Err(anyhow!("user-data not readable: {}", p.display()));
        }
        let raw = std::fs::read(p)?;
        args.extend(["--userdata".into(), b64::encode(&raw)]);
    }
    args.extend(["-o".into(), "json".into()]);
    Ok(args)
}

fn parse_server(v: &Value) -> ServerInfo {
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string();
    let name = v
        .get("label")
        .and_then(|x| x.as_str())
        .unwrap_or_default()
        .to_string();
    let status = v
        .get("status")
        .and_then(|x| x.as_str())
        .unwrap_or("unknown")
        .to_string();
    let ipv4 = v
        .get("main_ip")
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty() && *s != "0.0.0.0")
        .map(|s| s.to_string());
    ServerInfo {
        id,
        name,
        ipv4,
        status,
        backend_specific: v.clone(),
    }
}

