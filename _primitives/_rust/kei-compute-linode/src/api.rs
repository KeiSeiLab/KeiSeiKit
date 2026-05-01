// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Linode v4 REST API client. Thin wrapper over `reqwest::Client` —
//! one method per provider verb. Wire types live alongside.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://api.linode.com/v4";

/// Linode HTTP client. Holds bearer token + base URL (overridable for tests).
#[derive(Debug, Clone)]
pub struct LinodeClient {
    base_url: String,
    token: String,
    http: reqwest::Client,
}

impl LinodeClient {
    /// Construct from explicit token. For prod, prefer
    /// `LinodeClient::from_env()` which reads `LINODE_TOKEN` (RULE 0.8).
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            token: token.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Read `LINODE_TOKEN` from environment.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("LINODE_TOKEN").map_err(|_| {
            Error::Auth("LINODE_TOKEN not set; source ~/.claude/secrets/.env".into())
        })?;
        Ok(Self::new(token))
    }

    /// Override the base URL (test injection).
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// `POST /linode/instances` — create instance.
    pub async fn create_instance(
        &self,
        req: &CreateInstanceRequest,
    ) -> Result<InstanceResponse> {
        let url = format!("{}/linode/instances", self.base_url);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(req)
            .send()
            .await?;
        decode(resp).await
    }

    /// `GET /linode/instances/{id}` — read instance.
    pub async fn get_instance(&self, id: i64) -> Result<InstanceResponse> {
        let url = format!("{}/linode/instances/{id}", self.base_url);
        let resp = self.http.get(&url).bearer_auth(&self.token).send().await?;
        decode(resp).await
    }

    /// `DELETE /linode/instances/{id}` — destroy.
    pub async fn delete_instance(&self, id: i64) -> Result<()> {
        let url = format!("{}/linode/instances/{id}", self.base_url);
        let resp = self.http.delete(&url).bearer_auth(&self.token).send().await?;
        ok_no_body(resp).await
    }

    /// `POST /linode/instances/{id}/boot`
    pub async fn boot(&self, id: i64) -> Result<()> {
        let url = format!("{}/linode/instances/{id}/boot", self.base_url);
        let resp = self.http.post(&url).bearer_auth(&self.token).send().await?;
        ok_no_body(resp).await
    }

    /// `POST /linode/instances/{id}/shutdown`
    pub async fn shutdown(&self, id: i64) -> Result<()> {
        let url = format!("{}/linode/instances/{id}/shutdown", self.base_url);
        let resp = self.http.post(&url).bearer_auth(&self.token).send().await?;
        ok_no_body(resp).await
    }

    /// `POST /linode/instances/{id}/resize` — change tier slug.
    pub async fn resize(&self, id: i64, new_type: &str) -> Result<()> {
        let url = format!("{}/linode/instances/{id}/resize", self.base_url);
        let body = serde_json::json!({ "type": new_type });
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?;
        ok_no_body(resp).await
    }
}

async fn decode<T: for<'de> Deserialize<'de>>(resp: reqwest::Response) -> Result<T> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(Error::Api {
            status: status.as_u16(),
            body,
        });
    }
    let bytes = resp.bytes().await?;
    Ok(serde_json::from_slice(&bytes)?)
}

async fn ok_no_body(resp: reqwest::Response) -> Result<()> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(Error::Api {
            status: status.as_u16(),
            body,
        });
    }
    Ok(())
}

// ---- Wire types ----

/// `POST /linode/instances` body. `metadata.user_data` carries the
/// base64-encoded cloud-init blob (see `cloud_init::render_base64`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub label: String,
    pub region: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_pass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackscript_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InstanceMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceMetadata {
    /// Base64-encoded cloud-init user-data.
    pub user_data: String,
}

/// `GET /linode/instances/{id}` response (subset we use).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceResponse {
    pub id: i64,
    pub label: String,
    pub status: String,
    #[serde(default)]
    pub ipv4: Vec<String>,
    pub ipv6: Option<String>,
    pub region: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{bearer_token, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn create_instance_round_trip() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "id": 12345,
            "label": "kei-test",
            "status": "provisioning",
            "ipv4": ["192.0.2.10"],
            "ipv6": "2001:db8::1/128",
            "region": "us-east",
            "type": "g6-nanode-1"
        });
        Mock::given(method("POST"))
            .and(path("/linode/instances"))
            .and(bearer_token("tkn"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let cli = LinodeClient::new("tkn").with_base_url(server.uri());
        let req = CreateInstanceRequest {
            label: "kei-test".into(),
            region: "us-east".into(),
            type_: "g6-nanode-1".into(),
            image: "linode/debian12".into(),
            root_pass: None,
            authorized_keys: Some(vec!["ssh-ed25519 AAAA…".into()]),
            stackscript_data: None,
            metadata: Some(InstanceMetadata {
                user_data: "I2Nsb3VkLWNvbmZpZw==".into(),
            }),
            tags: None,
        };
        let resp = cli.create_instance(&req).await.expect("ok");
        assert_eq!(resp.id, 12345);
        assert_eq!(resp.status, "provisioning");
        assert_eq!(resp.type_, "g6-nanode-1");
        assert_eq!(resp.ipv4, vec!["192.0.2.10".to_string()]);
    }

    #[tokio::test]
    async fn get_instance_404_maps_to_api_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/linode/instances/999"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let cli = LinodeClient::new("tkn").with_base_url(server.uri());
        let err = cli.get_instance(999).await.unwrap_err();
        match err {
            Error::Api { status, .. } => assert_eq!(status, 404),
            other => panic!("expected Api error, got {other:?}"),
        }
    }
}
