//! High-level resume-or-create logic on top of `DaytonaClient`.
//!
//! Defines a minimal local `Backend` trait. When a shared backend trait
//! crate lands, this trait can be replaced by a re-export — the method
//! shapes are intentionally generic.

use crate::backend_sync::{pull_if_configured, push_if_configured};
use crate::client::DaytonaClient;
use crate::error::{DaytonaError, Result};
use crate::types::{
    CreateSandboxSpec, ExecOutput, Resources, Sandbox, SandboxState,
};
use async_trait::async_trait;
use std::path::PathBuf;

/// Opaque handle returned by `Backend::acquire`.
///
/// Carries the sandbox name + image so `release` does not need to re-query.
#[derive(Debug, Clone)]
pub struct SandboxHandle {
    pub name: String,
    pub image: String,
}

/// Optional file-sync configuration plumbed via `DaytonaBackend::with_sync`.
///
/// When configured, `acquire` pushes `local_root` into the sandbox after the
/// sandbox is up, and `release(persist=true)` pulls a sentinel marker back
/// from `remote_root` before stopping. See `backend_sync.rs` for helpers.
#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub local_root: PathBuf,
    pub remote_root: String,
}

/// Minimal sandbox-lifecycle trait, modelled after Hermes' BaseEnvironment
/// without the synchronous Python idioms.
#[async_trait]
pub trait Backend: Send + Sync {
    /// Resume an existing sandbox keyed by `task_id`, or create a new one.
    async fn acquire(&self, task_id: &str) -> Result<SandboxHandle>;

    /// `persist=true` → stop (preserve filesystem); `false` → delete.
    async fn release(&self, handle: SandboxHandle, persist: bool) -> Result<()>;

    /// Execute a single bash command inside the sandbox.
    async fn exec(&self, handle: &SandboxHandle, cmd: &str) -> Result<ExecOutput>;
}

/// Concrete Daytona backend wired to a `DaytonaClient`.
#[derive(Debug, Clone)]
pub struct DaytonaBackend {
    client: DaytonaClient,
    image: String,
    resources: Resources,
    sync: Option<SyncConfig>,
}

impl DaytonaBackend {
    /// Build with an explicit client + default resources.
    pub fn new(client: DaytonaClient, image: impl Into<String>) -> Self {
        Self {
            client,
            image: image.into(),
            resources: Resources::default(),
            sync: None,
        }
    }

    /// Override default resources.
    pub fn with_resources(mut self, r: Resources) -> Self {
        self.resources = r;
        self
    }

    /// Configure bidirectional file-sync with the sandbox.
    ///
    /// When set, `acquire` pushes `local_root → remote_root` after the
    /// sandbox reaches Running, and `release(persist=true|false)` pulls a
    /// sentinel marker back before stop/delete (best-effort, errors logged
    /// but not propagated; see `backend_sync.rs`).
    pub fn with_sync(mut self, cfg: SyncConfig) -> Self {
        self.sync = Some(cfg);
        self
    }

    /// Borrow the underlying low-level client (file uploads, etc.).
    pub fn client(&self) -> &DaytonaClient {
        &self.client
    }

    /// Sandbox name from a task id. Mirrors Hermes `f"hermes-{task_id}"`.
    fn sandbox_name(task_id: &str) -> String {
        format!("kei-{task_id}")
    }

    /// Build a CreateSandboxSpec for first-time creation.
    fn build_spec(&self, task_id: &str) -> CreateSandboxSpec {
        CreateSandboxSpec::new(&self.image, Self::sandbox_name(task_id))
            .with_resources(self.resources.clone())
            .with_label("kei_task_id", task_id)
            .with_persistent()
    }

    /// Resume-or-create state machine. Returns the live sandbox.
    async fn resume_or_create(&self, task_id: &str) -> Result<Sandbox> {
        let name = Self::sandbox_name(task_id);
        if let Some(sb) = self.client.get_sandbox(&name).await? {
            self.resume_existing(sb).await
        } else {
            let spec = self.build_spec(task_id);
            self.client.create_sandbox(&spec).await
        }
    }

    /// Bring an existing sandbox to Running, regardless of prior state.
    async fn resume_existing(&self, sb: Sandbox) -> Result<Sandbox> {
        match sb.state {
            SandboxState::Running => Ok(sb),
            SandboxState::Stopped | SandboxState::Hibernated | SandboxState::Pending => {
                self.client.start_sandbox(&sb.name).await?;
                Ok(sb)
            }
            SandboxState::Error => Err(DaytonaError::Unknown(format!(
                "sandbox {} is in Error state; not resumable",
                sb.name
            ))),
            SandboxState::Unknown => Err(DaytonaError::Unknown(format!(
                "sandbox {} has unknown state",
                sb.name
            ))),
        }
    }
}

#[async_trait]
impl Backend for DaytonaBackend {
    async fn acquire(&self, task_id: &str) -> Result<SandboxHandle> {
        let sb = self.resume_or_create(task_id).await?;
        let handle = SandboxHandle { name: sb.name, image: self.image.clone() };
        // Push local tree into the sandbox once it is Running. Errors are
        // logged inside the helper but never abort the lifecycle — the
        // sandbox is still usable, and a subsequent acquire can resync.
        push_if_configured(&self.client, &handle, self.sync.as_ref()).await?;
        Ok(handle)
    }

    async fn release(&self, handle: SandboxHandle, persist: bool) -> Result<()> {
        // Pull sentinel marker BEFORE stop/delete: post-stop pulls would
        // race the hibernation transition; pre-delete pulls are the only
        // chance to acknowledge sandbox state.
        pull_if_configured(&self.client, &handle, self.sync.as_ref()).await;
        if persist {
            self.client.stop_sandbox(&handle.name).await
        } else {
            self.client.delete_sandbox(&handle.name).await
        }
    }

    async fn exec(&self, handle: &SandboxHandle, cmd: &str) -> Result<ExecOutput> {
        self.client.exec(&handle.name, cmd).await
    }
}
