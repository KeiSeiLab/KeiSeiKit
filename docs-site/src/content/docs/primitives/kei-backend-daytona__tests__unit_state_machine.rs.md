---
title: unit_state_machine.rs
path: kei-backend-daytona/tests/unit_state_machine.rs
dna_hash: sha256:8bc5d1d08cca0bd0
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-backend-daytona/tests/unit_state_machine.rs

State-machine tests for `DaytonaBackend::acquire`.

Each test stands up a wiremock server, configures the responses Daytona
would emit for one of the resume-or-create branches, and asserts that
the right HTTP calls were made.

## Related

- parent: `kei-backend-daytona/tests`
- imports: kei_backend_daytona, serde_json, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
