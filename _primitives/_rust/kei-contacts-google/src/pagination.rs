// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Pagination helper for Google People API connections.

use crate::contact::GoogleContact;
use crate::error::ContactsError;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, warn};

const PERSON_FIELDS: &str = "names,emailAddresses,phoneNumbers,organizations,biographies";
const PAGE_SIZE: u32 = 200;
/// Safety cap: at most 50 pages (10 000 contacts).
const MAX_PAGES: usize = 50;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionsResponse {
    pub connections: Option<Vec<Connection>>,
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Connection {
    pub resource_name: Option<String>,
    pub names: Option<Vec<Name>>,
    pub email_addresses: Option<Vec<EmailAddress>>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub organizations: Option<Vec<OrgEntry>>,
    pub biographies: Option<Vec<Biography>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Name {
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct EmailAddress {
    pub value: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct PhoneNumber {
    pub value: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct OrgEntry {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Biography {
    pub value: Option<String>,
}

/// Fetch one page of connections.
///
/// Returns `(contacts, next_page_token)`.
pub(crate) async fn fetch_page(
    client: &Client,
    access_token: &str,
    base_url: &str,
    page_token: Option<&str>,
) -> Result<(Vec<GoogleContact>, Option<String>), ContactsError> {
    let mut url = format!(
        "{}/v1/people/me/connections?personFields={}&pageSize={}",
        base_url, PERSON_FIELDS, PAGE_SIZE
    );
    if let Some(tok) = page_token {
        url.push_str(&format!("&pageToken={}", tok));
    }
    debug!(%url, "GET people/me/connections");

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
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

    Ok((contacts, body.next_page_token))
}

/// Fetch ALL pages accumulating contacts, stopping after [`MAX_PAGES`].
pub(crate) async fn fetch_all_pages(
    client: &Client,
    access_token: &str,
    base_url: &str,
) -> Result<Vec<GoogleContact>, ContactsError> {
    let mut all: Vec<GoogleContact> = Vec::new();
    let mut next_token: Option<String> = None;

    for page in 0..MAX_PAGES {
        let (contacts, token) =
            fetch_page(client, access_token, base_url, next_token.as_deref()).await?;
        all.extend(contacts);
        next_token = token;
        if next_token.is_none() {
            break;
        }
        if page == MAX_PAGES - 1 {
            warn!(
                "hit {MAX_PAGES}-page safety cap; returning {} contacts so far",
                all.len()
            );
        }
    }

    Ok(all)
}

pub(crate) fn parse_connection(c: Connection) -> GoogleContact {
    let resource_name = c.resource_name.unwrap_or_default();

    let (display_name, given_name, family_name) = c
        .names
        .and_then(|mut v| if v.is_empty() { None } else { Some(v.remove(0)) })
        .map(|n| {
            (
                n.display_name.unwrap_or_default(),
                n.given_name.unwrap_or_default(),
                n.family_name.unwrap_or_default(),
            )
        })
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

