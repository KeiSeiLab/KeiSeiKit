//! JSON-on-disk job store.
//!
//! Hermes equivalent: `cron/jobs.py` (load/save). Uses `fs2` advisory file
//! locking so parallel processes can safely share the same `jobs.json`.
//! Writes are atomic via temp+rename.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use fs2::FileExt;
use thiserror::Error;

use crate::job::{Job, JobId};

/// All store errors.
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing parent directory for {0:?}")]
    MissingParent(PathBuf),
    #[error("job not found: {0}")]
    NotFound(JobId),
}

/// Opens / creates `jobs.json` at the configured path.
///
/// Default path: `~/.keiseikit/scheduler/jobs.json`. Callers can override.
pub struct JobStore {
    path: PathBuf,
}

impl JobStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Convenience helper: `~/.keiseikit/scheduler/jobs.json`.
    pub fn default_path() -> Result<PathBuf, StoreError> {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        Ok(home.join(".keiseikit").join("scheduler").join("jobs.json"))
    }

    /// Read all jobs (consumes a shared lock for the duration of the read).
    pub fn load_all(&self) -> Result<BTreeMap<JobId, Job>, StoreError> {
        if !self.path.exists() {
            return Ok(BTreeMap::new());
        }
        let mut file = File::open(&self.path)?;
        file.lock_shared()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        FileExt::unlock(&file)?;
        if buf.trim().is_empty() {
            return Ok(BTreeMap::new());
        }
        let map: BTreeMap<JobId, Job> = serde_json::from_str(&buf)?;
        Ok(map)
    }

    /// Atomic read-modify-write under exclusive lock.
    pub fn modify<F>(&self, mutator: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut BTreeMap<JobId, Job>) -> Result<(), StoreError>,
    {
        ensure_parent_dir(&self.path)?;
        let lock_path = self.path.with_extension("lock");
        let lock_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)?;
        lock_file.lock_exclusive()?;

        let result = (|| {
            let mut map = self.load_all()?;
            mutator(&mut map)?;
            self.write_atomic(&map)?;
            Ok(())
        })();

        FileExt::unlock(&lock_file)?;
        result
    }

    /// Insert or overwrite one job.
    pub fn upsert(&self, job: Job) -> Result<(), StoreError> {
        self.modify(|map| {
            map.insert(job.id.clone(), job);
            Ok(())
        })
    }

    /// Remove a job by ID. Errors if missing.
    pub fn remove(&self, id: &str) -> Result<(), StoreError> {
        self.modify(|map| match map.remove(id) {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(id.into())),
        })
    }

    /// Single-job lookup (no lock — best-effort eventual consistency).
    pub fn get(&self, id: &str) -> Result<Option<Job>, StoreError> {
        Ok(self.load_all()?.remove(id))
    }

    fn write_atomic(&self, map: &BTreeMap<JobId, Job>) -> Result<(), StoreError> {
        ensure_parent_dir(&self.path)?;
        let parent = self
            .path
            .parent()
            .ok_or_else(|| StoreError::MissingParent(self.path.clone()))?;
        let tmp = parent.join(format!(
            ".jobs-{}.tmp",
            std::process::id()
        ));
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp)?;
        let bytes = serde_json::to_vec_pretty(map)?;
        f.write_all(&bytes)?;
        f.sync_all()?;
        drop(f);
        std::fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn ensure_parent_dir(path: &Path) -> Result<(), StoreError> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    Ok(())
}
