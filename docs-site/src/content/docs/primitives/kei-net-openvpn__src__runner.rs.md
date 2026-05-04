---
title: runner.rs
path: kei-net-openvpn/src/runner.rs
dna_hash: sha256:68519d325572f47f
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-net-openvpn/src/runner.rs

[`Runner`] — minimal sync abstraction over a process invocation. The
`OpenvpnMode` impl holds a `Arc<dyn Runner + Send + Sync>` so tests
can substitute a mock recorder without touching `systemctl`. Real
deployments use [`SystemRunner`] which shells out via
`std::process::Command`.

The trait is intentionally synchronous — `systemctl start/stop` is
a sub-second blocking call and we do NOT want to drag a Tokio
runtime through the runner abstraction. The async `NetworkMode`
method wraps the call in `tokio::task::spawn_blocking` if the
caller is on a multi-thread runtime; for the smoke tests we call
it directly inline.

## Public API

- Invoke `program` with `args`. Returns the captured outcome.
- Real backend: `std::process::Command` shell-out. Used in production.

## Related

- parent: `kei-net-openvpn/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
