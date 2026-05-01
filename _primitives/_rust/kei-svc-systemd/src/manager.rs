// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::error::{Error, Result};
use crate::templates::{render_service, render_timer};
use kei_runtime_core::traits::service::{ServiceManager, ServiceStatus, ServiceUnit};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::PathBuf;
use std::process::Command;

pub struct SystemdManager {
    dna: Dna,
    parent: Option<Dna>,
    /// Where unit files are written. Defaults to `/etc/systemd/system/`
    /// for system-wide; pass `/run/...` or `~/.config/systemd/user/` to
    /// taste. Tests use a tempdir.
    units_dir: PathBuf,
    /// `systemctl` binary path; default just `"systemctl"`.
    systemctl: String,
    /// Whether to actually invoke systemctl or just write files (for tests).
    invoke_systemctl: bool,
}

impl SystemdManager {
    pub fn system() -> Result<Self> {
        Self::with("/etc/systemd/system", "systemctl", true, None)
    }

    pub fn user() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        let dir = format!("{}/.config/systemd/user", home);
        Self::with(&dir, "systemctl --user", true, None)
    }

    pub fn with(units_dir: &str, systemctl: &str, invoke: bool, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "SD"])
            .scope("keiseikit.dev/primitives/kei-svc-systemd")
            .body(units_dir.as_bytes())
            .build()?;
        Ok(Self {
            dna,
            parent,
            units_dir: PathBuf::from(units_dir),
            systemctl: systemctl.to_string(),
            invoke_systemctl: invoke,
        })
    }

    fn unit_path(&self, name: &str, suffix: &str) -> PathBuf {
        self.units_dir.join(format!("{name}.{suffix}"))
    }

    fn run_systemctl(&self, args: &[&str]) -> Result<String> {
        if !self.invoke_systemctl {
            return Ok(String::new());
        }
        let parts: Vec<&str> = self.systemctl.split_whitespace().collect();
        let bin = parts[0];
        let pre_args: Vec<&str> = parts[1..].to_vec();
        let mut cmd = Command::new(bin);
        cmd.args(&pre_args).args(args);
        let out = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::SystemctlNotFound
            } else {
                Error::Io(e)
            }
        })?;
        if !out.status.success() {
            return Err(Error::SystemctlFailed {
                cmd: args.join(" "),
                stderr: String::from_utf8_lossy(&out.stderr).into(),
            });
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}

impl HasDna for SystemdManager {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait::async_trait]
impl ServiceManager for SystemdManager {
    fn manager_name(&self) -> &'static str { "systemd" }

    async fn install(&self, unit: &ServiceUnit) -> kei_runtime_core::Result<()> {
        std::fs::create_dir_all(&self.units_dir).map_err(Error::Io)?;
        let svc_path = self.unit_path(&unit.name, "service");
        std::fs::write(&svc_path, render_service(unit)).map_err(Error::Io)?;
        if let Some(t) = &unit.timer_spec {
            let timer_path = self.unit_path(&unit.name, "timer");
            std::fs::write(&timer_path, render_timer(&unit.name, t)).map_err(Error::Io)?;
        }
        let _ = self.run_systemctl(&["daemon-reload"]);
        Ok(())
    }

    async fn uninstall(&self, name: &str) -> kei_runtime_core::Result<()> {
        let _ = self.run_systemctl(&["stop", name]);
        let _ = self.run_systemctl(&["disable", name]);
        let _ = std::fs::remove_file(self.unit_path(name, "service"));
        let _ = std::fs::remove_file(self.unit_path(name, "timer"));
        let _ = self.run_systemctl(&["daemon-reload"]);
        Ok(())
    }

    async fn start(&self, name: &str) -> kei_runtime_core::Result<()> {
        self.run_systemctl(&["start", name]).map_err(Error::from)?;
        Ok(())
    }

    async fn stop(&self, name: &str) -> kei_runtime_core::Result<()> {
        self.run_systemctl(&["stop", name]).map_err(Error::from)?;
        Ok(())
    }

    async fn status(&self, name: &str) -> kei_runtime_core::Result<ServiceStatus> {
        if !self.unit_path(name, "service").exists() {
            return Ok(ServiceStatus::NotInstalled);
        }
        if !self.invoke_systemctl {
            return Ok(ServiceStatus::Stopped);
        }
        match self.run_systemctl(&["is-active", name]) {
            Ok(out) if out.trim() == "active" => Ok(ServiceStatus::Running),
            Ok(_) => Ok(ServiceStatus::Stopped),
            Err(_) => Ok(ServiceStatus::Failed),
        }
    }

    async fn enable_at_boot(&self, name: &str) -> kei_runtime_core::Result<()> {
        self.run_systemctl(&["enable", name]).map_err(Error::from)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_runtime_core::traits::service::{RestartPolicy, TimerSpec};

    fn unit_for_test() -> ServiceUnit {
        ServiceUnit {
            name: "kei-runtime".into(),
            exec_path: "/opt/k/bin/kei-runtime".into(),
            args: vec!["run".into()],
            env: vec![],
            working_dir: "/opt/k".into(),
            user: Some("keisei".into()),
            restart_policy: RestartPolicy::OnFailure,
            timer_spec: Some(TimerSpec { on_calendar: "*-*-* 03:07".into(), randomized_delay_sec: 60 }),
        }
    }

    #[tokio::test]
    async fn install_writes_service_and_timer() {
        let dir = tempfile::tempdir().unwrap();
        let m = SystemdManager::with(dir.path().to_str().unwrap(), "systemctl", false, None).unwrap();
        m.install(&unit_for_test()).await.unwrap();
        assert!(dir.path().join("kei-runtime.service").exists());
        assert!(dir.path().join("kei-runtime.timer").exists());
    }

    #[tokio::test]
    async fn status_not_installed() {
        let dir = tempfile::tempdir().unwrap();
        let m = SystemdManager::with(dir.path().to_str().unwrap(), "systemctl", false, None).unwrap();
        let s = m.status("nonexistent").await.unwrap();
        assert_eq!(s, ServiceStatus::NotInstalled);
    }

    #[tokio::test]
    async fn uninstall_removes_files() {
        let dir = tempfile::tempdir().unwrap();
        let m = SystemdManager::with(dir.path().to_str().unwrap(), "systemctl", false, None).unwrap();
        m.install(&unit_for_test()).await.unwrap();
        m.uninstall("kei-runtime").await.unwrap();
        assert!(!dir.path().join("kei-runtime.service").exists());
    }

    #[test]
    fn dna_has_sd_cap() {
        let m = SystemdManager::with("/tmp", "systemctl", false, None).unwrap();
        assert!(m.dna().caps().contains("SD"));
        assert_eq!(m.manager_name(), "systemd");
    }
}
