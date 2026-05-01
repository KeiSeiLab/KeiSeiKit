// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock-only integration tests. No live HTTP, no `git` CLI calls.
//! Covers: repo_exists 200/404, create_user_repo 201, ensure_repo
//! end-to-end (404 → POST 201).

use kei_git_gitea::{CreateRepoRequest, GiteaBackend, GiteaClient};
use kei_runtime_core::traits::git::{GitAuthKind, GitBackend, GitRemote};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn backend_for(server: &MockServer) -> GiteaBackend {
    let client = GiteaClient::new(server.uri(), "test-token");
    GiteaBackend::new(client, None).expect("backend new")
}

#[tokio::test]
async fn repo_exists_200_and_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/present"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "full_name": "alice/present",
            "default_branch": "main",
            "private": true
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/missing"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let b = backend_for(&server);
    assert!(b.client().repo_exists("alice", "present").await.unwrap());
    assert!(!b.client().repo_exists("alice", "missing").await.unwrap());
}

#[tokio::test]
async fn create_user_repo_201() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/user/repos"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "full_name": "alice/new-repo",
            "default_branch": "main",
            "private": true
        })))
        .mount(&server)
        .await;

    let b = backend_for(&server);
    let info = b
        .client()
        .create_user_repo(&CreateRepoRequest::new("new-repo"))
        .await
        .unwrap();
    assert_eq!(info.full_name, "alice/new-repo");
    assert!(info.private);
    assert_eq!(info.default_branch, "main");
}

#[tokio::test]
async fn ensure_repo_creates_when_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/fresh"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/user/repos"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "full_name": "alice/fresh",
            "default_branch": "main",
            "private": true
        })))
        .mount(&server)
        .await;

    let b = backend_for(&server);
    let remote = GitRemote {
        url: format!("{}/alice/fresh.git", server.uri()),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    };
    b.ensure_repo(&remote).await.expect("ensure_repo ok");
}

#[tokio::test]
async fn ensure_repo_skips_create_when_present() {
    let server = MockServer::start().await;
    // Only the GET should be hit; if a POST happens with no Mock, wiremock
    // returns 404 and the test fails on Error::Api.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/alice/extant"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "full_name": "alice/extant",
            "default_branch": "main",
            "private": true
        })))
        .mount(&server)
        .await;

    let b = backend_for(&server);
    let remote = GitRemote {
        url: format!("{}/alice/extant.git", server.uri()),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    };
    b.ensure_repo(&remote).await.expect("ensure_repo ok (no create)");
}

#[tokio::test]
async fn provider_metadata() {
    let server = MockServer::start().await;
    let b = backend_for(&server);
    assert_eq!(b.provider_name(), "gitea");
    assert!(b.supports_auto_create());
    assert!(b.client().base_url().starts_with("http"));
}
