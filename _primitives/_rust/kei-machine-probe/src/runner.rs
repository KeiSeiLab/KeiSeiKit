//! Runner trait — the seam every detector goes through.
//!
//! Constructor Pattern: ALL `std::process::Command::new` lives here. Every
//! detector (arch / memory / gpu / os / tooling) accepts a `&dyn Runner`
//! so unit tests can substitute a fixture-backed mock without touching the
//! host system.
//!
//! Mock layout: each command becomes a fixture file
//! `<sanitized-cmd>.stdout`. Sanitization replaces every byte outside
//! `[A-Za-z0-9._-]` with `_`. Example:
//!   `sysctl -n hw.model` → `sysctl_-n_hw.model.stdout`
//!   `which ollama`      → `which_ollama.stdout`

use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// One run = one (cmd, args) → stdout. Failures map to `Err` (caller
/// decides whether the failure is fatal or means "not present").
pub trait Runner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<String>;
}

/// Default impl — shells out to the real host.
pub struct SystemRunner;

impl Runner for SystemRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<String> {
        let out = Command::new(cmd)
            .args(args)
            .output()
            .with_context(|| format!("spawn {cmd}"))?;
        if !out.status.success() {
            return Err(anyhow!(
                "{cmd} exited non-zero: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}

/// Sanitize a `(cmd, args)` pair into a fixture filename stem.
/// Bytes outside `[A-Za-z0-9._-]` collapse to `_`.
pub fn fixture_stem(cmd: &str, args: &[&str]) -> String {
    let mut joined = cmd.to_string();
    for a in args {
        joined.push(' ');
        joined.push_str(a);
    }
    joined
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' { c } else { '_' })
        .collect()
}

/// Test / CI runner that reads stdout from `<dir>/<stem>.stdout` files.
/// Missing fixture file = "command not found" error (lets detectors test
/// the absent-tooling path naturally).
pub struct MockRunner {
    pub dir: PathBuf,
    /// Optional in-memory overrides keyed by sanitized stem (no extension).
    pub overrides: HashMap<String, Result<String, String>>,
}

impl MockRunner {
    pub fn from_dir(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
            overrides: HashMap::new(),
        }
    }

    pub fn with_ok(mut self, stem: &str, body: &str) -> Self {
        self.overrides
            .insert(stem.to_string(), Ok(body.to_string()));
        self
    }

    pub fn with_err(mut self, stem: &str, msg: &str) -> Self {
        self.overrides
            .insert(stem.to_string(), Err(msg.to_string()));
        self
    }
}

impl Runner for MockRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> Result<String> {
        let stem = fixture_stem(cmd, args);
        if let Some(slot) = self.overrides.get(&stem) {
            return match slot {
                Ok(s) => Ok(s.clone()),
                Err(e) => Err(anyhow!("{e}")),
            };
        }
        let path = self.dir.join(format!("{stem}.stdout"));
        if !path.exists() {
            return Err(anyhow!("mock fixture missing: {}", path.display()));
        }
        std::fs::read_to_string(&path)
            .with_context(|| format!("read fixture {}", path.display()))
    }
}
