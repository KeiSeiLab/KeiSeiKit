// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! REST-surface integration tests against a `wiremock` Forgejo stub.
//! No live HTTP — every assertion is local to the test process.

use kei_git_forgejo::{ForgejoBackend, ForgejoClient};
use kei_runtime_core::HasDna;
use kei_runtime_core::traits::git::{GitAuthKind, GitBackend, GitRemote};
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn remote_for(server: &MockServer, owner: &str, name: &str) -> GitRemote {
    GitRemote {
        url: format!("{}/{}/{}.git", server.uri(), owner, name),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    }
}

#[tokio::test]
async fn repo_exists_returns_true_on_200() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/me/demo"))
        .and(header("authorization", "Bearer t"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "demo",
            "default_branch": "main"
        })))
        .mount(&server)
        .await;

    let client = ForgejoClient::with_url(server.uri(), "t").unwrap();
    assert!(client.repo_exists("me", "demo").await.unwrap());
}

#[tokio::test]
async fn repo_exists_returns_false_on_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/me/missing"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = ForgejoClient::with_url(server.uri(), "t").unwrap();
    assert!(!client.repo_exists("me", "missing").await.unwrap());
}

#[tokio::test]
async fn create_user_repo_returns_repo_info_on_201() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/user/repos"))
        .and(header("authorization", "Bearer t"))
        .and(body_partial_json(json!({
            "name": "demo",
            "private": false,
            "default_branch": "main"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "name": "demo",
            "default_branch": "main"
        })))
        .mount(&server)
        .await;

    let client = ForgejoClient::with_url(server.uri(), "t").unwrap();
    let info = client.create_user_repo("demo", false, "main").await.unwrap();
    assert_eq!(info.name, "demo");
    assert_eq!(info.default_branch, "main");
}

#[tokio::test]
async fn ensure_repo_creates_when_missing() {
    let server = MockServer::start().await;
    // First check: repo absent.
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/me/fresh"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;
    // Create endpoint expected exactly once.
    Mock::given(method("POST"))
        .and(path("/api/v1/user/repos"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "name": "fresh",
            "default_branch": "main"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = ForgejoClient::with_url(server.uri(), "t").unwrap();
    let backend = ForgejoBackend::new(client, None).unwrap();
    let remote = remote_for(&server, "me", "fresh");
    backend.ensure_repo(&remote).await.unwrap();
}

#[tokio::test]
async fn ensure_repo_skips_create_when_present() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/repos/me/already"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "already",
            "default_branch": "main"
        })))
        .expect(1)
        .mount(&server)
        .await;
    // No POST stub → if backend tries to create, the test will fail with
    // wiremock's "no matching mock" 404, which surfaces as a non-success.
    let client = ForgejoClient::with_url(server.uri(), "t").unwrap();
    let backend = ForgejoBackend::new(client, None).unwrap();
    let remote = remote_for(&server, "me", "already");
    backend.ensure_repo(&remote).await.unwrap();
}

#[tokio::test]
async fn dna_carries_fj_cap_and_provider_name_is_forgejo() {
    let client = ForgejoClient::with_url("http://localhost", "t").unwrap();
    let backend = ForgejoBackend::new(client, None).unwrap();
    assert_eq!(backend.provider_name(), "forgejo");
    assert!(backend.supports_auto_create());
    let caps = backend.dna().caps();
    assert!(caps.contains("FJ"), "expected FJ in caps, got {caps}");
    assert!(caps.contains("PR"));
    assert!(caps.contains("AP"));
}
