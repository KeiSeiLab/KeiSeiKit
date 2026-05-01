//! Persistence round-trip and crash-recovery tests for [`JobStore`].

use std::time::Duration;

use kei_cron_scheduler::job::{Job, Schedule};
use kei_cron_scheduler::store::JobStore;
use tempfile::tempdir;

fn store_in(tmp: &std::path::Path) -> JobStore {
    JobStore::new(tmp.join("jobs.json"))
}

#[test]
fn empty_load_returns_empty_map() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    let map = store.load_all().unwrap();
    assert!(map.is_empty());
}

#[test]
fn upsert_then_load_roundtrip() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    let job = Job::new("abc123", "do thing", Schedule::AfterDuration {
        delta: Duration::from_secs(60),
    });
    store.upsert(job.clone()).unwrap();
    let loaded = store.get("abc123").unwrap().expect("job present");
    assert_eq!(loaded.id, job.id);
    assert_eq!(loaded.prompt, "do thing");
}

#[test]
fn upsert_overwrites_existing() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    let mut job = Job::new("dup", "v1", Schedule::AfterDuration {
        delta: Duration::from_secs(60),
    });
    store.upsert(job.clone()).unwrap();
    job.prompt = "v2".to_string();
    store.upsert(job).unwrap();
    let loaded = store.get("dup").unwrap().unwrap();
    assert_eq!(loaded.prompt, "v2");
}

#[test]
fn remove_drops_job() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    let job = Job::new("gone", "x", Schedule::AfterDuration {
        delta: Duration::from_secs(60),
    });
    store.upsert(job).unwrap();
    store.remove("gone").unwrap();
    assert!(store.get("gone").unwrap().is_none());
}

#[test]
fn remove_missing_errors() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    assert!(store.remove("never-existed").is_err());
}

#[test]
fn restart_preserves_state() {
    let dir = tempdir().unwrap();
    {
        let store = store_in(dir.path());
        let job = Job::new(
            "persist-1",
            "morning brief",
            Schedule::Cron {
                expr: "0 9 * * *".into(),
            },
        );
        store.upsert(job).unwrap();
    }
    // Drop the first store; emulate process restart.
    let store2 = store_in(dir.path());
    let map = store2.load_all().unwrap();
    assert_eq!(map.len(), 1);
    assert!(map.contains_key("persist-1"));
}

#[test]
fn multiple_jobs_round_trip() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    for i in 0..5 {
        let id = format!("j{i:03}");
        let job = Job::new(
            id.clone(),
            format!("prompt {i}"),
            Schedule::Interval {
                every: Duration::from_secs(60 * (i as u64 + 1)),
            },
        );
        store.upsert(job).unwrap();
    }
    let map = store.load_all().unwrap();
    assert_eq!(map.len(), 5);
}

#[test]
fn modify_block_is_atomic() {
    let dir = tempdir().unwrap();
    let store = store_in(dir.path());
    store
        .modify(|map| {
            for i in 0..3 {
                let id = format!("batch-{i}");
                map.insert(
                    id.clone(),
                    Job::new(
                        id,
                        "batch insert",
                        Schedule::AfterDuration {
                            delta: Duration::from_secs(60),
                        },
                    ),
                );
            }
            Ok(())
        })
        .unwrap();
    assert_eq!(store.load_all().unwrap().len(), 3);
}

#[test]
fn job_mark_fired_advances_run_count() {
    let mut job = Job::new(
        "x",
        "y",
        Schedule::Interval {
            every: Duration::from_secs(60),
        },
    );
    let prior_next = job.next_run_at;
    job.mark_fired(chrono::Utc::now());
    assert_eq!(job.run_count, 1);
    assert!(job.last_run_at.is_some());
    // For interval, next_run_at must move forward.
    assert!(job.next_run_at > prior_next);
}
