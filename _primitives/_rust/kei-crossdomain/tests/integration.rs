use kei_crossdomain::auto_link::auto_link;
use kei_crossdomain::bfs::bfs;
use kei_crossdomain::edges::{count_by_type, link, query_edges};
use kei_crossdomain::Store;

fn mk() -> Store { Store::open_memory().unwrap() }

#[test]
fn link_and_query() {
    let s = mk();
    link(&s, "code://a.rs", "note://n1", "documents", 1.0, "E2").unwrap();
    let e = query_edges(&s, "code://a.rs").unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].to_uri, "note://n1");
}

#[test]
fn bfs_crosses_domains() {
    let s = mk();
    link(&s, "code://x", "note://y", "refs", 1.0, "E2").unwrap();
    link(&s, "note://y", "task://z", "linked", 1.0, "E2").unwrap();
    let r = bfs(&s, "code://x", 2).unwrap();
    let uris: Vec<&str> = r.iter().map(|rr| rr.uri.as_str()).collect();
    assert!(uris.contains(&"note://y"));
    assert!(uris.contains(&"task://z"));
}

#[test]
fn auto_link_cross_domain() {
    let s = mk();
    link(&s, "code://a/router", "note://tmp", "seed", 1.0, "E3").unwrap();
    link(&s, "task://epic/router", "note://tmp2", "seed", 1.0, "E3").unwrap();
    let added = auto_link(&s, "code://a/router").unwrap();
    assert!(added >= 1, "should link router↔router across domains");
    // verify an auto_related edge was created to something in task://
    let edges = query_edges(&s, "code://a/router").unwrap();
    assert!(edges.iter().any(|e| e.edge_type == "auto_related" && e.to_uri.starts_with("task://")));
}

#[test]
fn edge_type_stats() {
    let s = mk();
    link(&s, "a://x", "b://y", "refs", 1.0, "E2").unwrap();
    link(&s, "a://x", "b://z", "refs", 1.0, "E2").unwrap();
    link(&s, "a://x", "b://w", "doc", 1.0, "E2").unwrap();
    let counts = count_by_type(&s).unwrap();
    let refs = counts.iter().find(|(t, _)| t == "refs").unwrap().1;
    assert_eq!(refs, 2);
}

#[test]
fn bfs_depth_limit() {
    let s = mk();
    link(&s, "a://1", "b://2", "r", 1.0, "E2").unwrap();
    link(&s, "b://2", "c://3", "r", 1.0, "E2").unwrap();
    link(&s, "c://3", "d://4", "r", 1.0, "E2").unwrap();
    let r = bfs(&s, "a://1", 2).unwrap();
    let uris: Vec<&str> = r.iter().map(|rr| rr.uri.as_str()).collect();
    assert!(uris.contains(&"b://2"));
    assert!(uris.contains(&"c://3"));
    assert!(!uris.contains(&"d://4"));
}
