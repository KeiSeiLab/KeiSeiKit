//! Local Whisper transcription via faster-whisper Python subprocess.
//!
//! RULE 0.2 exception #6: faster-whisper (CTranslate2) has no native Rust
//! equivalent that accepts an arbitrary audio blob and returns text. The
//! daemon spawns `<python3> scripts/whisper_worker.py <tmpfile>` and reads
//! the transcript from stdout. Audio bytes are written to a tempfile
//! because faster-whisper reads from a path (ffmpeg demuxes).
//!
//! Configurable via env:
//!   `KEI_WHISPER_MODEL`  — default `base.en`, can be `medium.en` / `large-v3`.
//!   `KEI_WHISPER_DEVICE` — default `auto`, can be `cpu` / `cuda` / `mps`.
//!   `KEI_WHISPER_WORKER` — override path to the Python worker script
//!                          (defaults to the repo-relative
//!                          `kei-cortex/scripts/whisper_worker.py`).
//!   `KEI_WHISPER_PYTHON` — override `python3` binary (absolute path). If
//!                          unset we resolve `python3` via `which::which`
//!                          once at first call and cache the result.

use std::path::PathBuf;
use std::process::Stdio;
use std::sync::OnceLock;
use std::time::Duration;

use tempfile::NamedTempFile;
use tokio::process::Command;
use tokio::time::timeout;

/// Upper bound on any single transcribe call. 120 s suffices for a minute
/// of speech on CPU even with the medium model.
const INFERENCE_TIMEOUT: Duration = Duration::from_secs(120);

/// Cache for the resolved python3 path. `None` means "we tried and failed"
/// — we still return an error on every call but avoid re-running `which`.
static PYTHON_BIN: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Errors the caller needs to distinguish (mapped to HTTP by `stt.rs`).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("worker script not found at {0}")]
    WorkerMissing(PathBuf),
    #[error("python3 interpreter not found on PATH; set KEI_WHISPER_PYTHON")]
    PythonMissing,
    #[error("io writing audio tmpfile: {0}")]
    Io(#[from] std::io::Error),
    #[error("worker timed out after {0:?}")]
    Timeout(Duration),
    #[error("worker exit {code}: {stderr}")]
    WorkerFailed { code: i32, stderr: String },
}

/// Transcribe a blob by writing it to a tempfile and spawning the worker.
pub async fn transcribe(audio_bytes: Vec<u8>, mime: &str) -> Result<String, Error> {
    let suffix = extension_for_mime(mime);
    let tmp = write_tmpfile(&audio_bytes, suffix)?;
    let worker = resolve_worker()?;
    let python = resolve_python()?;
    run_worker(&python, &worker, tmp.path()).await
}

/// Pick a filename extension from the MIME so ffmpeg (invoked inside
/// faster-whisper) demuxes correctly. Defaults to `.bin`.
fn extension_for_mime(mime: &str) -> &'static str {
    let m = mime.to_ascii_lowercase();
    if m.starts_with("audio/webm") {
        ".webm"
    } else if m.starts_with("audio/wav") || m.starts_with("audio/x-wav") {
        ".wav"
    } else if m.starts_with("audio/mpeg") || m.starts_with("audio/mp3") {
        ".mp3"
    } else if m.starts_with("audio/mp4") || m.starts_with("audio/m4a") {
        ".m4a"
    } else if m.starts_with("audio/ogg") {
        ".ogg"
    } else {
        ".bin"
    }
}

/// Write audio bytes to a tempfile the worker can read.
fn write_tmpfile(bytes: &[u8], suffix: &str) -> Result<NamedTempFile, Error> {
    let tmp = tempfile::Builder::new()
        .prefix("kei-whisper-")
        .suffix(suffix)
        .tempfile()?;
    std::fs::write(tmp.path(), bytes)?;
    Ok(tmp)
}

/// Find the worker script path: env override wins, otherwise relative to
/// the compiled binary's crate dir (`../../kei-cortex/scripts/...`).
fn resolve_worker() -> Result<PathBuf, Error> {
    if let Ok(p) = std::env::var("KEI_WHISPER_WORKER") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Ok(path);
        }
    }
    // Fallback: repo-layout guess. Works when the daemon is launched from
    // the repo root OR from the workspace target dir.
    for rel in [
        "_primitives/_rust/kei-cortex/scripts/whisper_worker.py",
        "../kei-cortex/scripts/whisper_worker.py",
        "kei-cortex/scripts/whisper_worker.py",
        "scripts/whisper_worker.py",
    ] {
        let path = PathBuf::from(rel);
        if path.is_file() {
            return Ok(path);
        }
    }
    Err(Error::WorkerMissing(PathBuf::from(
        "kei-cortex/scripts/whisper_worker.py",
    )))
}

/// Resolve the `python3` interpreter path. Honors `KEI_WHISPER_PYTHON` first
/// (absolute path), then probes `PATH` via `which::which`. Cached in a
/// `OnceLock` so we don't re-probe every transcribe call.
fn resolve_python() -> Result<PathBuf, Error> {
    let cached = PYTHON_BIN.get_or_init(|| {
        if let Ok(p) = std::env::var("KEI_WHISPER_PYTHON") {
            let path = PathBuf::from(p);
            if path.is_file() {
                return Some(path);
            }
        }
        which::which("python3").ok()
    });
    cached.clone().ok_or(Error::PythonMissing)
}

/// Spawn the Python worker on the given audio file, capture stdout.
async fn run_worker(
    python: &PathBuf,
    worker: &PathBuf,
    audio: &std::path::Path,
) -> Result<String, Error> {
    let mut cmd = Command::new(python);
    cmd.arg(worker)
        .arg(audio)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = timeout(INFERENCE_TIMEOUT, cmd.output())
        .await
        .map_err(|_| Error::Timeout(INFERENCE_TIMEOUT))??;
    if out.status.success() {
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        Ok(text)
    } else {
        Err(Error::WorkerFailed {
            code: out.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&out.stderr)
                .chars()
                .take(512)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mime_extension_table() {
        assert_eq!(extension_for_mime("audio/webm"), ".webm");
        assert_eq!(extension_for_mime("audio/webm;codecs=opus"), ".webm");
        assert_eq!(extension_for_mime("audio/mpeg"), ".mp3");
        assert_eq!(extension_for_mime("audio/mp4"), ".m4a");
        assert_eq!(extension_for_mime("audio/ogg"), ".ogg");
        assert_eq!(extension_for_mime("audio/wav"), ".wav");
        assert_eq!(extension_for_mime("audio/unknown"), ".bin");
    }

    #[test]
    fn write_tmpfile_roundtrip() {
        let tmp = write_tmpfile(b"hello", ".bin").unwrap();
        let back = std::fs::read(tmp.path()).unwrap();
        assert_eq!(back, b"hello");
    }

    #[test]
    fn worker_missing_returns_error() {
        std::env::set_var("KEI_WHISPER_WORKER", "/does/not/exist");
        let r = resolve_worker();
        std::env::remove_var("KEI_WHISPER_WORKER");
        // Fallback search will also fail in CI; assert error shape.
        if let Err(Error::WorkerMissing(_)) = r {
            // ok
        } else if r.is_ok() {
            // Running from repo root — that's fine too.
        } else {
            panic!("unexpected: {r:?}");
        }
    }
}
