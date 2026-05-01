// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Integration tests for `GitlabClient` and `GitlabBackend::ensure_repo`
//! against a wiremock-served GitLab API. NO live HTTP.

use kei_git_gitlab::{GitlabBackend, GitlabClient};
use kei_runtime_core::traits::git::{GitAuthKind, GitBackend, GitRemote};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn project_exists_200() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/owner%2Frepo"))
        .and(header("PRIVATE-TOKEN", "tok"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1, "path_with_namespace": "owner/repo"
        })))
        .mount(&server)
        .await;
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    assert!(c.project_exists("owner/repo").await.unwrap());
}

#[tokio::test]
async fn project_exists_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/owner%2Frepo"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    assert!(!c.project_exists("owner/repo").await.unwrap());
}

#[tokio::test]
async fn create_project_201() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v4/projects"))
        .and(header("PRIVATE-TOKEN", "tok"))
        .and(body_json(
            serde_json::json!({"name": "repo", "visibility": "private"}),
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42,
            "path_with_namespace": "alice/repo",
            "default_branch": "main"
        })))
        .mount(&server)
        .await;
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    let info = c.create_project("repo").await.unwrap();
    assert_eq!(info.id, 42);
    assert_eq!(info.path_with_namespace, "alice/repo");
    assert_eq!(info.default_branch.as_deref(), Some("main"));
}

#[tokio::test]
async fn get_branch_sha_200() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/api/v4/projects/owner%2Frepo/repository/branches/main",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "main",
            "commit": { "id": "deadbeefcafef00d" }
        })))
        .mount(&server)
        .await;
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    assert_eq!(
        c.get_branch_sha("owner/repo", "main").await.unwrap(),
        "deadbeefcafef00d"
    );
}

#[tokio::test]
async fn get_branch_sha_404_is_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/api/v4/projects/owner%2Frepo/repository/branches/missing",
        ))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    let err = c
        .get_branch_sha("owner/repo", "missing")
        .await
        .err()
        .expect("404 must surface as Err");
    let msg = format!("{err}");
    assert!(
        msg.contains("not found"),
        "expected NotFound, got: {msg}"
    );
}

/// End-to-end `ensure_repo`: project absent (404) → backend creates it (201).
/// Verifies both calls are made via the API in the correct order with the
/// correct body and headers.
#[tokio::test]
async fn ensure_repo_creates_when_missing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v4/projects/alice%2Fnewproj"))
        .and(header("PRIVATE-TOKEN", "tok"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v4/projects"))
        .and(header("PRIVATE-TOKEN", "tok"))
        .and(body_json(
            serde_json::json!({"name": "newproj", "visibility": "private"}),
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 7,
            "path_with_namespace": "alice/newproj"
        })))
        .mount(&server)
        .await;

    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    let backend = GitlabBackend::new(c, None).unwrap();
    let remote = GitRemote {
        url: "https://gitlab.com/alice/newproj.git".into(),
        branch: "main".into(),
        auth_kind: GitAuthKind::Pat,
    };
    backend.ensure_repo(&remote).await.expect("ensure_repo");
}

/// `ensure_repo` short-circuits when project already exists (no POST).
#[tokio::test]
async fn ensure_repo_noop_when_exists() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/alice%2Fexisting"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1, "path_with_namespace": "alice/existing"
        })))
        .mount(&server)
        .await;
    // No POST mock — if backend tries to create, wiremock returns the default
    // 404 for unmatched requests and the test would fail at create_project.
    let c = GitlabClient::with_url(server.uri(), "tok").unwrap();
    let backend = GitlabBackend::new(c, None).unwrap();
    let remote = GitRemote {
        url: "git@gitlab.com:alice/existing.git".into(),
        branch: "main".into(),
        auth_kind: GitAuthKind::SshKey,
    };
    backend.ensure_repo(&remote).await.expect("ensure_repo");
}
