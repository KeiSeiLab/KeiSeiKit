//! Atomic-write test: pipe bytes through `<dest>.tmp` + rename, assert the
//! destination file contains exactly the bytes written.

use kei_frustration_loop::persistence::{
    atomic_swap, atomic_write, read_last_scan_ts, write_last_scan_ts,
};
use std::fs;
use tempfile::TempDir;

#[test]
fn atomic_write_creates_destination_with_exact_bytes() {
    let dir = TempDir::new().unwrap();
    let dest = dir.path().join("output.bin");
    let payload = b"hello world\n\xff\xfeHELLO";
    atomic_write(&dest, payload).unwrap();

    let read_back = fs::read(&dest).unwrap();
    assert_eq!(read_back, payload, "destination contents must match");

    let tmp = {
        let mut s = dest.as_os_str().to_owned();
        s.push(".tmp");
        std::path::PathBuf::from(s)
    };
    assert!(!tmp.exists(), "tmp file must be renamed away, not left behind");
}

#[test]
fn atomic_write_overwrites_existing_destination() {
    let dir = TempDir::new().unwrap();
    let dest = dir.path().join("dst.txt");
    fs::write(&dest, b"OLD").unwrap();
    atomic_write(&dest, b"NEW").unwrap();
    assert_eq!(fs::read(&dest).unwrap(), b"NEW");
}

#[test]
fn atomic_swap_renames_tmp_into_dest() {
    let dir = TempDir::new().unwrap();
    let tmp = dir.path().join("staging.bin");
    let dst = dir.path().join("final.bin");
    let payload = b"swap-me";
    fs::write(&tmp, payload).unwrap();
    atomic_swap(&tmp, &dst).unwrap();
    assert!(!tmp.exists(), "tmp must be gone after swap");
    assert_eq!(fs::read(&dst).unwrap(), payload);
}

#[test]
fn last_scan_ts_round_trip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("alice.last-scan.ts");
    assert_eq!(
        read_last_scan_ts(&path),
        0,
        "missing file should read as 0"
    );
    write_last_scan_ts(&path, 1_700_000_000).unwrap();
    assert_eq!(read_last_scan_ts(&path), 1_700_000_000);
    write_last_scan_ts(&path, 1_700_000_999).unwrap();
    assert_eq!(read_last_scan_ts(&path), 1_700_000_999);
}

#[test]
fn last_scan_ts_unparseable_reads_as_zero() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.ts");
    fs::write(&path, "not a number").unwrap();
    assert_eq!(read_last_scan_ts(&path), 0);
}
