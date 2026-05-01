//! Single-threaded pump: reads `Result<notify::Event>` from notify's
//! channel, maps + debounces, pushes canonical [`Event`] to the output
//! channel consumed by `next_event` / `drain`.
//!
//! Exactly one thread is spawned per [`crate::Watcher`] instance. The
//! thread exits cleanly when notify's sender is dropped (closing the
//! input channel), which happens when the `notify::RecommendedWatcher`
//! is dropped inside [`crate::Watcher::drop`].

use crate::debounce::Debouncer;
use crate::event::Event;
use crate::map;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Spawn the pump thread.
///
/// `notify_rx` — source end of notify's event channel.
/// `out_tx`    — destination channel; each accepted canonical event is
///               forwarded here.
/// Returns the thread handle so the watcher can join on drop.
pub fn spawn(
    notify_rx: Receiver<notify::Result<notify::Event>>,
    out_tx: Sender<Event>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name("kei-watch-pump".into())
        .spawn(move || run(notify_rx, out_tx))
        .expect("spawn kei-watch pump thread")
}

fn run(notify_rx: Receiver<notify::Result<notify::Event>>, out_tx: Sender<Event>) {
    let mut deb = Debouncer::new();
    while let Ok(res) = notify_rx.recv() {
        let Ok(notify_ev) = res else { continue };
        for canon in map::from_notify(&notify_ev) {
            if !deb.accept(&canon) {
                continue;
            }
            // out_tx dropped (watcher released) → exit cleanly.
            if out_tx.send(canon).is_err() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventKind;
    use notify::event::{CreateKind, EventKind as NK};
    use std::path::PathBuf;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn pump_forwards_canonical_event() {
        let (n_tx, n_rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let (o_tx, o_rx) = mpsc::channel::<Event>();
        let h = spawn(n_rx, o_tx);

        let mut e = notify::Event::new(NK::Create(CreateKind::File));
        e.paths = vec![PathBuf::from("/tmp/x")];
        n_tx.send(Ok(e)).unwrap();
        drop(n_tx);

        let ev = o_rx.recv_timeout(Duration::from_millis(500)).unwrap();
        assert_eq!(ev.kind, EventKind::Created);
        h.join().unwrap();
    }
}
