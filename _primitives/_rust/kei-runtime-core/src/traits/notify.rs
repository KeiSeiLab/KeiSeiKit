// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub dna: Dna,
    pub parent_dna: Dna,        // sleep_run or runtime that emitted it
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub severity: NotifySeverity,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotifySeverity {
    Info,
    Success,
    Warn,
    Error,
}

#[async_trait::async_trait]
pub trait NotifyChannel: HasDna + Send + Sync {
    fn channel_name(&self) -> &'static str;

    async fn send(&self, n: &Notification) -> Result<()>;

    /// True if this channel supports batched sends (e.g., email digest).
    fn supports_batching(&self) -> bool;

    /// Filter: should this severity be delivered? Channels can elect to
    /// drop info-tier notifications, etc. Default behaviour: deliver all.
    fn min_severity(&self) -> NotifySeverity {
        NotifySeverity::Info
    }
}
