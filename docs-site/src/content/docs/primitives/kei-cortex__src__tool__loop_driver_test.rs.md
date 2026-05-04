---
title: loop_driver_test.rs
path: kei-cortex/src/tool/loop_driver_test.rs
dna_hash: sha256:87bcd773caf6eddf
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-cortex/src/tool/loop_driver_test.rs

Inline unit tests for `loop_driver.rs`. Extracted to a sibling so
the parent stays under the 200-LOC Constructor Pattern ceiling after
the Wave 44c CancellationToken refactor (F-HIGH-5).

## Public API

- F-HIGH-5: cancel token fires mid-turn and the loop short-circuits

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, futures, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
