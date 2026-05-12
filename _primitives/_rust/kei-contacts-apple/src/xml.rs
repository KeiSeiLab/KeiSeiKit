// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! XML helpers for CardDAV multistatus responses.

use crate::contact::AppleContact;
use crate::error::ContactsError;
use crate::vcard::parse_vcard;
use regex::Regex;
use tracing::debug;

/// Build the CardDAV `addressbook-query` REPORT XML body.
pub(crate) fn addressbook_query_xml() -> String {
    r#"<?xml version="1.0" encoding="utf-8"?>
<C:addressbook-query xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
  <D:prop><D:getetag/><C:address-data/></D:prop>
  <C:filter/>
</C:addressbook-query>"#
    .to_string()
}

/// Extract embedded vCards from a CardDAV multistatus XML response.
///
/// Parsing is done with a simple regex to avoid pulling in a full XML crate.
pub(crate) fn extract_contacts_from_multistatus(
    xml: &str,
) -> Result<Vec<AppleContact>, ContactsError> {
    let re = Regex::new(r"(?si)<[^:>]*:?address-data[^>]*>(.*?)</[^:>]*:?address-data>")
        .map_err(|e| ContactsError::Parse(e.to_string()))?;

    let mut contacts = Vec::new();
    for cap in re.captures_iter(xml) {
        let raw = cap.get(1).map(|m| m.as_str()).unwrap_or("").trim();
        if raw.is_empty() {
            continue;
        }
        match parse_vcard(raw) {
            Ok(c) => contacts.push(c),
            Err(e) => {
                debug!("skipping unparseable vCard: {e}");
            }
        }
    }

    Ok(contacts)
}
