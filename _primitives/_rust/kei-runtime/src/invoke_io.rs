//! Bounded-read child-process IO for `invoke.rs`.
//!
//! Wave 44d resource-cap hardening: the previous `wait_with_output()`
//! buffered ALL stdout/stderr before any size check ran — a malicious
//! atom writing 10 GiB of zeros would OOM the runtime before truncation.
//! This module replaces that with size-tracked stream readers that KILL
//! the child the moment a cap is exceeded — the kill is issued from
//! INSIDE the reader thread so an unbounded writer cannot deadlock the
//! parent (post-hoc kill would never happen because the reader would
//! never return on infinite stdout).
//!
//! Constructor Pattern: extracted as a sibling module so `invoke.rs`
//! stays under 200 LOC and per-function under 30 LOC.

use std::io::Read;
use std::process::{Child, ChildStderr, ChildStdout};
use std::sync::{Arc, Mutex};
use std::thread;

/// Hard cap on stdout/stderr each. Mirrors the public `OUTPUT_CAP`
/// constant in `invoke.rs` (16 MiB). Kept as a module-private mirror
/// so this file can be reasoned about in isolation.
const STREAM_CAP: usize = 16 * 1024 * 1024;

/// Per-read chunk size. 8 KiB is the typical pipe buffer granularity
/// on Linux/macOS; smaller would inflate syscall count, larger would
/// risk overshooting the cap by up to one chunk.
const CHUNK: usize = 8 * 1024;

/// Captured child output. `truncated` is true when EITHER stream hit
/// its cap; the caller is expected to surface the truncation in any
/// error message it returns to the user.
pub(crate) struct Captured {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status_code: i32,
    pub truncated: bool,
}

/// Shared kill-handle: the reader thread that trips the cap takes the
/// `Mutex`, calls `kill()`, drops the lock. Wrapped so both readers
/// race-safely — only one wins, the other sees the child already gone.
type KillHandle = Arc<Mutex<Option<Child>>>;

/// Read both streams concurrently with size caps. If either reader
/// trips its cap, that reader kills the child IMMEDIATELY (from inside
/// the reader thread) so an unbounded writer cannot deadlock us. The
/// other reader then sees the pipe close and returns. Finally we reap.
pub(crate) fn capture_with_cap(mut child: Child) -> std::io::Result<Captured> {
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    let kill: KillHandle = Arc::new(Mutex::new(Some(child)));
    let stdout_handle = spawn_reader_stdout(stdout, kill.clone());
    let stderr_handle = spawn_reader_stderr(stderr, kill.clone());
    let (out_buf, out_trunc) = stdout_handle.join().expect("stdout thread");
    let (err_buf, err_trunc) = stderr_handle.join().expect("stderr thread");
    let status_code = reap_child(&kill);
    Ok(Captured {
        stdout: out_buf,
        stderr: err_buf,
        status_code,
        truncated: out_trunc || err_trunc,
    })
}

/// Reap the child (waiting on it) and return its exit code. If a reader
/// already killed the child the lock will hold `None` — fall through to
/// -1 (signaled). Otherwise we still call `wait()` to avoid a zombie.
fn reap_child(kill: &KillHandle) -> i32 {
    let mut guard = kill.lock().expect("kill mutex");
    if let Some(mut c) = guard.take() {
        c.wait().ok().and_then(|s| s.code()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Spawn the stdout reader thread. Returns `(buffer, truncated)`.
fn spawn_reader_stdout(
    stream: ChildStdout,
    kill: KillHandle,
) -> thread::JoinHandle<(Vec<u8>, bool)> {
    thread::spawn(move || read_capped(stream, kill))
}

/// Spawn the stderr reader thread. Returns `(buffer, truncated)`.
fn spawn_reader_stderr(
    stream: ChildStderr,
    kill: KillHandle,
) -> thread::JoinHandle<(Vec<u8>, bool)> {
    thread::spawn(move || read_capped(stream, kill))
}

/// Read until EOF or cap exceeded. On cap exceedance, kill the child
/// IMMEDIATELY via the shared handle then keep draining the pipe so
/// the OS-level pipe buffer empties cleanly (would-be writes by the
/// already-killed child error out fast, EOF arrives quickly).
fn read_capped<R: Read>(mut stream: R, kill: KillHandle) -> (Vec<u8>, bool) {
    let mut buf = Vec::with_capacity(CHUNK);
    let mut chunk = [0u8; CHUNK];
    let mut truncated = false;
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => append_or_truncate(&mut buf, &chunk[..n], &mut truncated, &kill),
            Err(_) => break,
        }
    }
    (buf, truncated)
}

/// Append-or-mark logic, extracted so `read_capped` stays under the 30-LOC
/// ceiling. Side effect: on the cap-crossing call, sends a kill via the
/// shared handle before returning.
fn append_or_truncate(
    buf: &mut Vec<u8>,
    incoming: &[u8],
    truncated: &mut bool,
    kill: &KillHandle,
) {
    if *truncated {
        return; // already past cap; drain silently so writer doesn't block
    }
    if buf.len() + incoming.len() > STREAM_CAP {
        let take = STREAM_CAP.saturating_sub(buf.len());
        buf.extend_from_slice(&incoming[..take]);
        *truncated = true;
        kill_child(kill);
    } else {
        buf.extend_from_slice(incoming);
    }
}

/// Kill the child via the shared handle. Best-effort: if another
/// reader has already killed + reaped, the lock holds `None` and this
/// is a no-op.
fn kill_child(kill: &KillHandle) {
    if let Ok(mut guard) = kill.lock() {
        if let Some(c) = guard.as_mut() {
            let _ = c.kill();
            eprintln!(
                "[kei-runtime] child killed: stdout/stderr exceeded {STREAM_CAP} byte cap"
            );
        }
    }
}
