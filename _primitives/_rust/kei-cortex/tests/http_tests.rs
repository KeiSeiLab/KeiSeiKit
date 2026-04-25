//! HTTP integration tests — spawn the full router on an ephemeral port
//! and exercise the public endpoints. Each test owns its own TempDir.

mod common;

use common::{async_client, spawn, write_minimal_pet};
use reqwest::header;
use serde_json::Value;

#[tokio::test]
async fn healthz_unauthenticated_returns_ok() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/healthz", srv.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "ok");
}

#[tokio::test]
async fn protected_route_without_token_returns_401() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/summary", srv.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 401);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "unauthorized");
}

#[tokio::test]
async fn protected_route_with_wrong_token_returns_403() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/summary", srv.base_url))
        .header(header::AUTHORIZATION, "Bearer deadbeef")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 403);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "forbidden");
}

#[tokio::test]
async fn summary_returns_valid_json_shape() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/summary", srv.base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["total_dnas"].is_i64());
    assert!(body["active_pets"].is_array());
    assert!(body["recent_sessions"].is_i64());
}

#[tokio::test]
async fn pet_get_404_when_file_missing() {
    let srv = spawn().await;
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/pet/nobody", srv.base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn pet_get_returns_parsed_manifest() {
    let srv = spawn().await;
    write_minimal_pet(&srv.config.pet_root, "alex");
    let resp = async_client()
        .get(format!("{}/api/v1/cortex/pet/alex", srv.base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["pet"]["identity"]["pet_name"], "Kei");
}

#[tokio::test]
async fn interaction_post_returns_201_and_id() {
    let srv = spawn().await;
    write_minimal_pet(&srv.config.pet_root, "alex");
    let resp = async_client()
        .post(format!("{}/api/v1/cortex/pet/alex/interaction", srv.base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", srv.token))
        .json(&serde_json::json!({
            "role": "user",
            "text": "hello",
            "ts": 1_700_000_000_i64
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let body: Value = resp.json().await.unwrap();
    assert!(body["interaction_id"].as_i64().unwrap() >= 1);
}

#[tokio::test]
async fn cors_preflight_allows_configured_origin() {
    let srv = spawn().await;
    let resp = async_client()
        .request(
            reqwest::Method::OPTIONS,
            format!("{}/api/v1/cortex/summary", srv.base_url),
        )
        .header("Origin", "https://keisei.app")
        .header("Access-Control-Request-Method", "GET")
        .header("Access-Control-Request-Headers", "authorization")
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success() || resp.status().as_u16() == 204);
    let allow_origin = resp
        .headers()
        .get("access-control-allow-origin")
        .expect("ACAO present")
        .to_str()
        .unwrap()
        .to_string();
    assert_eq!(allow_origin, "https://keisei.app");
}
