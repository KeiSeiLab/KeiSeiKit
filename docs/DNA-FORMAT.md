# DNA-FORMAT — Portable Specification

> How to parse and compute DNA strings without compiling any Rust.
> SSoT: `_primitives/_rust/kei-shared/src/dna.rs` + `kei-runtime-core/src/dna.rs` (2026-05-02).

---

## Section 1 — Wire format

```
<role>::<caps>::<scope_sha8>::<body_sha8>-<nonce8>
```

Example:

```
vm-managed::HZ-CX22-NB::A1B2C3D4::DEADBEEF-c0ffee01
```

### Segment table

| Segment | Separator | Length | Character class | Semantics |
|---------|-----------|--------|-----------------|-----------|
| `role` | `::` prefix | 1+ chars | Any non-empty string | Agent role slug (e.g. `vm-managed`, `code-implementer`) |
| `caps` | `::` | 1+ chars | Any non-empty, tags joined by `-` | Capability tags (e.g. `HZ-CX22-NB`, `EM`) |
| `scope_sha8` | `::` | exactly 8 | ASCII hex `[0-9A-Fa-f]` | First 4 bytes of SHA-256 of scope input |
| `body_sha8` | none | exactly 8 | ASCII hex | First 4 bytes of SHA-256 of body input |
| `-` | literal `-` | 1 | `-` | Separates body_sha8 from nonce |
| `nonce8` | end of string | exactly 8 | ASCII hex `[0-9a-f]` | Random 4 bytes, lowercase, per-spawn |

Split rule: four `::` segments → `parts[0..3]`; `parts[3]` splits on last `-` into `body_sha8` and `nonce8`.

Total minimum wire length: `1 + 2 + 1 + 2 + 8 + 2 + 8 + 1 + 8 = 33` chars.

---

## Section 2 — Computing scope_sha8

`scope_sha8` is the first 8 uppercase hex chars of `SHA-256(scope_input)`.

`scope_input` is arbitrary bytes representing "what task class is this" — typically a canonical URL, manifest path, or task description. The exact content is caller-defined; the only contract is that the same bytes always yield the same hash.

### Worked example (Python)

```python
import hashlib

scope_input = b"keiseikit.dev/vms/hetzner/nbg1"
digest = hashlib.sha256(scope_input).digest()
scope_sha8 = digest[:4].hex().upper()   # first 4 bytes → 8 hex chars
print(scope_sha8)  # e.g. "3F7A2C11"
```

### Worked example (shell)

```sh
echo -n "keiseikit.dev/vms/hetzner/nbg1" \
  | sha256sum \
  | cut -c1-8 \
  | tr 'a-f' 'A-F'
# prints e.g. "3F7A2C11"
```

Note: `sha256sum` outputs lowercase; `tr` uppercases to match Rust's `format!("{:02X}")`.

---

## Section 3 — Computing body_sha8

Identical algorithm to scope_sha8, applied to the body input bytes.

`body_input` represents "what is the substrate configuration" — typically a JSON manifest body, config struct, or similar content-addressable blob.

```python
body_input = b'{"tier":"cx22","cloud_init_sha":"abc"}'
body_sha8 = hashlib.sha256(body_input).digest()[:4].hex().upper()
```

---

## Section 4 — Nonce

`nonce8` is 4 random bytes formatted as 8 lowercase hex chars.

It is generated fresh on every agent spawn. It is NOT cryptographic — its sole purpose is to distinguish concurrent spawns of the same task class from each other in the ledger.

```python
import secrets
nonce8 = secrets.token_bytes(4).hex()   # always lowercase, 8 chars
```

```sh
openssl rand -hex 4   # produces e.g. "c0ffee01"
```

The Rust source (`kei-runtime-core/src/dna.rs::random_hex8_lower`) uses `rand::thread_rng().fill_bytes`.

---

## Section 5 — Parsing in pure shell and Python

### Shell (awk one-liner)

```sh
DNA="vm-managed::HZ-CX22-NB::A1B2C3D4::DEADBEEF-c0ffee01"

echo "$DNA" | awk -F'::' '{
  role=$1; caps=$2; scope_sha=$3
  n=split($4, tail, "-")
  nonce=tail[n]; body_sha=""
  for(i=1;i<n;i++) body_sha=(i==1?tail[i]:body_sha"-"tail[i])
  print "role="role, "caps="caps, "scope="scope_sha, "body="body_sha, "nonce="nonce
}'
# role=vm-managed caps=HZ-CX22-NB scope=A1B2C3D4 body=DEADBEEF nonce=c0ffee01
```

### Python (regex)

```python
import re

DNA_RE = re.compile(
    r'^(?P<role>[^:]+)::(?P<caps>[^:]+)::(?P<scope_sha>[0-9A-Fa-f]{8})'
    r'::(?P<body_sha>[0-9A-Fa-f]{8})-(?P<nonce>[0-9A-Fa-f]{8})$'
)

def parse_dna(s: str) -> dict:
    m = DNA_RE.match(s)
    if not m:
        raise ValueError(f"invalid DNA: {s!r}")
    return m.groupdict()

dna = parse_dna("vm-managed::HZ-CX22-NB::A1B2C3D4::DEADBEEF-c0ffee01")
# {'role': 'vm-managed', 'caps': 'HZ-CX22-NB',
#  'scope_sha': 'A1B2C3D4', 'body_sha': 'DEADBEEF', 'nonce': 'c0ffee01'}
```

---

## Section 6 — Same-task-class property

`scope_sha8` and `body_sha8` are deterministic: identical inputs always produce the same values. Only the nonce changes per spawn.

This means the **task_class_dna** (DNA with the `-<nonce8>` suffix stripped) is a stable per-task-class identifier:

```
task_class_dna = DNA[: -9]    # strip trailing "-xxxxxxxx"
# e.g. "vm-managed::HZ-CX22-NB::A1B2C3D4::DEADBEEF"
```

The ledger (v9+) stores this as a VIRTUAL generated column `task_class_dna` for empirical posterior aggregation. Two spawns of the same task class will share the same prefix and differ only in nonce.

```sql
-- Find all spawns of a given task class across time:
SELECT id, started_ts, outcome
FROM agents
WHERE task_class_dna = 'vm-managed::HZ-CX22-NB::A1B2C3D4::DEADBEEF'
ORDER BY started_ts DESC;
```
