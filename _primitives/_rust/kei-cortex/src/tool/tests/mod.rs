//! Integration-style tests for the tool substrate. Each sibling file is
//! one focused scenario; they share no fixtures so they can be debugged
//! in isolation.

mod bash_sandbox_denies;
mod edit_unique_old_string;
mod loop_terminates_on_max_turns;
mod registry_dispatch;
