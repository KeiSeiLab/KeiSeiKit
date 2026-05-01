//! Smoke-level integration tests: construction, error paths,
//! create / modify / delete events, debounce behaviour.
//!
//! Rename + flow-control tests live in `tests/watch_flow.rs`.

mod common;

use common::{same_path, wait_for};
use kei_watch::{EventKind, Watcher};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tempfile::tempdir;

#[test]
fn new_and_drop_does_not_panic() {
    let w = Watcher::new().expect("new");
    drop(w);
}

#[test]
fn watch_nonexistent_path_returns_error() {
    let mut w = Watcher::new().expect("new");
    let bogus = Path::new("/definitely/does/not/exist/kei-watch-test-xxx");
    assert!(w.watch(bogus, false).is_err());
}

#[test]
fn create_file_emits_created() {
    let d = tempdir().expect("tempdir");
    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(100));

    let f = d.path().join("new.txt");
    fs::write(&f, b"hello").expect("write");

    let got = wait_for(&w, |e| {
        e.kind == EventKind::Created && same_path(&e.path, &f)
    });
    assert!(got.is_some(), "expected Created for {}", f.display());
}

#[test]
fn modify_file_emits_modified() {
    let d = tempdir().expect("tempdir");
    let f = d.path().join("m.txt");
    fs::write(&f, b"v1").expect("seed");

    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(150));

    let mut fh = fs::OpenOptions::new().append(true).open(&f).unwrap();
    fh.write_all(b"v2").unwrap();
    fh.flush().unwrap();
    drop(fh);

    let got = wait_for(&w, |e| {
        e.kind == EventKind::Modified && same_path(&e.path, &f)
    });
    assert!(got.is_some(), "expected Modified for {}", f.display());
}

#[test]
fn delete_file_emits_deleted() {
    let d = tempdir().expect("tempdir");
    let f = d.path().join("del.txt");
    fs::write(&f, b"doomed").expect("seed");
    // Let the seed-create event flush before the watcher starts.
    sleep(Duration::from_millis(100));

    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(200));
    let _ = w.drain();

    fs::remove_file(&f).expect("remove");

    let got = wait_for(&w, |e| {
        e.kind == EventKind::Deleted && same_path(&e.path, &f)
    });
    assert!(got.is_some(), "expected Deleted for {}", f.display());
}

#[test]
fn rapid_modifies_are_debounced() {
    let d = tempdir().expect("tempdir");
    let f = d.path().join("burst.txt");
    fs::write(&f, b"seed").expect("seed");

    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(150));

    let start = Instant::now();
    for i in 0..5 {
        fs::write(&f, format!("v{i}")).unwrap();
    }
    assert!(start.elapsed() < Duration::from_millis(50));

    sleep(Duration::from_millis(300));
    let drained: Vec<_> = w
        .drain()
        .into_iter()
        .filter(|e| e.kind == EventKind::Modified && same_path(&e.path, &f))
        .collect();
    assert!(
        drained.len() <= 2,
        "expected ≤2 Modified events after debounce, got {}: {:?}",
        drained.len(),
        drained
    );
}
