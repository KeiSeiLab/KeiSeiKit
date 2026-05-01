// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! wiremock integration tests for BitbucketClient + BitbucketBackend.
//!
//! Required surface (per Wave 5 spec):
//! - repo_exists 200
//! - repo_exists 404
//! - create_repo 200
//! - ensure_repo end-to-end (404 then POST)

use kei_git_bitbucket::{BitbucketBackend, BitbucketClient};
use kei_runtime_core::traits::git::{GitAuthKind, GitBackend, GitRemote};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn repo_exists_200() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repositories/ws/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "uuid": "{u}", "full_name": "ws/repo", "scm": "git", "is_private": true
        })))
        .expect(1)
        .mount(&server)
        .await;
    let c = BitbucketClient::with_url("u", "p", server.uri()).unwrap();
    assert!(c.repo_exists("ws", "repo").await.unwrap());
}

#[tokio::test]
async fn repo_exists_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repositories/ws/missing"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;
    let c = BitbucketClient::with_url("u", "p", server.uri()).unwrap();
    assert!(!c.repo_exists("ws", "missing").await.unwrap());
}

#[tokio::test]
async fn create_repo_200() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/repositories/ws/new"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "uuid": "{abc}", "full_name": "ws/new", "scm": "git", "is_private": true
        })))
        .expect(1)
        .mount(&server)
        .await;
    let c = BitbucketClient::with_url("u", "p", server.uri()).unwrap();
    let repo = c.create_repo("ws", "new").await.unwrap();
    assert_eq!(repo.full_name, "ws/new");
    assert_eq!(repo.scm, "git");
    assert!(repo.is_private);
}

#[tokio::test]
async fn ensure_repo_creates_when_missing() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repositories/ws/repo"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/repositories/ws/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "uuid": "{abc}", "full_name": "ws/repo",
            "scm": "git", "is_private": true
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = BitbucketClient::with_url("u", "p", server.uri()).unwrap();
    let backend = BitbucketBackend::new(client, None).unwrap();
    let remote = GitRemote {
        url: format!("{}/ws/repo", server.uri()),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    };
    backend.ensure_repo(&remote).await.expect("ensure_repo ok");
}

#[tokio::test]
async fn ensure_repo_no_op_when_present() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repositories/ws/repo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "uuid": "{u}", "full_name": "ws/repo", "scm": "git", "is_private": true
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = BitbucketClient::with_url("u", "p", server.uri()).unwrap();
    let backend = BitbucketBackend::new(client, None).unwrap();
    let remote = GitRemote {
        url: format!("{}/ws/repo", server.uri()),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    };
    backend.ensure_repo(&remote).await.expect("ensure_repo ok");
}
