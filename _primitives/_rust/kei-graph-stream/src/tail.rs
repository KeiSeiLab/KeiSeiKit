use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};
use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};

use crate::state::AliveState;

const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Continuously tail `path`, parse events, update alive state, broadcast.
pub async fn run(
    path: PathBuf,
    tx: Arc<broadcast::Sender<String>>,
    alive: Arc<AliveState>,
) -> Result<()> {
    let mut file = tokio::fs::File::open(&path).await?;
    // Seek to end — no history replay.
    let initial_len = file.seek(tokio::io::SeekFrom::End(0)).await?;
    let mut cursor = initial_len;

    loop {
        sleep(POLL_INTERVAL).await;

        let meta = match tokio::fs::metadata(&path).await {
            Ok(m) => m,
            Err(_) => continue,
        };
        let current_len = meta.len();

        if current_len < cursor {
            // File was rotated/truncated — reopen and reset.
            file = tokio::fs::File::open(&path).await?;
            cursor = 0;
        }

        if current_len == cursor {
            continue;
        }

        // Read new bytes from cursor.
        file.seek(tokio::io::SeekFrom::Start(cursor)).await?;
        let mut reader = BufReader::new(&mut file);
        let mut lines_read: u64 = 0;

        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break;
            }
            lines_read += n as u64;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            process_line(trimmed, &tx, &alive);
        }

        cursor += lines_read;
    }
}

fn process_line(
    line: &str,
    tx: &broadcast::Sender<String>,
    alive: &AliveState,
) {
    let Ok(event) = serde_json::from_str::<serde_json::Value>(line) else {
        return;
    };

    match event["event"].as_str() {
        Some("agent_spawn") => alive.insert(&event),
        Some("agent_done") => alive.remove(&event),
        _ => {}
    }

    let frame = match serde_json::to_string(&serde_json::json!({
        "type": "event",
        "data": &event,
    })) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Ignore send errors (no subscribers yet is fine).
    let _ = tx.send(frame);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn tail_detects_new_lines() {
        let mut tmp = NamedTempFile::new().unwrap();
        let path = PathBuf::from(tmp.path());

        let (tx, mut rx) = broadcast::channel::<String>(16);
        let tx = Arc::new(tx);
        let alive = Arc::new(AliveState::new());

        // Spawn tail task (will seek to EOF of empty file → cursor=0).
        let path2 = path.clone();
        let tx2 = Arc::clone(&tx);
        let alive2 = Arc::clone(&alive);
        tokio::spawn(async move { run(path2, tx2, alive2).await });

        // Wait for first poll cycle.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Append a spawn event.
        let ev = json!({"ts":"2026-05-02T13:00:00Z","event":"agent_spawn","id":"t1","subagent_type":"researcher","model":"sonnet","prompt_preview":"test"});
        writeln!(tmp, "{}", ev.to_string()).unwrap();

        // Allow poll to pick it up.
        tokio::time::sleep(Duration::from_millis(400)).await;

        let msg = rx.recv().await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["type"], "event");
        assert_eq!(parsed["data"]["event"], "agent_spawn");

        // Alive state should contain t1.
        let snap = alive.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].id, "t1");
    }
}
