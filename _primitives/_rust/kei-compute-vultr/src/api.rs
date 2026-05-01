// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Thin async client for the Vultr Cloud v2 API.
//!
//! All requests carry `Authorization: Bearer $VULTR_API_KEY`. The wire
//! types here track the Vultr v2 schema (instances are wrapped in an
//! `instance` envelope on single-resource responses).

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE: &str = "https://api.vultr.com/v2";

/// HTTP client for the Vultr v2 API.
#[derive(Debug, Clone)]
pub struct VultrClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

impl VultrClient {
    /// Build a client. `token` should be the value of `VULTR_API_KEY`.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: DEFAULT_BASE.to_string(),
            token: token.into(),
        }
    }

    /// Override the API base — used by tests with `wiremock`.
    pub fn with_base_url(mut self, base: impl Into<String>) -> Self {
        self.base_url = base.into();
        self
    }

    pub async fn create_instance(
        &self,
        req: &CreateInstanceRequest,
    ) -> Result<InstanceResponse> {
        let url = format!("{}/instances", self.base_url);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(req)
            .send()
            .await?;
        decode(resp).await
    }

    pub async fn get_instance(&self, id: &str) -> Result<InstanceResponse> {
        let url = format!("{}/instances/{}", self.base_url, id);
        let resp = self.http.get(&url).bearer_auth(&self.token).send().await?;
        decode(resp).await
    }

    pub async fn delete_instance(&self, id: &str) -> Result<()> {
        let url = format!("{}/instances/{}", self.base_url, id);
        let resp = self
            .http
            .delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        decode_void(resp).await
    }

    pub async fn halt_instance(&self, id: &str) -> Result<()> {
        let url = format!("{}/instances/{}/halt", self.base_url, id);
        let resp = self.http.post(&url).bearer_auth(&self.token).send().await?;
        decode_void(resp).await
    }

    pub async fn start_instance(&self, id: &str) -> Result<()> {
        let url = format!("{}/instances/{}/start", self.base_url, id);
        let resp = self.http.post(&url).bearer_auth(&self.token).send().await?;
        decode_void(resp).await
    }

    pub async fn change_plan(&self, id: &str, plan: &str) -> Result<InstanceResponse> {
        let url = format!("{}/instances/{}", self.base_url, id);
        let body = serde_json::json!({ "plan": plan });
        let resp = self
            .http
            .patch(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?;
        decode(resp).await
    }
}

async fn decode(resp: reqwest::Response) -> Result<InstanceResponse> {
    let status = resp.status();
    let body = resp.text().await?;
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::Http {
            status: 404,
            body,
        });
    }
    if !status.is_success() {
        return Err(Error::Http {
            status: status.as_u16(),
            body,
        });
    }
    Ok(serde_json::from_str(&body)?)
}

async fn decode_void(resp: reqwest::Response) -> Result<()> {
    let status = resp.status();
    if status.is_success() {
        return Ok(());
    }
    let body = resp.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::Http {
            status: 404,
            body,
        });
    }
    Err(Error::Http {
        status: status.as_u16(),
        body,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateInstanceRequest {
    pub region: String,
    pub plan: String,
    pub label: String,
    pub hostname: String,
    /// One of `os_id` or `iso_id` is required by Vultr; we expose both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_id: Option<String>,
    /// Vultr requires user_data be base64-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sshkey_id: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstanceResponse {
    pub instance: VultrInstance,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VultrInstance {
    pub id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub power_status: String,
    #[serde(default)]
    pub main_ip: String,
    #[serde(default)]
    pub v6_main_ip: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub plan: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub hostname: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn create_instance_round_trip() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "instance": {
                "id": "abcd-1234",
                "status": "pending",
                "power_status": "running",
                "main_ip": "0.0.0.0",
                "v6_main_ip": "",
                "region": "ams",
                "plan": "vc2-1c-1gb",
                "label": "test-vm",
                "hostname": "test-vm"
            }
        });
        Mock::given(method("POST"))
            .and(path("/instances"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(202).set_body_json(&body))
            .mount(&server)
            .await;

        let client = VultrClient::new("test-token").with_base_url(server.uri());
        let req = CreateInstanceRequest {
            region: "ams".into(),
            plan: "vc2-1c-1gb".into(),
            label: "test-vm".into(),
            hostname: "test-vm".into(),
            os_id: Some(2136),
            iso_id: None,
            user_data: Some("Zm9v".into()),
            sshkey_id: vec!["k1".into()],
            tags: vec!["project=kei".into()],
        };
        let r = client.create_instance(&req).await.expect("create ok");
        assert_eq!(r.instance.id, "abcd-1234");
        assert_eq!(r.instance.status, "pending");
        assert_eq!(r.instance.plan, "vc2-1c-1gb");
    }

    #[tokio::test]
    async fn get_instance_404_maps_to_http_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/instances/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let client = VultrClient::new("t").with_base_url(server.uri());
        let err = client.get_instance("missing").await.unwrap_err();
        match err {
            Error::Http { status, .. } => assert_eq!(status, 404),
            other => panic!("expected Http 404, got {other:?}"),
        }
    }
}
