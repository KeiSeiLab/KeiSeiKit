//! PTY lifecycle for the `/term` WebSocket handler.
//!
//! Owns the spawned shell child + its reader/writer halves and tears them
//! down deterministically on `Drop`. Wave 44b fixes:
//!
//! - F-MED-2: `child.kill()` is now followed by `child.wait()` so we don't
//!   leak a zombie per disconnected session.
//! - MISS-2: the PTY reader runs on `tokio::task::spawn_blocking` (instead
//!   of a bare `std::thread::spawn`) and watches a shared `AtomicBool`
//!   cancellation flag, so daemon shutdown / WS disconnect cleanly stops
//!   the reader instead of leaking a thread per session.

use portable_pty::{native_pty_system, Child, CommandBuilder, PtySize};
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// PTY initial size — sane default; client resize ack lands in v0.41.
const PTY_ROWS: u16 = 30;
const PTY_COLS: u16 = 100;

/// Read chunk size from PTY stdout. Anything larger fragments without
/// noticeably reducing latency on local sockets.
const READ_CHUNK: usize = 4096;

/// Bag of PTY handles + bookkeeping we keep alive for the connection
/// lifetime. Drop order: stop reader → kill child → wait child → drop the
/// writer/master halves the bag was holding.
pub struct PtyBag {
    pub writer: Box<dyn std::io::Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    cancel: Arc<AtomicBool>,
    reader_handle: Option<JoinHandle<()>>,
}

impl Drop for PtyBag {
    fn drop(&mut self) {
        // 1. Tell the reader to stop on its next iteration.
        self.cancel.store(true, Ordering::SeqCst);
        // 2. Kill the child shell (best-effort; already-dead children are
        //    fine — kill returns Ok or InvalidInput depending on platform).
        let _ = self.child.kill();
        // 3. Reap the child to avoid zombies. `wait()` is sync and OK in
        //    Drop because spawn_blocking owns the long-blocking call; this
        //    is the post-kill reap which returns near-instantly.
        let _ = self.child.wait();
        // 4. Abort the reader handle if it hasn't observed the flag yet.
        //    `spawn_blocking` futures can't be aborted, but dropping the
        //    `JoinHandle` detaches it; the cancel flag above guarantees
        //    the read loop exits on its next iteration.
        if let Some(handle) = self.reader_handle.take() {
            handle.abort();
        }
    }
}

/// Spawn `$SHELL` (or `/bin/sh` fallback) in a PTY anchored at `cwd` and
/// kick off the reader task. The bag returned holds every handle the WS
/// driver needs.
pub fn spawn_pty(
    cwd: &Path,
    out_tx: mpsc::Sender<Vec<u8>>,
) -> Result<PtyBag, String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: PTY_ROWS,
            cols: PTY_COLS,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    let mut cmd = CommandBuilder::new(&shell);
    cmd.cwd(cwd);
    apply_safe_env(&mut cmd);
    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let cancel = Arc::new(AtomicBool::new(false));
    let reader_handle = spawn_reader(reader, out_tx, cancel.clone());
    Ok(PtyBag {
        writer,
        child,
        cancel,
        reader_handle: Some(reader_handle),
    })
}

/// SECURITY: drop ALL inherited env so daemon secrets (KEI_AUTH_KEY,
/// ANTHROPIC_API_KEY, MAGICLINK_HMAC_KEY, etc.) cannot leak into the PTY
/// shell. A stored XSS on the cors_origin domain would otherwise pivot
/// directly to a local shell holding every daemon secret. After clearing,
/// re-set ONLY the minimal env a shell legitimately needs.
fn apply_safe_env(cmd: &mut CommandBuilder) {
    cmd.env_clear();
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let path = std::env::var("PATH")
        .unwrap_or_else(|_| "/usr/local/bin:/usr/bin:/bin".into());
    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
    cmd.env("HOME", &home);
    cmd.env("PATH", &path);
    cmd.env("USER", &user);
    cmd.env("LANG", "en_US.UTF-8");
    cmd.env("TERM", "xterm-256color");
}

/// Forward PTY stdout bytes to the WS sender via a bounded channel. Reads
/// happen on a `spawn_blocking` task because the underlying `Read` is sync;
/// the loop checks `cancel` every iteration so disconnect / shutdown
/// terminates the task promptly without leaking the thread.
fn spawn_reader(
    mut reader: Box<dyn Read + Send>,
    out_tx: mpsc::Sender<Vec<u8>>,
    cancel: Arc<AtomicBool>,
) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        let mut buf = vec![0u8; READ_CHUNK];
        loop {
            if cancel.load(Ordering::SeqCst) {
                break;
            }
            if out_tx.is_closed() {
                break;
            }
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if out_tx.blocking_send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    })
}
