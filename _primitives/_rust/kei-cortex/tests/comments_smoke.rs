//! Integration smoke test for `/api/v1/cortex/comments/*`.
//!
//! Spawns the full router on an ephemeral port, walks
//! POST → GET → react → DELETE → GET (asserts deleted=true).
//! `KEI_COMMENTS_DB` is set to a per-test tempfile so the suite
//! never touches the real `~/.keisei/comments.sqlite`.

mod common;

use common::{async_client, spawn};
use reqwest::header;
use serde_json::{json, Value};

#[tokio::test]
async fn comments_lifecycle_post_list_react_delete() {
    let tmp = tempfile::tempdir().unwrap();
    let db = tmp.path().join("comments.sqlite");
    // Set BEFORE spawning the server so the OnceLock picks it up on
    // first handler hit. Safe because each test binary owns its own
    // process and this test is the only one in this binary.
    std::env::set_var("KEI_COMMENTS_DB", &db);

    let srv = spawn().await;
    let client = async_client();
    let auth = format!("Bearer {}", srv.token);
    let page_id = "page-smoke";

    // 1) POST a comment.
    let post = client
        .post(format!("{}/api/v1/cortex/comments/{page_id}", srv.base_url))
        .header(header::AUTHORIZATION, &auth)
        .json(&json!({ "author": "alice", "body": "hello world" }))
        .send()
        .await
        .unwrap();
    assert_eq!(post.status(), 200, "POST should be 200");
    let body: Value = post.json().await.unwrap();
    let id = body["comment_id"].as_str().expect("comment_id").to_string();
    assert_eq!(body["author"], "alice");
    assert_eq!(body["body"], "hello world");

    // 2) GET list contains it.
    let list = client
        .get(format!("{}/api/v1/cortex/comments/{page_id}", srv.base_url))
        .header(header::AUTHORIZATION, &auth)
        .send()
        .await
        .unwrap();
    assert_eq!(list.status(), 200);
    let body: Value = list.json().await.unwrap();
    let arr = body["comments"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["id"], id);

    // 3) POST react.
    let react = client
        .post(format!("{}/api/v1/cortex/comments/{id}/react", srv.base_url))
        .header(header::AUTHORIZATION, &auth)
        .json(&json!({ "author": "bob", "emoji": "👍" }))
        .send()
        .await
        .unwrap();
    assert_eq!(react.status(), 200);
    let body: Value = react.json().await.unwrap();
    assert_eq!(body["ok"], true);

    // 4) DELETE (author = alice → ok).
    let del = client
        .request(
            reqwest::Method::DELETE,
            format!("{}/api/v1/cortex/comments/{id}", srv.base_url),
        )
        .header(header::AUTHORIZATION, &auth)
        .json(&json!({ "author": "alice" }))
        .send()
        .await
        .unwrap();
    assert_eq!(del.status(), 200);
    let body: Value = del.json().await.unwrap();
    assert_eq!(body["ok"], true);

    // 5) GET list — comment is now deleted=true with empty body.
    let list = client
        .get(format!("{}/api/v1/cortex/comments/{page_id}", srv.base_url))
        .header(header::AUTHORIZATION, &auth)
        .send()
        .await
        .unwrap();
    assert_eq!(list.status(), 200);
    let body: Value = list.json().await.unwrap();
    let arr = body["comments"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["deleted"], true);
}

#[tokio::test]
async fn comments_unauthenticated_returns_401() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/comments/some-page", srv.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
}
