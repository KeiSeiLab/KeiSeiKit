// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Integration tests for `GooglePeopleClient` against a wiremock server.

use kei_contacts_google::{ContactsError, GooglePeopleClient};
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SAMPLE_RESPONSE: &str = r#"{
    "connections": [
        {
            "resourceName": "people/c111",
            "names": [{"displayName": "Alice Smith", "givenName": "Alice", "familyName": "Smith"}],
            "emailAddresses": [{"value": "alice@example.com"}],
            "phoneNumbers": [{"value": "+1-555-0101"}],
            "organizations": [{"name": "ACME"}],
            "biographies": [{"value": "Engineer"}]
        },
        {
            "resourceName": "people/c222",
            "names": [{"displayName": "Bob Jones", "givenName": "Bob", "familyName": "Jones"}],
            "emailAddresses": [{"value": "bob@example.com"}, {"value": "bob2@example.com"}],
            "phoneNumbers": [],
            "organizations": [],
            "biographies": []
        }
    ],
    "nextPageToken": "tok123"
}"#;

#[tokio::test]
async fn list_connections_parses_real_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/people/me/connections"))
        .and(header_exists("Authorization"))
        .respond_with(ResponseTemplate::new(200).set_body_string(SAMPLE_RESPONSE))
        .mount(&server)
        .await;

    let client = GooglePeopleClient::new("fake-token".to_string())
        .with_base_url(server.uri());
    let contacts = client.list_connections().await.expect("should succeed");

    assert_eq!(contacts.len(), 2);
    let alice = &contacts[0];
    assert_eq!(alice.resource_name, "people/c111");
    assert_eq!(alice.display_name, "Alice Smith");
    assert_eq!(alice.emails, vec!["alice@example.com"]);
    assert_eq!(alice.phones, vec!["+1-555-0101"]);
    assert_eq!(alice.organization, "ACME");
    assert_eq!(alice.bio, "Engineer");

    let bob = &contacts[1];
    assert_eq!(bob.emails.len(), 2);
    assert_eq!(bob.phones.len(), 0);
    assert_eq!(bob.organization, "");
}

#[tokio::test]
async fn auth_error_on_401() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/people/me/connections"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let client = GooglePeopleClient::new("expired-token".to_string())
        .with_base_url(server.uri());
    let err = client.list_connections().await.expect_err("should fail");
    assert!(matches!(err, ContactsError::Auth(_)));
}
