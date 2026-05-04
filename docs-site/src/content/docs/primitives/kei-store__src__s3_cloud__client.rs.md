---
title: client.rs
path: kei-store/src/s3_cloud/client.rs
dna_hash: sha256:90feb2a54a95765d
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-store/src/s3_cloud/client.rs

aws-sdk-s3 client builder for the S3 cloud backend.

Wraps `aws_config::defaults()` + optional endpoint override for
R2 / MinIO / Wasabi / any S3-compat provider. Credential chain is the
AWS default — env vars, `~/.aws/credentials`, IMDS — we do NOT invent
a new credential format (RULE 0.8 secrets-single-source honoured).

Security invariants (v0.21.1):

* `validate_endpoint` rejects loopback / link-local / metadata URLs
unless `KEI_STORE_S3_ALLOW_INTERNAL=1` is set, and plain-HTTP
unless `KEI_STORE_S3_ALLOW_INSECURE=1` is set. Closes the SSRF /
IMDS-leak surface where an operator-controlled `KEI_STORE_S3_ENDPOINT`
pointed at `http://169.254.169.254` would cause the AWS default
credential chain to sign requests against the instance metadata
endpoint (and leak IMDS creds to the attacker's server).

* When the S3Cfg names `access_key_env` + `secret_key_env` env vars,
we build an explicit `Credentials` provider and overlay it on the
SDK builder. Without this wiring the two fields were silently dead
(critic HIGH-2); with it, a user pointing the backend at MinIO can
name a dedicated key pair via env instead of re-using the ambient
AWS chain.

* For NON-AWS endpoints (anything with a custom endpoint URL) we
REQUIRE explicit `access_key_env` + `secret_key_env`. Otherwise the
default credential chain (which includes IMDS) would still fire —
defeating the SSRF guard. Real-AWS paths (no endpoint override)
keep the default chain.

## Public API

- `pub fn effective_endpoint` — Resolve the effective endpoint URL:
- `pub fn validate_endpoint` — SSRF / IMDS-leak guard. Rejects unsafe endpoint URLs unless the caller
- Is this host a loopback, link-local, or known metadata endpoint?
- Resolve `access_key_env` + `secret_key_env` into a `Credentials` object
- Build the aws-sdk-s3 client with optional endpoint + region overrides.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, aws_credential_types, aws_sdk_s3, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
