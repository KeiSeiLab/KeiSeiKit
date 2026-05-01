// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Cloud-init renderer for Linode. Linode's `metadata.user_data` field
//! requires the user-data blob to be **base64-encoded**, so we expose
//! both the raw render (`render`) and the API-ready encoded form
//! (`render_base64`).

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;

/// Minimal cloud-init template: hostname, ssh authorized_keys, runcmd.
#[derive(Debug, Clone)]
pub struct CloudInitTemplate {
    pub hostname: String,
    pub ssh_pubkey: String,
    pub run_cmds: Vec<String>,
}

impl CloudInitTemplate {
    pub fn new(hostname: impl Into<String>, ssh_pubkey: impl Into<String>) -> Self {
        Self {
            hostname: hostname.into(),
            ssh_pubkey: ssh_pubkey.into(),
            run_cmds: Vec::new(),
        }
    }

    pub fn run(mut self, cmd: impl Into<String>) -> Self {
        self.run_cmds.push(cmd.into());
        self
    }
}

/// Render the cloud-init YAML body. Deterministic — no I/O, no time.
pub fn render(t: &CloudInitTemplate) -> String {
    let mut out = String::from("#cloud-config\n");
    out.push_str(&format!("hostname: {}\n", t.hostname));
    out.push_str("ssh_authorized_keys:\n");
    out.push_str(&format!("  - {}\n", t.ssh_pubkey));
    if !t.run_cmds.is_empty() {
        out.push_str("runcmd:\n");
        for c in &t.run_cmds {
            // Quote with single quotes; cloud-init YAML accepts plain
            // strings but quoting is robust against `:` in commands.
            let escaped = c.replace('\'', "''");
            out.push_str(&format!("  - '{escaped}'\n"));
        }
    }
    out
}

/// Render and base64-encode for `metadata.user_data`.
pub fn render_base64(t: &CloudInitTemplate) -> String {
    STANDARD.encode(render(t).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_includes_required_sections() {
        let t = CloudInitTemplate::new("kei-vm-1", "ssh-ed25519 AAAA test")
            .run("apt-get update")
            .run("curl https://tailscale.com/install.sh | sh");
        let s = render(&t);
        assert!(s.starts_with("#cloud-config\n"));
        assert!(s.contains("hostname: kei-vm-1"));
        assert!(s.contains("ssh-ed25519 AAAA test"));
        assert!(s.contains("apt-get update"));
        assert!(s.contains("tailscale.com/install.sh"));
    }

    #[test]
    fn base64_is_decodable_to_render() {
        let t = CloudInitTemplate::new("h", "ssh-ed25519 K");
        let b64 = render_base64(&t);
        let decoded = STANDARD.decode(b64.as_bytes()).expect("valid base64");
        let text = String::from_utf8(decoded).expect("utf8");
        assert_eq!(text, render(&t));
    }
}
