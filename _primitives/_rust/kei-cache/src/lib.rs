//! kei-cache — deterministic caching primitive for pure atom invocations.
//!
//! Entry point is [`wrap_with`]: given a cache [`rusqlite::Connection`], an
//! [`exec::AtomExecutor`], an atom id, JSON input, and a TTL, either
//! return the cached payload or invoke the executor, store the result,
//! and return it.
//!
//! Key derivation lives in [`key`]. Storage lives in [`store`]. Invocation
//! on miss lives in [`exec`]. `lib.rs` only composes them — it owns no
//! persistent state.

pub mod exec;
pub mod key;
pub mod store;

use anyhow::{Context, Result};
use rusqlite::Connection;
use serde_json::Value;

pub use exec::{AtomExecutor, SubprocessExecutor};
pub use store::Stats;

/// Outcome of a [`wrap_with`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Hit,
    Miss,
}

impl Outcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Outcome::Hit => "hit",
            Outcome::Miss => "miss",
        }
    }
}

/// Cache trait — library-level API for downstream consumers.
///
/// Production impl is [`SqliteCache`]. Tests may provide in-memory impls.
pub trait Cache {
    /// Fetch from the cache; `None` if absent or expired.
    fn get(&self, key: &str) -> Result<Option<String>>;
    /// Store `payload` under `key` with TTL (seconds).
    fn put(&self, key: &str, atom_id: &str, payload: &str, ttl_sec: i64) -> Result<()>;
}

/// SQLite-backed cache impl. Holds a borrowed [`Connection`].
pub struct SqliteCache<'c> {
    pub conn: &'c Connection,
}

impl<'c> Cache for SqliteCache<'c> {
    fn get(&self, key: &str) -> Result<Option<String>> {
        store::get(self.conn, key)
    }
    fn put(&self, key: &str, atom_id: &str, payload: &str, ttl_sec: i64) -> Result<()> {
        store::put(self.conn, key, atom_id, payload, ttl_sec)
    }
}

/// Top-level wrap: lookup → return on hit, invoke + store on miss.
///
/// Returns `(payload_string, outcome)`. `payload_string` is the atom's
/// JSON stdout verbatim (trimmed). `outcome` distinguishes hit vs miss
/// so the CLI can emit `cache=hit|miss` to stderr.
pub fn wrap_with<E: AtomExecutor>(
    conn: &Connection,
    executor: &E,
    atom_id: &str,
    input_json: &str,
    ttl_sec: i64,
) -> Result<(String, Outcome)> {
    let input: Value =
        serde_json::from_str(input_json).with_context(|| "input is not valid JSON")?;
    let key = key::cache_key(atom_id, &input);
    if let Some(payload) = store::get(conn, &key)? {
        let _ = store::bump(conn, "hits");
        return Ok((payload, Outcome::Hit));
    }
    let payload = executor
        .execute(atom_id, input_json)
        .with_context(|| format!("execute atom `{atom_id}`"))?;
    store::put(conn, &key, atom_id, &payload, ttl_sec)?;
    let _ = store::bump(conn, "misses");
    Ok((payload, Outcome::Miss))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use tempfile::tempdir;

    struct CountingExec {
        calls: Cell<u32>,
        reply: String,
    }

    impl AtomExecutor for CountingExec {
        fn execute(&self, _atom_id: &str, _input_json: &str) -> Result<String> {
            self.calls.set(self.calls.get() + 1);
            Ok(self.reply.clone())
        }
    }

    #[test]
    fn hit_skips_executor() {
        let d = tempdir().unwrap();
        let p = d.path().join("c.sqlite");
        let conn = store::open(&p).unwrap();
        let ex = CountingExec { calls: Cell::new(0), reply: "{\"r\":1}".into() };
        let (_, o1) = wrap_with(&conn, &ex, "atom:x", "{\"a\":1}", 60).unwrap();
        let (_, o2) = wrap_with(&conn, &ex, "atom:x", "{\"a\":1}", 60).unwrap();
        assert_eq!(o1, Outcome::Miss);
        assert_eq!(o2, Outcome::Hit);
        assert_eq!(ex.calls.get(), 1);
    }
}
