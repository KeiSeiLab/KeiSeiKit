//! Async fsevents watcher.
//!
//! Wraps [`notify::RecommendedWatcher`] (FSEvents on macOS) and exposes a
//! tokio `mpsc::Receiver<PathBuf>` of debounced project roots. Each
//! emission means: "something inside this project was touched at least
//! `debounce` ago and has been quiet since — re-index it now".
//!
//! Filters: only Modify/Create/Remove kinds; only paths strictly under
//! the watched root; events received in the first 1 s after watcher
//! start are dropped (FSEvents replays startup events on macOS).

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::event::EventKind;
use notify::{RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::debounce::{project_root_of, Debouncer};

/// Tokio mpsc channel capacity for raw `notify` events. 1024 is generous
/// for a ~50-project tree; bursts above that drop oldest events.
const RAW_CAPACITY: usize = 1024;

/// Async filesystem watcher anchored at a single root directory.
pub struct Watcher {
    _inner: RecommendedWatcher,
    root: PathBuf,
    raw_rx: Option<mpsc::Receiver<notify::Event>>,
    debounce: Duration,
}

impl Watcher {
    /// Create a recursive fsevents watcher on `root`. `debounce` is the
    /// quiet window per project before [`Self::events`] emits.
    pub fn new(root: PathBuf, debounce: Duration) -> Result<Self> {
        let (tx, rx) = mpsc::channel::<notify::Event>(RAW_CAPACITY);
        let mut inner = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(ev) = res {
                let _ = tx.blocking_send(ev);
            }
        })
        .context("construct notify::RecommendedWatcher")?;
        inner
            .watch(&root, RecursiveMode::Recursive)
            .with_context(|| format!("watch root {:?}", root))?;
        Ok(Self { _inner: inner, root, raw_rx: Some(rx), debounce })
    }

    /// Take the receiver of debounced project paths. Single-use; later
    /// calls return an immediately-empty channel.
    pub fn events(&mut self) -> mpsc::Receiver<PathBuf> {
        let (tx, rx) = mpsc::channel::<PathBuf>(RAW_CAPACITY);
        let Some(raw_rx) = self.raw_rx.take() else { return rx };
        let root = self.root.clone();
        let window = self.debounce;
        tokio::spawn(async move { run_loop(raw_rx, tx, root, window).await });
        rx
    }
}

/// Does this `notify` event kind trigger a re-index?
fn is_relevant(kind: &EventKind) -> bool {
    matches!(kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_))
}

/// Push one raw event's project roots into the debouncer at `now`.
fn ingest(ev: notify::Event, root: &PathBuf, deb: &mut Debouncer, now: Instant) {
    if !is_relevant(&ev.kind) {
        return;
    }
    for p in ev.paths {
        if let Some(project) = project_root_of(&p, root) {
            deb.push(project, now);
        }
    }
}

/// Pump raw events through the debouncer; emit ready projects every 250 ms.
async fn run_loop(
    mut raw_rx: mpsc::Receiver<notify::Event>,
    tx: mpsc::Sender<PathBuf>,
    root: PathBuf,
    window: Duration,
) {
    let started = Instant::now();
    let mut deb = Debouncer::new(window);
    let mut tick = interval(Duration::from_millis(250));
    loop {
        tokio::select! {
            maybe_ev = raw_rx.recv() => {
                let Some(ev) = maybe_ev else { break };
                let now = Instant::now();
                if now.duration_since(started) < Duration::from_secs(1) { continue; }
                ingest(ev, &root, &mut deb, now);
            }
            _ = tick.tick() => {
                for project in deb.drain_ready(Instant::now()) {
                    if tx.send(project).await.is_err() { return; }
                }
            }
        }
    }
}
