// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Cloud-init renderer for Vultr instances. Same field surface as the
//! Hetzner sibling. Vultr's API requires the user_data payload be
//! base64-encoded — `render_base64()` returns the wire-ready form.

use serde::{Deserialize, Serialize};

/// Inputs to render a YAML cloud-init document for a managed VM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudInitSpec {
    pub user_handle: String,
    pub tailscale_auth_key: String,
    pub anthropic_api_key_env: String,
    pub git_remote_url: String,
    pub schedule_cron: String,
    pub install_forgejo_local: bool,
    pub control_plane_url: String,
}

impl CloudInitSpec {
    /// Render plain YAML. Whitespace-stable, deterministic for a given
    /// input — good for SHA-ing into the body of a DNA.
    pub fn render(&self) -> String {
        let forgejo_block = if self.install_forgejo_local {
            "  - curl -fsSL https://forgejo.example/install.sh | sh\n"
        } else {
            ""
        };
        format!(
            "#cloud-config\n\
write_files:\n\
  - path: /etc/keisei/agent.env\n\
    permissions: '0600'\n\
    content: |\n\
      USER_HANDLE={user}\n\
      ANTHROPIC_API_KEY=${env_key}\n\
      GIT_REMOTE_URL={git}\n\
      SCHEDULE_CRON='{cron}'\n\
      CONTROL_PLANE_URL={cp}\n\
runcmd:\n\
  - curl -fsSL https://tailscale.com/install.sh | sh\n\
  - tailscale up --authkey={ts}\n\
{fj}\
  - systemctl enable --now keisei-agent.service\n",
            user = self.user_handle,
            env_key = self.anthropic_api_key_env,
            git = self.git_remote_url,
            cron = self.schedule_cron,
            cp = self.control_plane_url,
            ts = self.tailscale_auth_key,
            fj = forgejo_block,
        )
    }

    /// Vultr v2 demands base64 in the `user_data` field. STANDARD alphabet
    /// without padding stripping.
    pub fn render_base64(&self) -> String {
        let yaml = self.render();
        base64_encode(yaml.as_bytes())
    }
}

/// Tiny self-contained STANDARD base64 encoder. Avoids pulling the
/// `base64` crate just for one call. RFC 4648 §4 alphabet, with padding.
pub fn encode_base64(input: &[u8]) -> String {
    base64_encode(input)
}

fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut i = 0;
    while i + 3 <= input.len() {
        let b0 = input[i] as u32;
        let b1 = input[i + 1] as u32;
        let b2 = input[i + 2] as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        out.push(ALPHABET[(n & 0x3F) as usize] as char);
        i += 3;
    }
    let rem = input.len() - i;
    if rem == 1 {
        let n = (input[i] as u32) << 16;
        out.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let n = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8);
        out.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        out.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
        out.push('=');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CloudInitSpec {
        CloudInitSpec {
            user_handle: "alice".into(),
            tailscale_auth_key: "tskey-abc".into(),
            anthropic_api_key_env: "ANTHROPIC_API_KEY".into(),
            git_remote_url: "git@forgejo:user/memory.git".into(),
            schedule_cron: "0 3 * * *".into(),
            install_forgejo_local: false,
            control_plane_url: "https://cp.example".into(),
        }
    }

    #[test]
    fn render_contains_required_directives() {
        let yaml = sample().render();
        assert!(yaml.starts_with("#cloud-config"));
        assert!(yaml.contains("USER_HANDLE=alice"));
        assert!(yaml.contains("tailscale up --authkey=tskey-abc"));
        assert!(yaml.contains("0 3 * * *"));
        assert!(!yaml.contains("forgejo/install.sh"));
    }

    #[test]
    fn render_base64_round_trip_chars_only() {
        let b64 = sample().render_base64();
        // Standard base64: A-Z a-z 0-9 + / =
        assert!(!b64.is_empty());
        assert!(b64
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='));
        // Padding aligned to 4
        assert_eq!(b64.len() % 4, 0);
    }
}
