//! Pure tests for `project_root_of` + `Debouncer`. No fs, no notify, no
//! tokio — these helpers are sync by construction.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use kei_projects_watcher::{project_root_of, Debouncer};

#[test]
fn project_root_of_returns_immediate_child() {
    let root = Path::new("/Users/x/Projects");
    let touched = Path::new("/Users/x/Projects/MyApp/src/main.rs");
    let got = project_root_of(touched, root).expect("path is under root");
    assert_eq!(got, PathBuf::from("/Users/x/Projects/MyApp"));
}

#[test]
fn project_root_of_handles_top_level_file_in_project() {
    let root = Path::new("/Users/x/Projects");
    let touched = Path::new("/Users/x/Projects/MyApp/Cargo.toml");
    let got = project_root_of(touched, root).expect("path is under root");
    assert_eq!(got, PathBuf::from("/Users/x/Projects/MyApp"));
}

#[test]
fn project_root_of_returns_none_when_outside_root() {
    let root = Path::new("/Users/x/Projects");
    assert!(project_root_of(Path::new("/Users/x/Other/file.txt"), root).is_none());
    assert!(project_root_of(Path::new("/Users/x/Projects"), root).is_none());
}

#[test]
fn debouncer_collapses_multiple_events_to_one() {
    let window = Duration::from_secs(2);
    let mut deb = Debouncer::new(window);
    let project = PathBuf::from("/Users/x/Projects/MyApp");
    let t0 = Instant::now();
    deb.push(project.clone(), t0);
    deb.push(project.clone(), t0 + Duration::from_millis(500));
    deb.push(project.clone(), t0 + Duration::from_millis(1000));
    let probe_early = t0 + Duration::from_millis(1500);
    assert!(deb.drain_ready(probe_early).is_empty(), "still inside quiet window");
    assert_eq!(deb.pending_len(), 1, "all 3 pushes coalesced into one project entry");
    let probe_late = t0 + Duration::from_millis(3500);
    let ready = deb.drain_ready(probe_late);
    assert_eq!(ready, vec![project], "exactly one emission for the project");
    assert_eq!(deb.pending_len(), 0);
    assert!(deb.drain_ready(probe_late).is_empty(), "drain consumes the entry");
}

#[test]
fn debouncer_isolates_distinct_projects() {
    let mut deb = Debouncer::new(Duration::from_millis(100));
    let a = PathBuf::from("/r/A");
    let b = PathBuf::from("/r/B");
    let t0 = Instant::now();
    deb.push(a.clone(), t0);
    deb.push(b.clone(), t0 + Duration::from_millis(80));
    let mut ready = deb.drain_ready(t0 + Duration::from_millis(150));
    ready.sort();
    assert_eq!(ready, vec![a.clone()]);
    let ready2 = deb.drain_ready(t0 + Duration::from_millis(200));
    assert_eq!(ready2, vec![b]);
}
