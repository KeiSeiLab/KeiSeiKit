//! Primitive scanner — walks `<kit-root>/_primitives/_rust/*/Cargo.toml`.
//!
//! Constructor Pattern: this cube knows the workspace-crate naming
//! convention only. The body bytes are the raw `Cargo.toml`; the name is
//! `[package].name`; caps are the comma-joined dependency family heuristic
//! (e.g. `regex,sqlite,toml`).

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `_primitives/_rust/<crate>/Cargo.toml` adapter.
pub struct PrimitiveScanner;

impl Scanner for PrimitiveScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        let rust_root = root.join("_primitives").join("_rust");
        if !rust_root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(&rust_root)? {
            let entry = entry?;
            let crate_dir = entry.path();
            if !crate_dir.is_dir() {
                continue;
            }
            if let Some(f) = scan_one_crate(&crate_dir)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn scan_one_crate(crate_dir: &Path) -> Result<Option<Found>> {
    let cargo_toml = crate_dir.join("Cargo.toml");
    if !cargo_toml.is_file() {
        return Ok(None);
    }
    let body = match fs::read(&cargo_toml) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let name = match parse_package_name(&body) {
        Some(n) => n,
        None => return Ok(None),
    };
    let caps = derive_caps_from_toml(&body);
    let path = canonical_str(&cargo_toml);
    Ok(Some(Found {
        block_type: BlockType::Primitive,
        name,
        path,
        body,
        caps,
    }))
}

/// Extract `[package].name` from a Cargo.toml byte slice. Tolerates both
/// `[package]` table form and inline. Returns None on malformed TOML.
fn parse_package_name(body: &[u8]) -> Option<String> {
    let txt = std::str::from_utf8(body).ok()?;
    let value: toml::Value = txt.parse().ok()?;
    value
        .get("package")?
        .get("name")?
        .as_str()
        .map(|s| s.to_string())
}

/// Heuristic capability codes from declared `[dependencies]` keys.
/// Stable, lossy. Empty if no recognised deps.
fn derive_caps_from_toml(body: &[u8]) -> String {
    let txt = match std::str::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return String::new(),
    };
    let value: toml::Value = match txt.parse() {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let deps = match value.get("dependencies").and_then(|d| d.as_table()) {
        Some(t) => t,
        None => return String::new(),
    };
    let mut caps: Vec<&str> = Vec::new();
    for (k, _) in deps {
        if let Some(c) = dep_to_cap(k.as_str()) {
            if !caps.contains(&c) {
                caps.push(c);
            }
        }
    }
    caps.sort();
    caps.join(",")
}

fn dep_to_cap(name: &str) -> Option<&'static str> {
    match name {
        "rusqlite" => Some("sqlite"),
        "regex" => Some("regex"),
        "tokio" | "reqwest" | "axum" => Some("network"),
        "clap" => Some("cli"),
        "serde" | "serde_json" | "toml" | "serde_yaml" => Some("md"),
        "sha2" => Some("hash"),
        "walkdir" => Some("fs"),
        _ => None,
    }
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
