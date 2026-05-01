// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::HasDna;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnit {
    pub name: String,
    pub exec_path: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_dir: String,
    pub user: Option<String>,
    pub restart_policy: RestartPolicy,
    pub timer_spec: Option<TimerSpec>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSpec {
    pub on_calendar: String,         // systemd OnCalendar / launchd StartCalendarInterval / cron
    pub randomized_delay_sec: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    NotInstalled,
    Stopped,
    Running,
    Failed,
}

#[async_trait::async_trait]
pub trait ServiceManager: HasDna + Send + Sync {
    fn manager_name(&self) -> &'static str;

    async fn install(&self, unit: &ServiceUnit) -> Result<()>;
    async fn uninstall(&self, name: &str) -> Result<()>;
    async fn start(&self, name: &str) -> Result<()>;
    async fn stop(&self, name: &str) -> Result<()>;
    async fn status(&self, name: &str) -> Result<ServiceStatus>;
    async fn enable_at_boot(&self, name: &str) -> Result<()>;
}
