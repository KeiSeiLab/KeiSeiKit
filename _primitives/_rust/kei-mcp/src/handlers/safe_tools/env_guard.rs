//! Subprocess environment + process-group hardening for kei_* tools.
//!
//! v0.46: extracted from monolithic safe_tools.rs.

use tokio::process::Command;

/// v0.41 fix #5: process-group helper (Unix-only; no-op on other platforms).
/// tokio::process::Command::process_group is available on Unix without
/// requiring the std::os::unix::process::CommandExt trait import.
#[cfg(unix)]
pub fn set_process_group(cmd: &mut Command) {
    cmd.process_group(0);
}
#[cfg(not(unix))]
pub fn set_process_group(_cmd: &mut Command) {}

/// v0.41 fix #5: SIGKILL the entire process group (negative pid).
#[cfg(unix)]
pub fn killpg_best_effort(pid: u32) {
    unsafe {
        let _ = libc::kill(-(pid as i32), libc::SIGKILL);
    }
}
#[cfg(not(unix))]
pub fn killpg_best_effort(_pid: u32) {}

/// v0.46 architectural fix: RAII guard. `kill_on_drop` only kills the
/// immediate child; backgrounded grandchildren survive (e.g. `bash -c
/// 'sleep 1000 &'`). v0.41 killpg fix only ran on the timeout error path.
/// Now: killpg fires on EVERY exit path (success, error, panic, early return)
/// via Drop. Caller disarms on clean wait_with_output success via `disarm()`.
pub struct KillPgGuard {
    pid: Option<u32>,
}

impl KillPgGuard {
    pub fn new(pid: Option<u32>) -> Self { Self { pid } }
    /// Caller succeeded cleanly; child is already reaped by wait_with_output.
    /// Skip the killpg fire on Drop.
    pub fn disarm(&mut self) { self.pid = None; }
}

impl Drop for KillPgGuard {
    fn drop(&mut self) {
        if let Some(pid) = self.pid {
            killpg_best_effort(pid);
        }
    }
}

/// Names an operator MUST NOT forward into a subprocess even via the
/// `KEI_SAFE_ENV_EXTRA` override. These vars hijack the dynamic loader
/// or interpreter startup and let an attacker inject code into every
/// child the safe-tools spawn — Stack-Overflow-copy-paste-style. Every
/// hit here is logged AND skipped.
///
/// Sources: ld.so(8), dyld(1), node --help, ruby(1), python(1).
const BANNED_FORWARD: &[&str] = &[
    // GNU/Linux dynamic linker
    "LD_PRELOAD",
    "LD_LIBRARY_PATH",
    "LD_AUDIT",
    // macOS dynamic linker
    "DYLD_INSERT_LIBRARIES",
    "DYLD_LIBRARY_PATH",
    "DYLD_FRAMEWORK_PATH",
    // Node, Python, Ruby interpreter-side equivalents
    "NODE_OPTIONS",
    "PYTHONSTARTUP",
    "PYTHONPATH",
    "RUBYOPT",
];

/// True iff `name` matches a banned forward (case-sensitive — POSIX env
/// names are case-sensitive on every supported platform). Exposed for
/// unit tests.
pub fn is_banned_forward(name: &str) -> bool {
    BANNED_FORWARD.iter().any(|b| *b == name)
}

/// v0.44 fix #4 (Gemini HIGH): strip parent env on subprocess spawn so secrets
/// like AWS_*, GITHUB_TOKEN, MOONSHOT_API_KEY etc. don't leak to user-controlled
/// bash commands or hook scripts. Whitelist forwards only PATH/HOME/USER/LANG/
/// TERM/SHELL/PWD/TMPDIR/LOGNAME/LC_* — enough to keep tools functional, none
/// of it sensitive.
///
/// Override: `KEI_SAFE_ENV_EXTRA=":-separated list"` adds named vars to the
/// whitelist for callers that legitimately need (e.g. NIX_PATH, JAVA_HOME).
/// The override is filtered by [`BANNED_FORWARD`] — loader-injection vars
/// (LD_PRELOAD, DYLD_INSERT_LIBRARIES, NODE_OPTIONS, ...) are refused even
/// when the operator names them explicitly. Attempts are logged.
pub fn apply_safe_env(cmd: &mut Command) {
    cmd.env_clear();
    let default_keep = [
        "PATH", "HOME", "USER", "LOGNAME", "SHELL", "LANG", "LC_ALL",
        "LC_CTYPE", "TERM", "PWD", "TMPDIR",
    ];
    for k in default_keep {
        if let Ok(v) = std::env::var(k) {
            cmd.env(k, v);
        }
    }
    if let Ok(extras) = std::env::var("KEI_SAFE_ENV_EXTRA") {
        for k in extras.split(':') {
            let k = k.trim();
            if k.is_empty() { continue; }
            if is_banned_forward(k) {
                eprintln!(
                    "kei-mcp safe_tools: refusing to forward `{k}` via \
                     KEI_SAFE_ENV_EXTRA (loader-injection class)"
                );
                continue;
            }
            if let Ok(v) = std::env::var(k) {
                cmd.env(k, v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banned_list_covers_loader_vars() {
        for v in [
            "LD_PRELOAD",
            "LD_LIBRARY_PATH",
            "LD_AUDIT",
            "DYLD_INSERT_LIBRARIES",
            "DYLD_LIBRARY_PATH",
            "DYLD_FRAMEWORK_PATH",
            "NODE_OPTIONS",
            "PYTHONSTARTUP",
            "PYTHONPATH",
            "RUBYOPT",
        ] {
            assert!(is_banned_forward(v), "{v} must be banned");
        }
    }

    #[test]
    fn banned_check_is_case_sensitive() {
        // POSIX env names are case-sensitive; "ld_preload" is a different
        // variable from "LD_PRELOAD" and not handled by ld.so. We mirror that.
        assert!(!is_banned_forward("ld_preload"));
        assert!(!is_banned_forward("Ld_Preload"));
    }

    #[test]
    fn benign_names_not_banned() {
        for v in ["PATH", "HOME", "JAVA_HOME", "NIX_PATH", "GOPATH"] {
            assert!(!is_banned_forward(v));
        }
    }
}
