// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! CardDAV auto-discovery via three successive PROPFIND requests.
//!
//! Implements RFC 6764 §6 well-known URI discovery.

use crate::error::ContactsError;
use regex::Regex;
use reqwest::{Client, Method};
use tracing::debug;

// ── XML bodies ────────────────────────────────────────────────────────────────

fn propfind_principal_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="utf-8"?>
<D:propfind xmlns:D="DAV:">
  <D:prop><D:current-user-principal/></D:prop>
</D:propfind>"#
}

fn propfind_home_set_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="utf-8"?>
<D:propfind xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
  <D:prop><C:addressbook-home-set/></D:prop>
</D:propfind>"#
}

fn propfind_resourcetype_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="utf-8"?>
<D:propfind xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
  <D:prop><D:resourcetype/></D:prop>
</D:propfind>"#
}

// ── XML helpers ───────────────────────────────────────────────────────────────

/// Extract the first `<D:href>` child of `tag` using regex.
///
/// Matches namespace-prefixed variants of `tag` (e.g. `D:current-user-principal`
/// or `C:addressbook-home-set`).
pub(crate) fn extract_first_href_under(xml: &str, tag: &str) -> Option<String> {
    let pattern = format!(
        r"(?si)<(?:[a-zA-Z0-9_-]+:)?{tag}[^>]*>\s*<(?:[a-zA-Z0-9_-]+:)?href[^>]*>([^<]+)</",
        tag = regex::escape(tag)
    );
    let re = Regex::new(&pattern).ok()?;
    re.captures(xml)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}

/// Extract the first `<D:href>` from a multistatus `<D:response>` whose
/// `<D:resourcetype>` contains `addressbook`.
pub(crate) fn extract_addressbook_href(xml: &str) -> Option<String> {
    // Split on <D:response> or <response> boundaries (case-insensitive).
    let re_split = Regex::new(r"(?si)<(?:[a-zA-Z0-9_-]+:)?response[^>]*>").ok()?;
    let boundaries: Vec<_> = re_split.find_iter(xml).map(|m| m.start()).collect();

    for (i, &start) in boundaries.iter().enumerate() {
        let end = boundaries.get(i + 1).copied().unwrap_or(xml.len());
        let chunk = &xml[start..end];

        if chunk.to_ascii_lowercase().contains("addressbook") {
            // Extract the href from this response chunk.
            let re_href =
                Regex::new(r"(?si)<(?:[a-zA-Z0-9_-]+:)?href[^>]*>([^<]+)</").ok()?;
            if let Some(cap) = re_href.captures(chunk) {
                if let Some(m) = cap.get(1) {
                    return Some(m.as_str().trim().to_string());
                }
            }
        }
    }
    None
}

// ── PROPFIND helper ───────────────────────────────────────────────────────────

async fn propfind(
    client: &Client,
    apple_id: &str,
    password: &str,
    url: &str,
    depth: &str,
    body: &'static str,
) -> Result<String, ContactsError> {
    debug!(%url, %depth, "PROPFIND");
    let resp = client
        .request(
            Method::from_bytes(b"PROPFIND")
                .map_err(|e| ContactsError::Http(e.to_string()))?,
            url,
        )
        .basic_auth(apple_id, Some(password))
        .header("Content-Type", "application/xml; charset=utf-8")
        .header("Depth", depth)
        .body(body)
        .send()
        .await
        .map_err(|e| ContactsError::Http(e.to_string()))?;

    let status = resp.status();
    if status.as_u16() == 401 || status.as_u16() == 403 {
        return Err(ContactsError::Auth(format!("iCloud returned {}", status.as_u16())));
    }
    if !status.is_success() && status.as_u16() != 207 {
        return Err(ContactsError::Http(format!("PROPFIND status={}", status)));
    }
    resp.text()
        .await
        .map_err(|e| ContactsError::Parse(e.to_string()))
}

// ── public entry point ────────────────────────────────────────────────────────

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn discover_walks_three_propfinds() {
        let server = MockServer::start().await;

        Mock::given(method("PROPFIND"))
            .and(path("/.well-known/carddav"))
            .respond_with(ResponseTemplate::new(207).set_body_string(
                r#"<D:multistatus xmlns:D="DAV:"><D:response><D:propstat>
  <D:current-user-principal><D:href>/principals/users/testuser/</D:href></D:current-user-principal>
</D:propstat></D:response></D:multistatus>"#,
            ))
            .mount(&server)
            .await;

        Mock::given(method("PROPFIND"))
            .and(path("/principals/users/testuser/"))
            .respond_with(ResponseTemplate::new(207).set_body_string(
                r#"<D:multistatus xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav"><D:response><D:propstat>
  <C:addressbook-home-set><D:href>/addressbooks/testuser/</D:href></C:addressbook-home-set>
</D:propstat></D:response></D:multistatus>"#,
            ))
            .mount(&server)
            .await;

        Mock::given(method("PROPFIND"))
            .and(path("/addressbooks/testuser/"))
            .respond_with(ResponseTemplate::new(207).set_body_string(
                r#"<D:multistatus xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
  <D:response>
    <D:href>/addressbooks/testuser/card/</D:href>
    <D:propstat><D:resourcetype><D:collection/><C:addressbook/></D:resourcetype></D:propstat>
  </D:response>
</D:multistatus>"#,
            ))
            .mount(&server)
            .await;

        let client = Client::new();
        let url = discover_addressbook(&client, "user@icloud.com", "app-pass", &server.uri())
            .await
            .expect("discovery should succeed");
        assert_eq!(url, "/addressbooks/testuser/card/");
    }
}

/// Perform CardDAV three-step auto-discovery.
///
/// 1. PROPFIND `/.well-known/carddav` → `current-user-principal`
/// 2. PROPFIND `{principal}` → `addressbook-home-set`
/// 3. PROPFIND `{home-set}` (depth=1) → first `addressbook` resource href
pub(crate) async fn discover_addressbook(
    client: &Client,
    apple_id: &str,
    password: &str,
    base_url: &str,
) -> Result<String, ContactsError> {
    // Step 1: principal
    let url1 = format!("{}/.well-known/carddav", base_url);
    let xml1 = propfind(client, apple_id, password, &url1, "0", propfind_principal_xml()).await?;
    let principal = extract_first_href_under(&xml1, "current-user-principal")
        .ok_or_else(|| ContactsError::Parse("discover step 1: no current-user-principal".into()))?;

    // Step 2: home set
    let url2 = format!("{}{}", base_url, principal);
    let xml2 = propfind(client, apple_id, password, &url2, "0", propfind_home_set_xml()).await?;
    let home_set = extract_first_href_under(&xml2, "addressbook-home-set")
        .ok_or_else(|| ContactsError::Parse("discover step 2: no addressbook-home-set".into()))?;

    // Step 3: addressbook resource
    let url3 = format!("{}{}", base_url, home_set);
    let xml3 =
        propfind(client, apple_id, password, &url3, "1", propfind_resourcetype_xml()).await?;
    extract_addressbook_href(&xml3)
        .ok_or_else(|| ContactsError::Parse("discover step 3: no addressbook resource".into()))
}
