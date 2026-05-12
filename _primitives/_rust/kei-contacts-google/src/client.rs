// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! [`GooglePeopleClient`] — thin HTTP wrapper around Google People API v1.

use crate::contact::GoogleContact;
use crate::error::ContactsError;
use crate::pagination::{fetch_all_pages, fetch_page};
use reqwest::Client;

const DEFAULT_BASE_URL: &str = "https://people.googleapis.com";

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

    /// Fetch the first page of contacts (≤ 200).
    ///
    /// Back-compat API — use [`list_all_connections`] for full pagination.
    ///
    /// [`list_all_connections`]: GooglePeopleClient::list_all_connections
    pub async fn list_connections(&self) -> Result<Vec<GoogleContact>, ContactsError> {
        let (contacts, _) =
            fetch_page(&self.client, &self.access_token, &self.base_url, None).await?;
        Ok(contacts)
    }

    /// Fetch ALL contacts across all pages.
    ///
    /// Loops on `nextPageToken` until none is returned. Hard cap at 50 pages
    /// (~10 000 contacts) — if hit, returns what was collected and logs a warning.
    pub async fn list_all_connections(&self) -> Result<Vec<GoogleContact>, ContactsError> {
        fetch_all_pages(&self.client, &self.access_token, &self.base_url).await
    }
}
