// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Minimal vCard 3.0 / 4.0 parser.
//!
//! # Limitations (MVP)
//! - **Line-folding** (continuation lines starting with a single space or tab)
//!   is **not** handled. Such lines are treated as separate malformed lines and
//!   silently skipped. This covers the vast majority of iCloud-generated vCards.
//! - Only a fixed set of properties (FN, N, EMAIL, TEL, ORG, NOTE, UID) is extracted.
//! - Property parameters (e.g. `TYPE=INTERNET`) are stripped; only the value is kept.
//! - Multi-valued ORG (e.g. `ORG:Company;Department`) uses the first segment.

use crate::contact::AppleContact;
use crate::error::ContactsError;

/// Parse a single vCard text into an [`AppleContact`].
///
/// `text` must be the content of one vCard (between `BEGIN:VCARD` and `END:VCARD`
/// inclusive).
pub fn parse_vcard(text: &str) -> Result<AppleContact, ContactsError> {
    let mut contact = AppleContact {
        raw_vcard: text.to_string(),
        ..Default::default()
    };

    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        // Skip continuation lines (line-folding, not handled in MVP).
        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        let Some((key_full, value)) = line.split_once(':') else {
            continue;
        };
        // Strip parameters: key_full may be "EMAIL;TYPE=INTERNET" → key = "EMAIL"
        let key = key_full
            .split(';')
            .next()
            .unwrap_or(key_full)
            .to_ascii_uppercase();
        let value = value.trim();
        apply_property(&key, value, &mut contact);
    }

    if contact.uid.is_empty() && contact.display_name.is_empty() {
        return Err(ContactsError::InvalidVCard(
            "no UID or FN found in vCard".to_string(),
        ));
    }

    Ok(contact)
}

fn apply_property(key: &str, value: &str, c: &mut AppleContact) {
    match key {
        "FN" => c.display_name = value.to_string(),
        "UID" => c.uid = value.to_string(),
        "NOTE" => c.note = value.to_string(),
        "EMAIL" => {
            if !value.is_empty() {
                c.emails.push(value.to_string());
            }
        }
        "TEL" => {
            if !value.is_empty() {
                c.phones.push(value.to_string());
            }
        }
        "N" => parse_n(value, c),
        "ORG" => {
            // ORG may be "Company;Department;..." — take first segment.
            let org = value.split(';').next().unwrap_or(value);
            if !org.is_empty() {
                c.organization = org.to_string();
            }
        }
        _ => {}
    }
}

/// Parse vCard N property: `family;given;additional;prefix;suffix`
fn parse_n(value: &str, c: &mut AppleContact) {
    let mut parts = value.splitn(5, ';');
    c.family_name = parts.next().unwrap_or("").to_string();
    c.given_name = parts.next().unwrap_or("").to_string();
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_VCARD: &str = "\
BEGIN:VCARD\r\n\
VERSION:3.0\r\n\
FN:Denis Parfionovich\r\n\
N:Parfionovich;Denis;;;\r\n\
EMAIL;TYPE=INTERNET:denis@example.com\r\n\
ORG:KeiSei Labs\r\n\
NOTE:hand-written note\r\n\
UID:abc-123\r\n\
END:VCARD\r\n";

    #[test]
    fn parse_simple_vcard() {
        let c = parse_vcard(SIMPLE_VCARD).expect("should parse");
        assert_eq!(c.display_name, "Denis Parfionovich");
        assert_eq!(c.given_name, "Denis");
        assert_eq!(c.family_name, "Parfionovich");
        assert_eq!(c.emails, vec!["denis@example.com"]);
        assert_eq!(c.organization, "KeiSei Labs");
        assert_eq!(c.note, "hand-written note");
        assert_eq!(c.uid, "abc-123");
    }

    #[test]
    fn parse_multi_email_vcard() {
        let text = "\
BEGIN:VCARD\r\n\
VERSION:3.0\r\n\
FN:Alice Smith\r\n\
UID:uid-alice\r\n\
EMAIL;TYPE=INTERNET:alice@work.com\r\n\
EMAIL;TYPE=HOME:alice@home.com\r\n\
TEL;TYPE=CELL:+1234567890\r\n\
END:VCARD\r\n";
        let c = parse_vcard(text).expect("should parse");
        assert_eq!(c.emails.len(), 2);
        assert!(c.emails.contains(&"alice@work.com".to_string()));
        assert!(c.emails.contains(&"alice@home.com".to_string()));
        assert_eq!(c.phones, vec!["+1234567890"]);
    }

    #[test]
    fn parse_invalid_vcard_returns_error() {
        let text = "NOTACARD:yes\r\n";
        assert!(parse_vcard(text).is_err());
    }
}
