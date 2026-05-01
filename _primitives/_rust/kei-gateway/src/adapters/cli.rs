//! CLI adapter — stdin/stdout async loop.
//!
//! Reads one line at a time from stdin, wraps it in a [`MessageEvent`] under
//! [`Platform::Cli`], pushes it onto the inbound channel. Outbound messages
//! print to stdout, prefixed with `>>>` for visual separation.
//!
//! This is the only fully-wired adapter in P4.1 MVP. Telegram / Discord /
//! Slack are stubs (see siblings).

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{mpsc, Mutex};

use crate::adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
use crate::message::{ChatType, MessageEvent, Platform, SessionSource};

/// Tunables for the CLI adapter.
pub struct CliAdapter {
    /// Logical chat_id used in inbound events (defaults to "stdin").
    pub chat_id: String,
    /// Synchronises stdout writes so concurrent sends don't interleave bytes.
    out_lock: Mutex<()>,
}

impl Default for CliAdapter {
    fn default() -> Self {
        Self::new("stdin")
    }
}

impl CliAdapter {
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            out_lock: Mutex::new(()),
        }
    }

    fn build_event(&self, line: String) -> MessageEvent {
        let source = SessionSource {
            platform: Platform::Cli,
            chat_type: ChatType::Dm,
            chat_id: Some(self.chat_id.clone()),
            user_id: Some("local".into()),
            user_id_alt: None,
            thread_id: None,
        };
        MessageEvent::new(line, source)
    }
}

#[async_trait]
impl PlatformAdapter for CliAdapter {
    fn platform(&self) -> Platform {
        Platform::Cli
    }

    async fn connect(&self) -> Result<()> {
        // No-op for stdio; we lazily attach in recv_loop / send.
        Ok(())
    }

    async fn send(&self, msg: OutboundMessage) -> Result<SendResult> {
        let _g = self.out_lock.lock().await;
        let mut stdout = tokio::io::stdout();
        stdout.write_all(b">>> ").await?;
        stdout.write_all(msg.text.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
        Ok(SendResult::ok(None))
    }

    async fn recv_loop(&self, tx: mpsc::Sender<MessageEvent>) -> Result<()> {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();
        while let Some(line) = reader.next_line().await? {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            let event = self.build_event(line);
            if tx.send(event).await.is_err() {
                // Receiver dropped — runner shut down.
                break;
            }
        }
        Ok(())
    }
}
