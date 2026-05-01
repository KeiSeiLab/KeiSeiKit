//! Flow-control integration tests: rename semantics, `drain` behaviour,
//! `next_event` timeout, `unwatch`.

mod common;

use common::{same_path, wait_for};
use kei_watch::{EventKind, Watcher};
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tempfile::tempdir;

#[test]
fn rename_file_emits_renamed() {
    // Platform-flexible: macOS fsevents emits Modify(Name(Both)) with
    // both paths; Linux inotify emits Modify(Name(From)) +
    // Modify(Name(To)) as two events. The test accepts any Renamed
    // referencing either endpoint.
    let d = tempdir().expect("tempdir");
    let from = d.path().join("src.txt");
    let to = d.path().join("dst.txt");
    fs::write(&from, b"x").expect("seed");

    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(150));

    fs::rename(&from, &to).expect("rename");

    let got = wait_for(&w, |e| {
        e.kind == EventKind::Renamed
            && (same_path(&e.path, &from)
                || same_path(&e.path, &to)
                || e.from_path
                    .as_ref()
                    .is_some_and(|p| same_path(p, &from)))
    });
    assert!(got.is_some(), "expected any Renamed event referencing src/dst");
}

#[test]
fn drain_is_non_blocking() {
    let d = tempdir().expect("tempdir");
    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    let start = Instant::now();
    let out = w.drain();
    assert!(start.elapsed() < Duration::from_millis(100));
    assert!(out.is_empty());
}

#[test]
fn next_event_times_out_on_idle() {
    let d = tempdir().expect("tempdir");
    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(100));
    let _ = w.drain();
    let start = Instant::now();
    let ev = w.next_event(Duration::from_millis(200));
    let elapsed = start.elapsed();
    assert!(ev.is_none(), "expected None on idle, got {ev:?}");
    assert!(
        elapsed >= Duration::from_millis(150),
        "next_event returned too fast: {elapsed:?}"
    );
}

#[test]
fn unwatch_stops_events() {
    let d = tempdir().expect("tempdir");
    let mut w = Watcher::new().expect("new");
    w.watch(d.path(), true).expect("watch");
    sleep(Duration::from_millis(100));
    w.unwatch(d.path()).expect("unwatch");
    sleep(Duration::from_millis(100));
    let _ = w.drain();

    fs::write(d.path().join("ghost.txt"), b"boo").unwrap();
    sleep(Duration::from_millis(400));

    let after: Vec<_> = w
        .drain()
        .into_iter()
        .filter(|e| same_path(&e.path, &d.path().join("ghost.txt")))
        .collect();
    assert!(
        after.is_empty(),
        "expected zero events after unwatch, got {after:?}"
    );
}
