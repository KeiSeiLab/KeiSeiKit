// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! 12 capability traits. Each extends [`HasDna`]: every impl must
//! produce its DNA serial. No anonymous impls.

pub mod auth;
pub mod backup;
pub mod compute;
pub mod cost;
pub mod git;
pub mod llm;
pub mod memory;
pub mod network;
pub mod notify;
pub mod observability;
pub mod scheduler;
pub mod service;

pub use auth::{AuthChallenge, AuthProvider, AuthSession};
pub use backup::{Backup, Snapshot};
pub use compute::{ComputeProvider, VmHandle, VmSpec, VmStatus};
pub use cost::{CostBudget, CostGuard, CostScope, CostVerdict};
pub use git::{CommitMeta, GitAuthKind, GitBackend, GitRemote};
pub use llm::{CompletionOpts, CompletionResponse, LlmBackend, Message};
pub use memory::{MemoryBackend, MemoryItem, MemoryQuery};
pub use network::{NetworkConfig, NetworkMode, PeerStatus};
pub use notify::{Notification, NotifyChannel, NotifySeverity};
pub use observability::{LogEvent, LogLevel, Metric, Observability};
pub use scheduler::{ScheduleKind, ScheduledTask, Scheduler};
pub use service::{RestartPolicy, ServiceManager, ServiceStatus, ServiceUnit, TimerSpec};
