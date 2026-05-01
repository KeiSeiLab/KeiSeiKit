//! Inline unit tests for `memory_review_task.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC after wiring `PersistTarget` + `persist_if_configured`.

use super::*;
use crate::agent::memory_nudge::AgentContext;
use kei_pet::memory::{ensure_schema, recent};
use std::sync::atomic::{AtomicUsize, Ordering};
use tempfile::TempDir;

struct FakeInvoker {
    reply: String,
    calls: Arc<AtomicUsize>,
}

impl Invoker for FakeInvoker {
    fn invoke(
        &self,
        _snapshot: Vec<Turn>,
        _prompt: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let r = self.reply.clone();
        Box::pin(async move { r })
    }
}

fn tag() -> MemoryTag {
    MemoryTag {
        user_id: "alice".into(),
        pet_name: "felix".into(),
    }
}

#[tokio::test]
async fn run_review_short_circuits_on_phrase() {
    let calls = Arc::new(AtomicUsize::new(0));
    let inv = Arc::new(FakeInvoker {
        reply: "Nothing to save.".to_string(),
        calls: calls.clone(),
    });
    let h = ReviewHandles {
        session_id: "s1".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: None,
    };
    let out = run_review(h).await;
    assert!(out.short_circuited);
    assert_eq!(out.wrote_entries, 0);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn run_review_returns_reply_unchanged() {
    let inv = Arc::new(FakeInvoker {
        reply: "Saved a fact about user.".to_string(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let h = ReviewHandles {
        session_id: "s2".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: None,
    };
    let out = run_review(h).await;
    assert!(!out.short_circuited);
    assert_eq!(out.raw_reply, "Saved a fact about user.");
    // No persist target → no write attempted, count stays 0.
    assert_eq!(out.wrote_entries, 0);
}

#[tokio::test]
async fn run_review_persists_to_disk_when_configured() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("review.sqlite");
    let inv = Arc::new(FakeInvoker {
        reply: "User prefers mornings.".into(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let h = ReviewHandles {
        session_id: "s3".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: Some(PersistTarget {
            db_path: db.clone(),
            tag: tag(),
        }),
    };
    let out = run_review(h).await;
    assert!(!out.short_circuited);
    assert_eq!(out.wrote_entries, 1);
    let conn = rusqlite::Connection::open(&db).unwrap();
    ensure_schema(&conn).unwrap();
    let rows = recent(&conn, &tag(), 10).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].text, "User prefers mornings.");
}

#[tokio::test]
async fn run_review_skips_persist_on_short_circuit() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("review.sqlite");
    let inv = Arc::new(FakeInvoker {
        reply: "Nothing to save.".into(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let h = ReviewHandles {
        session_id: "s4".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: Some(PersistTarget {
            db_path: db.clone(),
            tag: tag(),
        }),
    };
    let out = run_review(h).await;
    assert!(out.short_circuited);
    assert_eq!(out.wrote_entries, 0);
    // Database file may not exist at all — short-circuit means we
    // never opened a connection.
}

#[tokio::test]
async fn from_context_returns_some_invoker_when_set() {
    let inv: Arc<dyn Invoker> = Arc::new(FakeInvoker {
        reply: "ok".into(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let ctx = AgentContext {
        session_id: "s5".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: None,
    };
    let h = ReviewHandles::from_context(&ctx);
    assert!(h.invoker.is_some(), "from_context must propagate invoker");
}

#[tokio::test]
async fn from_context_no_longer_returns_none_unconditionally() {
    // Regression test for the dead-code state described in the
    // hermes-batch-2026-04-28 audit (STATUS BANNER): the prior
    // implementation always set invoker = None, causing spawn_review
    // to early-return. Now that AgentContext carries an Option,
    // a configured context propagates through from_context.
    let inv: Arc<dyn Invoker> = Arc::new(FakeInvoker {
        reply: "ok".into(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let with = AgentContext {
        session_id: "s6".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: Some(inv),
        persist: None,
    };
    let without = AgentContext {
        session_id: "s7".into(),
        turns: Arc::new(RwLock::new(vec![])),
        invoker: None,
        persist: None,
    };
    assert!(ReviewHandles::from_context(&with).invoker.is_some());
    assert!(ReviewHandles::from_context(&without).invoker.is_none());
}
