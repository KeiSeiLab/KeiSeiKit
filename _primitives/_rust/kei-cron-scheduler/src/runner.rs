//! Tokio-based job runner.
//!
//! Ticks every `tick_interval` (default 60s — Hermes parity) and fires due
//! jobs via an outbound `mpsc` channel. The actual execution is delegated to
//! the consumer; this crate is metadata-only.
//!
//! Crash safety: `jobs.json` is the source of truth — restart re-reads it.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use tokio::sync::{mpsc, Mutex};

use crate::job::{Job, JobId};
use crate::store::JobStore;

const DEFAULT_TICK_SECS: u64 = 60;

/// Events emitted by the runner.
#[derive(Debug, Clone)]
pub enum RunnerEvent {
    /// Job is due — caller executes the prompt.
    Fire { job: Job },
    /// One tick boundary has elapsed (for debugging / observability).
    Tick {
        at: chrono::DateTime<chrono::Utc>,
        due_count: usize,
    },
}

/// Job runner config.
#[derive(Clone)]
pub struct RunnerConfig {
    pub tick_interval: Duration,
    pub event_buffer: usize,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(DEFAULT_TICK_SECS),
            event_buffer: 64,
        }
    }
}

/// Drives [`JobStore`] forward in time, emitting [`RunnerEvent`]s.
pub struct JobRunner {
    store: Arc<Mutex<JobStore>>,
    config: RunnerConfig,
}

impl JobRunner {
    pub fn new(store: JobStore) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
            config: RunnerConfig::default(),
        }
    }

    pub fn with_config(mut self, config: RunnerConfig) -> Self {
        self.config = config;
        self
    }

    /// Spawn the tick loop. Returns the receiver half of the event channel.
    pub fn start(self: Arc<Self>) -> mpsc::Receiver<RunnerEvent> {
        let (tx, rx) = mpsc::channel(self.config.event_buffer);
        let me = self.clone();
        tokio::spawn(async move {
            me.run_loop(tx).await;
        });
        rx
    }

    async fn run_loop(&self, tx: mpsc::Sender<RunnerEvent>) {
        let mut interval = tokio::time::interval(self.config.tick_interval);
        // Skip the immediate tick fired by tokio::time::interval at t=0.
        interval.tick().await;
        loop {
            interval.tick().await;
            if let Err(e) = self.tick_once(&tx).await {
                eprintln!("[kei-cron-scheduler] tick failed: {e:#}");
            }
        }
    }

    /// Single tick: load jobs, fire due ones, persist updated state.
    pub async fn tick_once(&self, tx: &mpsc::Sender<RunnerEvent>) -> Result<()> {
        let now = Utc::now();
        let due_ids: Vec<JobId> = {
            let store = self.store.lock().await;
            let map = store.load_all()?;
            map.values()
                .filter(|j| j.is_due(now))
                .map(|j| j.id.clone())
                .collect()
        };

        let _ = tx
            .send(RunnerEvent::Tick {
                at: now,
                due_count: due_ids.len(),
            })
            .await;

        for id in due_ids {
            self.fire_one(&id, &tx, now).await?;
        }
        Ok(())
    }

    async fn fire_one(
        &self,
        id: &str,
        tx: &mpsc::Sender<RunnerEvent>,
        now: chrono::DateTime<chrono::Utc>,
    ) -> Result<()> {
        let store = self.store.lock().await;
        let mut snapshot: Option<Job> = None;
        store.modify(|map| {
            if let Some(job) = map.get_mut(id) {
                job.mark_fired(now);
                snapshot = Some(job.clone());
            }
            Ok(())
        })?;
        drop(store);

        if let Some(job) = snapshot {
            let _ = tx.send(RunnerEvent::Fire { job }).await;
        }
        Ok(())
    }
}
