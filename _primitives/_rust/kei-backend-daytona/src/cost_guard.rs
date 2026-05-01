//! Free-tier cost guard for the Daytona backend.
//!
//! Daytona's free tier covers **2 concurrent sandboxes** with **30-min idle
//! hibernate**. Anything past that is paid. Before any `create_sandbox`
//! call, `pre_create_check` lists existing sandboxes and counts the ones
//! that consume quota (`Running | Hibernated | Stopped | Pending`). If the
//! count is at or above `cap`, the call is blocked with a structured
//! error — kei-cost-guardian will eventually consume this signal directly.

use crate::client::DaytonaClient;
use crate::error::DaytonaError;
use crate::types::SandboxState;
use std::fmt;

/// Daytona free-tier concurrent-sandbox cap.
pub const FREE_TIER_CAP: usize = 2;

/// Error returned when a creation would exceed the configured cap.
#[derive(Debug, Clone)]
pub struct CostGuardError {
    /// Number of sandboxes currently consuming quota.
    pub current: usize,
    /// Cap the call would have crossed.
    pub cap: usize,
}

impl fmt::Display for CostGuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "daytona cost guard: would create sandbox at quota \
             (current={}, cap={}); release one first",
            self.current, self.cap
        )
    }
}

impl std::error::Error for CostGuardError {}

impl From<CostGuardError> for DaytonaError {
    fn from(e: CostGuardError) -> Self {
        DaytonaError::Unknown(e.to_string())
    }
}

/// True if `state` consumes a quota slot on the Daytona side.
fn consumes_quota(state: SandboxState) -> bool {
    matches!(
        state,
        SandboxState::Running
            | SandboxState::Hibernated
            | SandboxState::Stopped
            | SandboxState::Pending
    )
}

/// Block a sandbox creation when `count(quota-consuming sandboxes) >= cap`.
///
/// Returns `Ok(())` when there is at least one free slot. Returns
/// `Err(CostGuardError)` when the cap has been reached. Network / parse
/// failures from `list_sandboxes` are surfaced as `DaytonaError` and
/// converted into `CostGuardError::Unknown`-shaped output by the caller —
/// the guard itself does not silently allow creation when the listing
/// failed.
pub async fn pre_create_check(
    client: &DaytonaClient,
    cap: usize,
) -> Result<(), CostGuardError> {
    let list = match client.list_sandboxes().await {
        Ok(v) => v,
        Err(_) => {
            // Fail-closed: if we can't enumerate, treat as if at cap.
            return Err(CostGuardError { current: cap, cap });
        }
    };
    let current = list.iter().filter(|sb| consumes_quota(sb.state)).count();
    if current >= cap {
        return Err(CostGuardError { current, cap });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_quota_matches_active_states() {
        assert!(consumes_quota(SandboxState::Running));
        assert!(consumes_quota(SandboxState::Hibernated));
        assert!(consumes_quota(SandboxState::Stopped));
        assert!(consumes_quota(SandboxState::Pending));
        assert!(!consumes_quota(SandboxState::Error));
        assert!(!consumes_quota(SandboxState::Unknown));
    }

    #[test]
    fn cost_guard_error_display_mentions_cap() {
        let e = CostGuardError { current: 2, cap: 2 };
        let s = format!("{e}");
        assert!(s.contains("cap=2"));
        assert!(s.contains("current=2"));
    }

    #[test]
    fn cost_guard_error_converts_to_daytona_error() {
        let e = CostGuardError { current: 3, cap: 2 };
        let d: DaytonaError = e.into();
        let s = format!("{d}");
        assert!(s.contains("cost guard"));
    }

    #[test]
    fn free_tier_cap_is_two() {
        assert_eq!(FREE_TIER_CAP, 2);
    }
}
