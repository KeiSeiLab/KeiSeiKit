//! Textual rules for `injection_check`.
//!
//! Constructor Pattern: prompt-override, ChatML, system-prefix, secret
//! markers, exfil substrings — all lower-case substring tests on the
//! input. No regex; no allocations beyond a single `to_lowercase`.

use crate::injection_check::InjectionFinding;

/// Detect prompt-override / role-prefix / ChatML payloads.
pub(crate) fn scan_prompt_override(content: &str) -> Option<InjectionFinding> {
    let lower = content.to_lowercase();
    scan_override_phrases(&lower)
        .or_else(|| scan_role_prefix(&lower))
        .or_else(|| scan_chatml_tags(&lower))
}

fn scan_override_phrases(lower: &str) -> Option<InjectionFinding> {
    if contains_phrase(lower, &["ignore", "previous", "instructions"]) {
        return Some(InjectionFinding {
            pattern: "prompt_override_ignore_previous",
            source: "promptguard:override",
        });
    }
    if lower.contains("you are now") {
        return Some(InjectionFinding {
            pattern: "prompt_override_you_are_now",
            source: "promptguard:roleplay",
        });
    }
    if lower.contains("disregard all")
        || lower.contains("disregard prior")
        || lower.contains("disregard above")
    {
        return Some(InjectionFinding {
            pattern: "prompt_override_disregard",
            source: "promptguard:override",
        });
    }
    None
}

fn scan_role_prefix(lower: &str) -> Option<InjectionFinding> {
    if has_leading_system_prefix(lower) {
        return Some(InjectionFinding {
            pattern: "system_role_prefix",
            source: "promptguard:role-prefix",
        });
    }
    None
}

fn scan_chatml_tags(lower: &str) -> Option<InjectionFinding> {
    if lower.contains("<|im_start|>") {
        return Some(InjectionFinding {
            pattern: "chatml_im_start",
            source: "chatml:tag",
        });
    }
    if lower.contains("<|endoftext|>") {
        return Some(InjectionFinding {
            pattern: "chatml_endoftext",
            source: "chatml:tag",
        });
    }
    None
}

/// Detect PEM private-key markers.
pub(crate) fn scan_secrets(content: &str) -> Option<InjectionFinding> {
    let dashes = "-".repeat(5);
    let openssh = format!("{dashes}BEGIN OPENSSH PRIVATE KEY{dashes}");
    let rsa = format!("{dashes}BEGIN RSA PRIVATE KEY{dashes}");
    if content.contains(&openssh) {
        return Some(InjectionFinding {
            pattern: "ssh_openssh_private",
            source: "secret:openssh",
        });
    }
    if content.contains(&rsa) {
        return Some(InjectionFinding {
            pattern: "ssh_rsa_private",
            source: "secret:rsa",
        });
    }
    None
}

/// Detect exfiltration shapes (bearer-token + URL, api_key + URL,
/// raw `aws_secret` keyword).
pub(crate) fn scan_exfil(content: &str) -> Option<InjectionFinding> {
    let lower = content.to_lowercase();
    if lower.contains("aws_secret") {
        return Some(InjectionFinding {
            pattern: "aws_secret_keyword",
            source: "secret:aws",
        });
    }
    if lower.contains("bearer ") && lower.contains("://") {
        return Some(InjectionFinding {
            pattern: "curl_with_bearer",
            source: "exfil:curl-bearer",
        });
    }
    if lower.contains("api_key=") && lower.contains("://") {
        return Some(InjectionFinding {
            pattern: "api_key_url",
            source: "exfil:api-key-url",
        });
    }
    None
}

/// True if any line, after trimming leading whitespace, starts with `system:`.
fn has_leading_system_prefix(lower: &str) -> bool {
    lower.lines().any(|line| line.trim_start().starts_with("system:"))
}

/// True if `hay` contains all needles in order.
fn contains_phrase(hay: &str, needles: &[&str]) -> bool {
    let mut search = hay;
    for n in needles {
        match search.find(n) {
            Some(off) => search = &search[off + n.len()..],
            None => return false,
        }
    }
    true
}
