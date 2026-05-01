//! kei-watch CLI — streams canonical FS events as JSON Lines.
//!
//! Usage:
//! ```text
//! kei-watch watch --path <DIR> [--recursive] [--timeout-ms <N>]
//! ```
//!
//! Each event is one JSON object per line, flushed per event:
//! `{"kind":"Modified","path":"/abs/path","from":null,"ts":1712345678}`.
//!
//! Exits after `--timeout-ms` of no activity. Without the flag, runs
//! until killed (Ctrl-C).

use clap::{Parser, Subcommand};
use kei_watch::{Event, Watcher};
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "kei-watch", version, about = "Filesystem watcher primitive")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Watch a path and emit JSON-line events to stdout.
    Watch {
        /// Path to watch (file or directory).
        #[arg(long)]
        path: PathBuf,
        /// Recurse into subdirectories.
        #[arg(long)]
        recursive: bool,
        /// Exit after this many ms without activity. Omit → run forever.
        #[arg(long)]
        timeout_ms: Option<u64>,
    },
}

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-watch: {msg}");
    ExitCode::from(1)
}

fn event_to_json_line(ev: &Event) -> String {
    // Compact, stable shape — not using serde_json::to_string on Event
    // because we want `from` (short) rather than `from_path` (long).
    let from = match &ev.from_path {
        Some(p) => serde_json::Value::String(p.to_string_lossy().into_owned()),
        None => serde_json::Value::Null,
    };
    let obj = serde_json::json!({
        "kind": ev.kind.as_str(),
        "path": ev.path.to_string_lossy(),
        "from": from,
        "ts": ev.timestamp,
    });
    obj.to_string()
}

fn emit_event(ev: &Event) -> std::io::Result<()> {
    let line = event_to_json_line(ev);
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    writeln!(lock, "{line}")?;
    lock.flush()
}

fn run_watch(path: PathBuf, recursive: bool, timeout_ms: Option<u64>) -> ExitCode {
    let mut watcher = match Watcher::new() {
        Ok(w) => w,
        Err(e) => return err(&format!("new: {e}")),
    };
    if let Err(e) = watcher.watch(&path, recursive) {
        return err(&format!("watch {}: {e}", path.display()));
    }
    let step = Duration::from_millis(500);
    let limit = timeout_ms.unwrap_or(u64::MAX);
    let mut idle_ms: u64 = 0;
    loop {
        match watcher.next_event(step) {
            Some(ev) => {
                if emit_event(&ev).is_err() {
                    return ExitCode::SUCCESS;
                }
                idle_ms = 0;
            }
            None => {
                idle_ms = idle_ms.saturating_add(step.as_millis() as u64);
                if idle_ms >= limit {
                    return ExitCode::SUCCESS;
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Watch { path, recursive, timeout_ms } => run_watch(path, recursive, timeout_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_watch::EventKind;
    use std::path::PathBuf;

    #[test]
    fn json_line_has_required_fields() {
        let ev = Event::new(EventKind::Modified, PathBuf::from("/x"), None);
        let line = event_to_json_line(&ev);
        assert!(line.contains("\"kind\":\"Modified\""));
        assert!(line.contains("\"path\":\"/x\""));
        assert!(line.contains("\"from\":null"));
        assert!(line.contains("\"ts\":"));
    }

    #[test]
    fn json_line_includes_from_when_renamed() {
        let ev = Event::new(
            EventKind::Renamed,
            PathBuf::from("/b"),
            Some(PathBuf::from("/a")),
        );
        let line = event_to_json_line(&ev);
        assert!(line.contains("\"from\":\"/a\""));
        assert!(line.contains("\"path\":\"/b\""));
    }
}
