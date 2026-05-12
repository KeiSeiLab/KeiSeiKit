// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! [`GooglePeopleClient`] — thin HTTP wrapper around Google People API v1.

use crate::contact::GoogleContact;
use crate::error::ContactsError;
use reqwest::Client;
use serde::Deserialize;
use tracing::debug;

const DEFAULT_BASE_URL: &str = "https://people.googleapis.com";
const PERSON_FIELDS: &str =
    "names,emailAddresses,phoneNumbers,organizations,biographies";
const PAGE_SIZE: u32 = 200;

/// Thin client for the Google People API.
///
/// Expects a valid OAuth2 access token. Does NOT perform OAuth itself;
/// obtain the token from `kei-auth-google` or similar.
pub struct GooglePeopleClient {
    access_token: String,
    base_url: String,
    client: Client,
}

impl GooglePeopleClient {
    /// Construct a client with the given access token and the production base URL.
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            base_url: DEFAULT_BASE_URL.to_string(),
            client: Client::new(),
        }
    }

    /// Override the base URL (useful for wiremock tests).
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Fetch the authenticated user's contacts (first page only, ≤ 200).
    ///
    /// # TODO
    /// Pagination via `nextPageToken` is not yet implemented. For users
    /// with > 200 contacts only the first 200 are returned.
    pub async fn list_connections(&self) -> Result<Vec<GoogleContact>, ContactsError> {
        let url = format!(
            "{}/v1/people/me/connections?personFields={}&pageSize={}",
            self.base_url, PERSON_FIELDS, PAGE_SIZE
        );
        debug!(%url, "GET people/me/connections");

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await
            .map_err(|e| ContactsError::Http(e.to_string()))?;

        let status = resp.status();
        if status.as_u16() == 401 {
            return Err(ContactsError::Auth("token expired or invalid".to_string()));
        }
        if !status.is_success() {
            return Err(ContactsError::Http(format!("status={}", status)));
        }

        let body: ConnectionsResponse = resp
            .json()
            .await
            .map_err(|e| ContactsError::Parse(e.to_string()))?;

        let contacts = body
            .connections
            .unwrap_or_default()
            .into_iter()
            .map(parse_connection)
            .collect();

        Ok(contacts)
    }
}

// ── internal deserialization types ───────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectionsResponse {
    connections: Option<Vec<Connection>>,
    // next_page_token intentionally ignored (TODO: pagination)
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Connection {
    resource_name: Option<String>,
    names: Option<Vec<Name>>,
    email_addresses: Option<Vec<EmailAddress>>,
    phone_numbers: Option<Vec<PhoneNumber>>,
    organizations: Option<Vec<OrgEntry>>,
    biographies: Option<Vec<Biography>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    display_name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
}

#[derive(Deserialize)]
struct EmailAddress {
    value: Option<String>,
}

#[derive(Deserialize)]
struct PhoneNumber {
    value: Option<String>,
}

#[derive(Deserialize)]
struct OrgEntry {
    name: Option<String>,
}

#[derive(Deserialize)]
struct Biography {
    value: Option<String>,
}

fn parse_connection(c: Connection) -> GoogleContact {
    let resource_name = c.resource_name.unwrap_or_default();

    let (display_name, given_name, family_name) = c
        .names
        .and_then(|mut v| if v.is_empty() { None } else { Some(v.remove(0)) })
        .map(|n| (
            n.display_name.unwrap_or_default(),
            n.given_name.unwrap_or_default(),
            n.family_name.unwrap_or_default(),
        ))
        .unwrap_or_default();

    let emails = c
        .email_addresses
        .unwrap_or_default()
        .into_iter()
        .filter_map(|e| e.value)
        .collect();

    let phones = c
        .phone_numbers
        .unwrap_or_default()
        .into_iter()
        .filter_map(|p| p.value)
        .collect();

    let organization = c
        .organizations
        .and_then(|mut v| v.first_mut().and_then(|o| o.name.take()))
        .unwrap_or_default();

    let bio = c
        .biographies
        .and_then(|mut v| v.first_mut().and_then(|b| b.value.take()))
        .unwrap_or_default();

    GoogleContact {
        resource_name,
        display_name,
        given_name,
        family_name,
        emails,
        phones,
        organization,
        bio,
    }
}
