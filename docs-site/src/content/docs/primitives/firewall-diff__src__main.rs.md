---
title: main.rs
path: firewall-diff/src/main.rs
dna_hash: sha256:52e9564f79fc288b
language: rust
size_loc: 101
generated: by-keidocs
---

# firewall-diff/src/main.rs

firewall-diff — compare an intended ufw rule set (YAML) against the
running firewall (parsed from `ufw status numbered` output).

USAGE
firewall-diff --intent firewall-intent.yaml --status-file live.txt
ufw status numbered | firewall-diff --intent firewall-intent.yaml --stdin
firewall-diff --intent firewall-intent.yaml --json

The tool does NOT execute `ufw` itself (defensive-only). Feed it the
output of `ufw status numbered` or have the skill pipe it in.

EXIT
0  intent ≡ live (no diff)
1  usage / parse error
2  differences found (live deviates from intent)

## Public API

- Path to the intent YAML file.
- Path to a file holding captured `ufw status numbered` output.
- Read the ufw status text from stdin (use when piping from the host).
- Emit JSON instead of human text.

## Related

- parent: `firewall-diff/Cargo.toml`
- imports: clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
