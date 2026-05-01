//! Inbound dedup for the Discord adapter.
//!
//! Discord's gateway occasionally re-delivers the same `Message` event on
//! reconnect. We hash `(channel_id, message_id, text)` with blake3 and keep
//! a bounded LRU set — duplicates are silently dropped before we push a
//! [`MessageEvent`] downstream.

use std::num::NonZeroUsize;
use std::sync::Mutex;

use lru::LruCache;

/// Default capacity. ~512 hashes × 32 bytes ≈ 16 KiB — fits in cache.
pub const DEFAULT_CAPACITY: usize = 512;

/// Bounded "have-I-seen-this-before" set keyed on a 32-byte blake3 digest.
pub struct DedupCache {
    inner: Mutex<LruCache<[u8; 32], ()>>,
}

impl DedupCache {
    /// Build a dedup cache holding up to `capacity` recent message hashes.
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).expect("non-zero");
        Self {
            inner: Mutex::new(LruCache::new(cap)),
        }
    }

    /// True iff `hash` had already been seen. Touches the LRU on hit.
    /// Inserts on miss so the next call returns true.
    pub fn observe(&self, hash: [u8; 32]) -> bool {
        let mut g = self.inner.lock().expect("poisoned");
        if g.get(&hash).is_some() {
            return true;
        }
        g.put(hash, ());
        false
    }
}

impl Default for DedupCache {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

/// Hash `(channel_id, message_id, text)` to a stable 32-byte digest.
///
/// Discord uses `u64` snowflake IDs; we serialise as little-endian bytes.
pub fn message_digest(channel_id: u64, message_id: u64, text: &str) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&channel_id.to_le_bytes());
    h.update(&message_id.to_le_bytes());
    h.update(text.as_bytes());
    *h.finalize().as_bytes()
}

#[cfg(test)]
#[cfg(feature = "discord")]
mod tests {
    use super::*;

    #[test]
    fn first_observation_returns_false() {
        let c = DedupCache::default();
        assert!(!c.observe(message_digest(1, 2, "hi")));
    }

    #[test]
    fn second_observation_of_same_hash_returns_true() {
        let c = DedupCache::default();
        let h = message_digest(1, 2, "hi");
        assert!(!c.observe(h));
        assert!(c.observe(h));
    }

    #[test]
    fn distinct_messages_do_not_collide() {
        let c = DedupCache::default();
        let a = message_digest(1, 2, "hi");
        let b = message_digest(1, 2, "bye");
        assert!(!c.observe(a));
        assert!(!c.observe(b));
    }

    #[test]
    fn capacity_evicts_oldest() {
        let c = DedupCache::new(2);
        let a = message_digest(1, 1, "a");
        let b = message_digest(1, 2, "b");
        let z = message_digest(1, 9, "z");
        c.observe(a);
        c.observe(b);
        c.observe(z); // a evicted
        assert!(!c.observe(a)); // re-inserted as fresh
    }
}
