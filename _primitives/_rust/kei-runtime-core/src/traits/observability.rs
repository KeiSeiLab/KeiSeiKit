// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub dna: Dna,
    pub parent_dna: Option<Dna>,
    pub ts_ms: i64,
    pub level: LogLevel,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub dna: Dna,
    pub parent_dna: Option<Dna>,
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub tags: Vec<(String, String)>,
    pub ts_ms: i64,
}

#[async_trait::async_trait]
pub trait Observability: HasDna + Send + Sync {
    fn sink_name(&self) -> &'static str;

    async fn log(&self, event: &LogEvent) -> Result<()>;
    async fn metric(&self, m: &Metric) -> Result<()>;
    async fn flush(&self) -> Result<()>;

    /// Minimum log level this sink processes. Events below it are dropped.
    fn min_level(&self) -> LogLevel {
        LogLevel::Info
    }

    /// True if this sink supports structured fields (JSON / OTLP) vs
    /// flat strings (syslog).
    fn supports_structured(&self) -> bool;
}
