//! Async event loop — couples the [`App`] state machine to crossterm key
//! events and the daemon SSE stream over a `tokio::mpsc` channel.

use crate::app::{App, LineKind};
use crate::client::chat_stream;
use crate::keys::{handle_key, KeyOutcome};
use crate::types::ChatEvent;
use crate::ui::draw;
use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::StreamExt;
use ratatui::backend::Backend;
use ratatui::Terminal;
use tokio::sync::mpsc;

/// Run the TUI event loop until the user presses Ctrl+D / Ctrl+C twice.
pub async fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    daemon_url: String,
    token: String,
    user_id: String,
) -> Result<()> {
    let mut app = App::new();
    let (tx, mut rx) = mpsc::unbounded_channel::<ChatEvent>();
    let mut keys = EventStream::new();
    while !app.should_quit {
        terminal.draw(|f| draw(f, &app))?;
        tokio::select! {
            maybe_key = keys.next() => {
                if let Some(Ok(Event::Key(k))) = maybe_key {
                    if k.kind != KeyEventKind::Release {
                        dispatch_key(&mut app, k, &daemon_url, &token, &user_id, tx.clone());
                    }
                }
            }
            Some(ev) = rx.recv() => {
                app.apply_event(ev);
            }
        }
    }
    Ok(())
}

/// Hand a [`KeyEvent`](crossterm::event::KeyEvent) to [`handle_key`] and
/// react to the resulting [`KeyOutcome`].
fn dispatch_key(
    app: &mut App,
    k: crossterm::event::KeyEvent,
    daemon_url: &str,
    token: &str,
    user_id: &str,
    tx: mpsc::UnboundedSender<ChatEvent>,
) {
    match handle_key(k, app) {
        KeyOutcome::Send(msg) => start_send(app, msg, daemon_url, token, user_id, tx),
        KeyOutcome::Quit => app.should_quit = true,
        KeyOutcome::Cancel => {
            app.cancel_requested = true;
            app.in_flight = false;
            app.status = "cancelled".into();
        }
        KeyOutcome::Nothing => {}
    }
}

/// Spawn the background daemon-client task for a single send.
fn start_send(
    app: &mut App,
    msg: String,
    daemon_url: &str,
    token: &str,
    user_id: &str,
    tx: mpsc::UnboundedSender<ChatEvent>,
) {
    app.push_line(LineKind::User, msg.clone());
    app.in_flight = true;
    app.status = "streaming…".into();
    let url = daemon_url.to_string();
    let token = token.to_string();
    let uid = user_id.to_string();
    let cid = app.conversation_id.clone();
    tokio::spawn(async move {
        let send = |e: ChatEvent| {
            let _ = tx.send(e);
        };
        if let Err(e) = chat_stream(&url, &token, &uid, &msg, cid, send).await {
            let _ = tx.clone().send(ChatEvent::Error(e.to_string()));
            let _ = tx.send(ChatEvent::Done {
                conversation_id: String::new(),
            });
        }
    });
}
