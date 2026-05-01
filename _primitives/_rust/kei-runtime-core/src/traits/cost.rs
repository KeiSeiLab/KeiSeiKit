// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBudget {
    pub dna: Dna,
    pub parent_dna: Dna,
    pub scope: CostScope,
    pub hard_kill_microcents: u64,
    pub soft_alert_microcents: u64,
    pub current_microcents: u64,
    pub reset_unix_ms: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CostScope {
    UserMonthly,
    RuntimeDaily,
    TaskBudget,
    VmSpend,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CostVerdict {
    Ok,
    SoftAlert,
    HardKill,
}

#[async_trait::async_trait]
pub trait CostGuard: HasDna + Send + Sync {
    fn guard_name(&self) -> &'static str;

    async fn record_spend(&self, budget: &Dna, microcents: u64) -> Result<CostVerdict>;
    async fn current(&self, budget: &Dna) -> Result<CostBudget>;
    async fn reset(&self, budget: &Dna) -> Result<()>;

    /// Configure a new budget. Returns its DNA.
    async fn install(&self, b: &CostBudget) -> Result<Dna>;
}
