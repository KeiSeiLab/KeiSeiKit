// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! [`ICloudCardDavClient`] — CardDAV client for iCloud Contacts.

use crate::contact::AppleContact;
use crate::discovery::discover_addressbook;
use crate::error::ContactsError;
use crate::xml::{addressbook_query_xml, extract_contacts_from_multistatus};
use reqwest::{Client, Method};
use tracing::debug;

const DEFAULT_BASE_URL: &str = "https://contacts.icloud.com";

/// CardDAV client for iCloud Contacts.
///
/// # Authentication
/// iCloud requires an **app-specific password** (not the main Apple ID password
/// and not Sign in with Apple). Generate one at <https://appleid.apple.com>.
///
/// # Discovery
/// Full CardDAV discovery (PROPFIND `.well-known/carddav`) is complex. For the
/// MVP, supply the addressbook URL directly via [`with_addressbook_url`].
///
/// [`with_addressbook_url`]: ICloudCardDavClient::with_addressbook_url
pub struct ICloudCardDavClient {
    apple_id: String,
    app_specific_password: String,
    base_url: String,
    addressbook_url: Option<String>,
    client: Client,
}

impl ICloudCardDavClient {
    /// Construct a client with the given credentials and the production base URL.
    pub fn new(apple_id: String, app_specific_password: String) -> Self {
        Self {
            apple_id,
            app_specific_password,
            base_url: DEFAULT_BASE_URL.to_string(),
            addressbook_url: None,
            client: Client::new(),
        }
    }

    /// Override the base URL (useful for wiremock tests).
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Set the full addressbook URL, skipping CardDAV discovery.
    ///
    /// Example (iCloud): `https://p01-contacts.icloud.com/123456789/carddavhome/card/`
    pub fn with_addressbook_url(mut self, url: String) -> Self {
        self.addressbook_url = Some(url);
        self
    }

    /// Discover the addressbook URL via three successive PROPFIND requests.
    ///
    /// Implements RFC 6764 §6:
    /// 1. `.well-known/carddav` → principal URL
    /// 2. principal → addressbook-home-set
    /// 3. home-set (depth=1) → first addressbook resource href
    pub async fn discover_addressbook_url(&self) -> Result<String, ContactsError> {
        discover_addressbook(
            &self.client,
            &self.apple_id,
            &self.app_specific_password,
            &self.base_url,
        )
        .await
    }

    /// Fetch all contacts from the configured addressbook.
    ///
    /// Issues a CardDAV REPORT `addressbook-query` and returns parsed contacts.
    pub async fn list_contacts(&self) -> Result<Vec<AppleContact>, ContactsError> {
        let url = self
            .addressbook_url
            .clone()
            .unwrap_or_else(|| self.base_url.clone());

        debug!(%url, "REPORT addressbook-query");

        let resp = self
            .client
            .request(
                Method::from_bytes(b"REPORT")
                    .map_err(|e| ContactsError::Http(e.to_string()))?,
                &url,
            )
            .basic_auth(&self.apple_id, Some(&self.app_specific_password))
            .header("Content-Type", "application/xml; charset=utf-8")
            .header("Depth", "1")
            .body(addressbook_query_xml())
            .send()
            .await
            .map_err(|e| ContactsError::Http(e.to_string()))?;

        let status = resp.status();
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err(ContactsError::Auth(format!(
                "iCloud returned {}",
                status.as_u16()
            )));
        }
        if !status.is_success() && status.as_u16() != 207 {
            return Err(ContactsError::Http(format!("status={}", status)));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| ContactsError::Parse(e.to_string()))?;

        extract_contacts_from_multistatus(&text)
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn two_contacts_xml() -> String {
        let vc1 = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Alice Smith\r\nUID:uid-alice\r\nEMAIL:alice@example.com\r\nEND:VCARD";
        let vc2 = "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Bob Jones\r\nUID:uid-bob\r\nEMAIL:bob@example.com\r\nEND:VCARD";
        format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<D:multistatus xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
  <D:response>
    <D:propstat><C:address-data>{vc1}</C:address-data></D:propstat>
  </D:response>
  <D:response>
    <D:propstat><C:address-data>{vc2}</C:address-data></D:propstat>
  </D:response>
</D:multistatus>"#
        )
    }

    #[tokio::test]
    async fn list_contacts_parses_carddav_xml() {
        let server = MockServer::start().await;
        Mock::given(method("REPORT"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(207).set_body_string(two_contacts_xml()))
            .mount(&server)
            .await;

        let client = ICloudCardDavClient::new(
            "user@icloud.com".to_string(),
            "app-pass".to_string(),
        )
        .with_base_url(server.uri());

        let contacts = client.list_contacts().await.expect("should succeed");
        assert_eq!(contacts.len(), 2);
        let names: Vec<_> = contacts.iter().map(|c| c.display_name.as_str()).collect();
        assert!(names.contains(&"Alice Smith"));
        assert!(names.contains(&"Bob Jones"));
    }

    #[tokio::test]
    async fn auth_401_maps_to_auth_error() {
        let server = MockServer::start().await;
        Mock::given(method("REPORT"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = ICloudCardDavClient::new(
            "user@icloud.com".to_string(),
            "wrong-pass".to_string(),
        )
        .with_base_url(server.uri());

        let err = client.list_contacts().await.expect_err("should fail");
        assert!(matches!(err, ContactsError::Auth(_)));
    }
}
