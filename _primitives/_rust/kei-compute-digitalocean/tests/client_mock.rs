// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `wiremock`-driven mock tests for [`DigitalOceanClient`]. No live calls.
//! Lives in `tests/` (separate compilation unit) to keep `src/client.rs`
//! under the 200-LOC Constructor Pattern limit.

use kei_compute_digitalocean::client::{CreateDropletSpec, DigitalOceanClient};
use kei_compute_digitalocean::Error;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sample_droplet_json(id: u64, status: &str) -> serde_json::Value {
    serde_json::json!({
        "droplet": {
            "id": id,
            "name": format!("kei-{id}"),
            "status": status,
            "region": {"slug": "fra1"},
            "networks": {
                "v4": [{"ip_address": "1.2.3.4", "type": "public"}],
                "v6": []
            },
            "created_at": "2026-04-28T00:00:00Z"
        }
    })
}

#[tokio::test]
async fn create_droplet_202_returns_droplet() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/droplets"))
        .and(header("authorization", "Bearer t-test"))
        .respond_with(ResponseTemplate::new(202).set_body_json(sample_droplet_json(42, "new")))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    let spec = CreateDropletSpec {
        name: "kei-42".into(),
        region: "fra1".into(),
        size: "s-1vcpu-1gb".into(),
        image: "ubuntu-24-04-x64".into(),
        ssh_keys: vec!["ssh-ed25519 AAA".into()],
        user_data: Some("#cloud-config".into()),
        tags: vec!["kei".into()],
    };
    let d = c.create_droplet(&spec).await.unwrap();
    assert_eq!(d.id, 42);
    assert_eq!(d.status, "new");
}

#[tokio::test]
async fn power_on_204_ok() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/droplets/42/actions"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    c.power_on(42).await.unwrap();
}

#[tokio::test]
async fn get_droplet_200_active_status_maps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/droplets/7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_droplet_json(7, "active")))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    let d = c.get_droplet(7).await.unwrap();
    assert_eq!(d.status, "active");
    assert_eq!(d.networks.v4.len(), 1);
    assert_eq!(d.networks.v4[0].ip_address, "1.2.3.4");
}

#[tokio::test]
async fn delete_droplet_204_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/droplets/99"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    c.delete(99).await.unwrap();
}

#[tokio::test]
async fn get_droplet_404_maps_to_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/droplets/404"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    let err = c.get_droplet(404).await.unwrap_err();
    assert!(matches!(err, Error::NotFound(_)));
}

#[tokio::test]
async fn list_droplets_200_returns_two() {
    let server = MockServer::start().await;
    let body = serde_json::json!({
        "droplets": [
            sample_droplet_json(1, "active")["droplet"],
            sample_droplet_json(2, "off")["droplet"]
        ]
    });
    Mock::given(method("GET"))
        .and(path("/droplets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;
    let c = DigitalOceanClient::new("t-test", server.uri()).unwrap();
    let v = c.list_droplets().await.unwrap();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].status, "active");
    assert_eq!(v[1].status, "off");
}
