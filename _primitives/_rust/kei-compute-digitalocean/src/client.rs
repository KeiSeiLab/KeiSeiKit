// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Thin async REST v2 client for DigitalOcean.
//!
//! No upstream Rust SDK is used — we hit the public surface directly
//! (`https://api.digitalocean.com/v2`) with bearer-token auth read from
//! `DIGITALOCEAN_TOKEN`. Base URL is overridable for `wiremock` tests.

use crate::error::{Error, Result};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Default REST root.
pub const DEFAULT_BASE_URL: &str = "https://api.digitalocean.com/v2";
/// Per-request timeout.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Spec passed to [`DigitalOceanClient::create_droplet`].
#[derive(Debug, Clone, Serialize)]
pub struct CreateDropletSpec {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Subset of the DigitalOcean droplet object we depend on.
#[derive(Debug, Clone, Deserialize)]
pub struct Droplet {
    pub id: u64,
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub networks: Networks,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub region: RegionRef,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Networks {
    #[serde(default)]
    pub v4: Vec<NetAddr>,
    #[serde(default)]
    pub v6: Vec<NetAddr>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetAddr {
    pub ip_address: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RegionRef {
    #[serde(default)]
    pub slug: String,
}

#[derive(Debug, Deserialize)]
struct DropletEnvelope {
    droplet: Droplet,
}

#[derive(Debug, Deserialize)]
struct DropletsEnvelope {
    droplets: Vec<Droplet>,
}

/// REST client. Cheap to clone (`Arc` inside `reqwest::Client`).
#[derive(Debug, Clone)]
pub struct DigitalOceanClient {
    http: Client,
    base_url: String,
    token: String,
}

impl DigitalOceanClient {
    /// Build with explicit token + base URL (use [`DEFAULT_BASE_URL`] in prod).
    pub fn new(token: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;
        Ok(Self { http, base_url: base_url.into(), token: token.into() })
    }

    /// Read `DIGITALOCEAN_TOKEN` from env, default base URL.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("DIGITALOCEAN_TOKEN").map_err(|_| {
            Error::Api("DIGITALOCEAN_TOKEN env var not set".into())
        })?;
        Self::new(token, DEFAULT_BASE_URL)
    }

    /// POST /droplets — returns the freshly-created droplet (status `new`).
    pub async fn create_droplet(&self, spec: &CreateDropletSpec) -> Result<Droplet> {
        let url = format!("{}/droplets", self.base_url);
        let resp = self.send(self.req(Method::POST, &url).json(spec)).await?;
        let env: DropletEnvelope = parse_json(resp).await?;
        Ok(env.droplet)
    }

    /// POST /droplets/{id}/actions — `power_on`. 201 expected.
    pub async fn power_on(&self, id: u64) -> Result<()> {
        self.action(id, "power_on").await
    }

    /// POST /droplets/{id}/actions — `shutdown`. 201 expected.
    pub async fn shutdown(&self, id: u64) -> Result<()> {
        self.action(id, "shutdown").await
    }

    /// DELETE /droplets/{id} — 204 expected.
    pub async fn delete(&self, id: u64) -> Result<()> {
        let url = format!("{}/droplets/{}", self.base_url, id);
        self.send(self.req(Method::DELETE, &url)).await.map(drop)
    }

    /// GET /droplets/{id} — `Error::NotFound` on 404.
    pub async fn get_droplet(&self, id: u64) -> Result<Droplet> {
        let url = format!("{}/droplets/{}", self.base_url, id);
        let resp = self.send(self.req(Method::GET, &url)).await?;
        let env: DropletEnvelope = parse_json(resp).await?;
        Ok(env.droplet)
    }

    /// GET /droplets — list all droplets the token can see.
    pub async fn list_droplets(&self) -> Result<Vec<Droplet>> {
        let url = format!("{}/droplets", self.base_url);
        let resp = self.send(self.req(Method::GET, &url)).await?;
        let env: DropletsEnvelope = parse_json(resp).await?;
        Ok(env.droplets)
    }

    async fn action(&self, id: u64, kind: &str) -> Result<()> {
        let url = format!("{}/droplets/{}/actions", self.base_url, id);
        let body = serde_json::json!({ "type": kind });
        self.send(self.req(Method::POST, &url).json(&body)).await.map(drop)
    }

    fn req(&self, method: Method, url: &str) -> RequestBuilder {
        self.http
            .request(method, url)
            .bearer_auth(&self.token)
            .header("accept", "application/json")
    }

    async fn send(&self, builder: RequestBuilder) -> Result<Response> {
        let resp = builder.send().await?;
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let body = resp.text().await.unwrap_or_default();
        Err(classify(status, body))
    }
}

fn classify(status: StatusCode, body: String) -> Error {
    if status.as_u16() == 404 {
        Error::NotFound(body)
    } else {
        Error::Api(format!("http {}: {}", status, body))
    }
}

async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
    let bytes = resp.bytes().await?;
    if bytes.is_empty() {
        return Err(Error::Api("empty body where JSON expected".into()));
    }
    Ok(serde_json::from_slice(&bytes)?)
}

