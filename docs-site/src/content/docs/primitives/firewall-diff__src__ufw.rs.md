---
title: ufw.rs
path: firewall-diff/src/ufw.rs
dna_hash: sha256:defb48a329821930
language: rust
size_loc: 173
generated: by-keidocs
---

# firewall-diff/src/ufw.rs

Parse `ufw status numbered` output.

Typical shape (Ubuntu 22.04, ufw 0.36):

Status: active

To                         Action      From
--                         ------      ----
[ 1] 22/tcp                     LIMIT IN    Anywhere
[ 2] 443/tcp                    ALLOW IN    Anywhere
[ 3] 22/tcp (v6)                LIMIT IN    Anywhere (v6)

We normalise "(v6)" to a separate family tag but key rules on port/proto
only (v6 and v4 rules with the same port/proto are treated as duplicates
of intent, which is usually the desired behaviour for parity checks).

## Public API

- Parse one numbered rule line. Returns None if the line is not a rule.

## Related

- parent: `firewall-diff/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
