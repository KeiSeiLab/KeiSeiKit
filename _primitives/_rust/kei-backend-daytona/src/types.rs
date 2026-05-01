//! DTOs for the Daytona REST API.
//!
//! These mirror the subset of the Daytona Python SDK we depend on
//! (resume-or-create, exec, file sync). Fields marked `#[serde(default)]`
//! tolerate unknown / older API responses.

use serde::{Deserialize, Serialize};

/// Lifecycle state of a sandbox. Matches Daytona Python SDK `SandboxState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SandboxState {
    /// Sandbox is up and accepting commands.
    Running,
    /// Stopped but filesystem preserved (resume target).
    Stopped,
    /// Hibernated (filesystem snapshotted, deeper than stopped).
    Hibernated,
    /// Sandbox terminal-failed; not resumable.
    Error,
    /// Pending creation / starting up.
    Pending,
    /// Anything we don't know yet.
    #[serde(other)]
    Unknown,
}

/// CPU / memory / disk envelope. Memory + disk in GiB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub cpu: u32,
    pub memory: u32,
    pub disk: u32,
}

impl Default for Resources {
    fn default() -> Self {
        // Mirrors Hermes default: 1 CPU, 5 GiB RAM, 10 GiB disk.
        Self { cpu: 1, memory: 5, disk: 10 }
    }
}

/// REST API representation of a sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sandbox {
    /// Daytona-assigned unique id.
    pub id: String,
    /// Human-readable name (we use this as our `task_id`-derived handle).
    pub name: String,
    /// Current lifecycle state.
    pub state: SandboxState,
    /// Container image used to create the sandbox.
    #[serde(default)]
    pub image: String,
    /// Resource envelope.
    #[serde(default)]
    pub resources: Resources,
    /// Free-form labels (we store `task_id` here).
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

/// Request body for `POST /sandboxes`. Subset of `CreateSandboxFromImageParams`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSandboxSpec {
    pub image: String,
    pub name: String,
    /// `auto_stop_interval = 0` disables idle hibernation; `30` = 30-min idle.
    pub auto_stop_interval: u32,
    pub resources: Resources,
    pub labels: std::collections::HashMap<String, String>,
}

impl CreateSandboxSpec {
    /// Builder convenience: `image` + `name`, defaults for everything else.
    pub fn new(image: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            name: name.into(),
            auto_stop_interval: 30,
            resources: Resources::default(),
            labels: Default::default(),
        }
    }

    /// Override resources.
    pub fn with_resources(mut self, r: Resources) -> Self {
        self.resources = r;
        self
    }

    /// Add a label (e.g. `"task_id" → task-uuid`).
    pub fn with_label(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.labels.insert(k.into(), v.into());
        self
    }

    /// Disable auto-stop (caller manages stop/start manually).
    pub fn with_persistent(mut self) -> Self {
        self.auto_stop_interval = 0;
        self
    }
}

/// Output from a single `exec` call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecOutput {
    /// Combined stdout+stderr (Daytona's `result` field).
    #[serde(default)]
    pub stdout: String,
    /// Process exit code.
    pub exit_code: i32,
}

/// Internal: Daytona REST exec response shape.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExecResponse {
    #[serde(default)]
    pub result: String,
    pub exit_code: i32,
}

impl From<ExecResponse> for ExecOutput {
    fn from(r: ExecResponse) -> Self {
        Self { stdout: r.result, exit_code: r.exit_code }
    }
}
