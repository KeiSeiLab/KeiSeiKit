// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub dna: Dna,
    pub parent_dna: Dna,         // runtime that scheduled it
    pub kind: ScheduleKind,
    pub payload_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleKind {
    Cron { expression: String, jitter_minutes: u32 },
    Once { at_unix_ms: i64 },
    Webhook { listen_addr: String },
    Event { sources: Vec<String> },
    External { provider: String, opaque: String },
    Manual,
}

#[async_trait::async_trait]
pub trait Scheduler: HasDna + Send + Sync {
    fn scheduler_name(&self) -> &'static str;

    async fn register(&self, task: &ScheduledTask) -> Result<()>;
    async fn cancel(&self, dna: &Dna) -> Result<()>;
    async fn list(&self) -> Result<Vec<ScheduledTask>>;

    /// Block until the next firing of any registered task. Returns the
    /// task that fired. Implementations may use this for `manual` mode
    /// or for testing.
    async fn next_fire(&self) -> Result<ScheduledTask>;
}
