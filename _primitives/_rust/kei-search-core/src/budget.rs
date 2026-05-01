//! Budget tracker — all costs in microcents (1 USD = 1_000_000 mc).

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Budget {
    cap_mc: i64,
    spent_mc: i64,
    stopped: bool,
}

impl Budget {
    pub fn new(cap_mc: i64) -> Self {
        Self { cap_mc, spent_mc: 0, stopped: false }
    }

    /// Record a cost; returns error if this push would exceed the cap.
    pub fn charge(&mut self, mc: i64) -> Result<()> {
        if self.stopped {
            return Err(anyhow!("budget stopped"));
        }
        if self.spent_mc + mc > self.cap_mc {
            return Err(anyhow!(
                "budget exceeded: spent={} cap={}", self.spent_mc + mc, self.cap_mc));
        }
        self.spent_mc += mc;
        Ok(())
    }

    pub fn spent(&self) -> i64 { self.spent_mc }
    pub fn remaining(&self) -> i64 { self.cap_mc - self.spent_mc }
    pub fn stop(&mut self) { self.stopped = true; }
    pub fn is_stopped(&self) -> bool { self.stopped }
}
