## No dependency bumps

You MUST NOT add, remove, or upgrade dependencies. Specifically:

- Do NOT edit the `[dependencies]`, `[dev-dependencies]`,
  `[build-dependencies]`, or `[workspace.dependencies]` sections of
  any `Cargo.toml`
- Do NOT write or regenerate `Cargo.lock`
- Do NOT `cargo add`, `cargo remove`, or `cargo update`

Each new or upgraded dependency expands the supply-chain attack
surface and can trigger breaking-change cascades across the
workspace. Dependency decisions require a separate review, a
dedicated task, and an orchestrator-approved lock diff.

Editing other sections of `Cargo.toml` (e.g. `[package]`,
`[features]`, `[[bin]]`, `[lib]`, `[package.metadata.*]`) is allowed
if the file is in your whitelist and not in your denylist. The gate
inspects the specific region of the diff.

If your task genuinely requires a new dependency, STOP. Describe the
crate, version, and reason in your return. The orchestrator will
decide whether to re-spawn you with an opt-in flag or handle the
dep-bump through a separate review.

On return, the verifier diffs `Cargo.lock` against main; any change
rejects the return.
