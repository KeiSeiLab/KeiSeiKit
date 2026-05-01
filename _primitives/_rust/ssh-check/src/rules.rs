//! Hardened SSH baseline — rule matrix. See block
//! `_blocks/security-ssh-hardening.md` for rationale per directive.

#[derive(Debug, Clone)]
pub enum Expect {
    /// Value must equal (case-insensitive) one of the given strings.
    OneOf(Vec<&'static str>),
    /// Value must equal the given string (case-insensitive).
    Equals(&'static str),
    /// Value must be a numeric literal ≤ given bound.
    MaxInt(u32),
    /// Value must contain ALL of the given tokens (comma-split, case-insensitive).
    ContainsAll(Vec<&'static str>),
    /// Value must NOT contain ANY of the given tokens.
    DeniesAny(Vec<&'static str>),
    /// Value must be present and non-empty; dynamic equality deferred to check.rs.
    AllowedUsersSubset(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub directive: &'static str,
    pub required: bool,
    pub expect: Expect,
    pub rationale: &'static str,
}

pub fn hardened_matrix(allow_users: &[String]) -> Vec<Rule> {
    vec![
        Rule {
            directive: "passwordauthentication",
            required: true,
            expect: Expect::Equals("no"),
            rationale: "Passwords are the #1 brute-force vector; keys only.",
        },
        Rule {
            directive: "permitrootlogin",
            required: true,
            expect: Expect::OneOf(vec!["no", "prohibit-password"]),
            rationale: "Root via key only (or not at all).",
        },
        Rule {
            directive: "permitemptypasswords",
            required: false,
            expect: Expect::Equals("no"),
            rationale: "Empty passwords never.",
        },
        Rule {
            directive: "challengeresponseauthentication",
            required: false,
            expect: Expect::Equals("no"),
            rationale: "Disables keyboard-interactive fallback.",
        },
        Rule {
            directive: "kbdinteractiveauthentication",
            required: false,
            expect: Expect::Equals("no"),
            rationale: "OpenSSH 8.7+ directive; supersedes ChallengeResponseAuthentication.",
        },
        Rule {
            directive: "maxauthtries",
            required: true,
            expect: Expect::MaxInt(3),
            rationale: "Limits per-connection key attempts; combine with fail2ban.",
        },
        Rule {
            directive: "x11forwarding",
            required: false,
            expect: Expect::Equals("no"),
            rationale: "Not needed on servers; attack surface.",
        },
        Rule {
            directive: "allowtcpforwarding",
            required: false,
            expect: Expect::OneOf(vec!["no", "local"]),
            rationale: "Blocks SSH-as-VPN; enable per Match block if needed.",
        },
        Rule {
            directive: "permittunnel",
            required: false,
            expect: Expect::Equals("no"),
            rationale: "Blocks tun(4) tunnel device.",
        },
        Rule {
            directive: "clientaliveinterval",
            required: false,
            expect: Expect::MaxInt(300),
            rationale: "Idle sessions terminated after a few minutes.",
        },
        Rule {
            directive: "loglevel",
            required: false,
            expect: Expect::OneOf(vec!["verbose", "debug1", "debug2", "debug3"]),
            rationale: "VERBOSE logs key fingerprints for audit.",
        },
        Rule {
            directive: "allowusers",
            required: true,
            expect: Expect::AllowedUsersSubset(allow_users.to_vec()),
            rationale: "Explicit admin whitelist.",
        },
        Rule {
            directive: "ciphers",
            required: false,
            expect: Expect::DeniesAny(vec![
                "aes128-cbc",
                "aes192-cbc",
                "aes256-cbc",
                "3des-cbc",
                "blowfish-cbc",
                "rijndael-cbc@lysator.liu.se",
            ]),
            rationale: "CBC ciphers vulnerable to Terrapin / padding oracles.",
        },
        Rule {
            directive: "macs",
            required: false,
            expect: Expect::ContainsAll(vec!["etm"]),
            rationale: "ETM (Encrypt-Then-MAC) only; legacy MAC is broken.",
        },
        Rule {
            directive: "hostkeyalgorithms",
            required: false,
            expect: Expect::DeniesAny(vec!["ssh-rsa", "ssh-dss"]),
            rationale: "ssh-rsa = SHA-1 signature, deprecated. Use rsa-sha2-*.",
        },
    ]
}
