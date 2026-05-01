//! Unit tests for the Slack adapter (filter + dedup logic).

use crate::adapters::slack::convert::EventCallback;
use crate::adapters::slack::dedup::DedupCache;
use crate::adapters::slack::filter_and_dedup;

fn make_dm(channel: &str, ts: &str, text: &str, user: &str) -> EventCallback {
    let json = format!(
        r#"{{"event":{{"type":"message","channel":"{channel}","channel_type":"im",
           "user":"{user}","text":"{text}","ts":"{ts}"}}}}"#
    );
    serde_json::from_str(&json).unwrap()
}

#[test]
fn allow_list_admits_listed_channel() {
    let dedup = DedupCache::default();
    let cb = make_dm("D001", "1.1", "hello", "U1");
    assert!(filter_and_dedup(&cb, &dedup, &["D001".to_string()]).is_some());
}

#[test]
fn allow_list_blocks_unlisted_channel() {
    let dedup = DedupCache::default();
    let cb = make_dm("D999", "1.2", "hello", "U1");
    assert!(filter_and_dedup(&cb, &dedup, &["D001".to_string()]).is_none());
}

#[test]
fn empty_allow_list_admits_all() {
    let dedup = DedupCache::default();
    let cb = make_dm("D999", "1.3", "hello", "U1");
    assert!(filter_and_dedup(&cb, &dedup, &[]).is_some());
}

#[test]
fn dedup_blocks_duplicate_events() {
    let dedup = DedupCache::default();
    let cb = make_dm("D001", "2.0", "dup", "U1");
    assert!(filter_and_dedup(&cb, &dedup, &[]).is_some());
    let cb2 = make_dm("D001", "2.0", "dup", "U1");
    assert!(filter_and_dedup(&cb2, &dedup, &[]).is_none());
}
