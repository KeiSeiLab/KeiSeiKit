use kei_ledger_sign::{
    canonical_message, generate_keypair, load_keypair, save_keypair, sign_row, verify_row, Error,
};
use tempfile::tempdir;

#[test]
fn keygen_produces_pair() {
    let kp = generate_keypair();
    let pub_bytes = kp.verifying().to_bytes();
    assert_eq!(pub_bytes.len(), 32);
    assert!(pub_bytes.iter().any(|&b| b != 0), "pubkey must be nonzero");
}

#[test]
fn sign_verify_round_trip() {
    let kp = generate_keypair();
    let sig = sign_row(&kp, "dna-xyz", "sha-abc", "creator-1").unwrap();
    verify_row(&kp.verifying(), "dna-xyz", "sha-abc", "creator-1", &sig)
        .expect("valid signature must verify");
}

#[test]
fn verify_rejects_wrong_pubkey() {
    let kp1 = generate_keypair();
    let kp2 = generate_keypair();
    let sig = sign_row(&kp1, "dna-xyz", "sha-abc", "creator-1").unwrap();
    let err = verify_row(&kp2.verifying(), "dna-xyz", "sha-abc", "creator-1", &sig)
        .expect_err("wrong pubkey must fail");
    match err {
        Error::Signature(_) => {}
        other => panic!("expected Error::Signature, got {:?}", other),
    }
}

#[test]
fn verify_rejects_tampered_dna() {
    let kp = generate_keypair();
    let sig = sign_row(&kp, "dna-xyz", "sha-abc", "creator-1").unwrap();
    let err = verify_row(&kp.verifying(), "dna-TAMPERED", "sha-abc", "creator-1", &sig)
        .expect_err("tampered dna must fail");
    match err {
        Error::Signature(_) => {}
        other => panic!("expected Error::Signature, got {:?}", other),
    }
}

#[test]
fn save_load_keypair_preserves_signing() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("keys.json");
    let kp1 = generate_keypair();
    save_keypair(&kp1, &path).unwrap();
    let kp2 = load_keypair(&path).unwrap();
    assert_eq!(
        kp1.verifying().to_bytes(),
        kp2.verifying().to_bytes(),
        "loaded pubkey must match"
    );
    let sig = sign_row(&kp2, "dna-xyz", "sha-abc", "creator-1").unwrap();
    verify_row(&kp1.verifying(), "dna-xyz", "sha-abc", "creator-1", &sig)
        .expect("signature by loaded key must verify against original pubkey");
}

#[cfg(unix)]
#[test]
fn save_keypair_sets_600_perms() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempdir().unwrap();
    let path = dir.path().join("keys.json");
    let kp = generate_keypair();
    save_keypair(&kp, &path).unwrap();
    let meta = std::fs::metadata(&path).unwrap();
    let mode = meta.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "expected 0o600, got {:o}", mode);
}

#[cfg(unix)]
#[test]
fn save_keypair_atomic_no_race_window() {
    // Regression: save_keypair MUST NOT leave an intermediate world-readable
    // file between write and chmod. With the rename-into-place fix, the
    // final file is mode 0o600 from the first byte and the <path>.tmp
    // sidecar is cleaned up by rename(2).
    use std::os::unix::fs::PermissionsExt;
    let dir = tempdir().unwrap();
    let path = dir.path().join("keys.json");
    let kp = generate_keypair();
    save_keypair(&kp, &path).unwrap();
    let meta = std::fs::metadata(&path).unwrap();
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    let tmp = dir.path().join("keys.json.tmp");
    assert!(!tmp.exists(), "tmp sidecar must be renamed away, found {:?}", tmp);
}

#[test]
fn save_keypair_overwrites_existing_file() {
    // Overwrite semantics must survive the atomic-rename refactor:
    // a second save_keypair on the same path replaces the prior content.
    let dir = tempdir().unwrap();
    let path = dir.path().join("keys.json");
    let kp1 = generate_keypair();
    save_keypair(&kp1, &path).unwrap();
    let kp2 = generate_keypair();
    save_keypair(&kp2, &path).unwrap();
    let loaded = load_keypair(&path).unwrap();
    assert_eq!(
        loaded.verifying().to_bytes(),
        kp2.verifying().to_bytes(),
        "second save must replace first"
    );
}

#[test]
fn canonical_message_rejects_pipe_in_fields() {
    let err = canonical_message("dna|bad", "sha", "creator").expect_err("pipe in dna must fail");
    assert!(matches!(err, Error::MessageSeparator(_)));

    let err =
        canonical_message("dna", "sha|bad", "creator").expect_err("pipe in spec_sha must fail");
    assert!(matches!(err, Error::MessageSeparator(_)));

    let err =
        canonical_message("dna", "sha", "creator|bad").expect_err("pipe in creator_id must fail");
    assert!(matches!(err, Error::MessageSeparator(_)));

    canonical_message("dna", "sha", "creator").expect("clean fields must succeed");
}
