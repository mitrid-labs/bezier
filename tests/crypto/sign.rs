use mitrid_core::crypto::Sign;
use bezier_lib::crypto::sign::*;

#[test]
fn test_keyseed_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SEED_SIZE-1 {
        buf.push(0);
    }

    let res = Seed::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Seed::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_digest_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SEED_SIZE-1 {
        buf.push(0);
    }

    let res = Seed::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Seed::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_secret_key_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SECRETKEY_SIZE-1 {
        buf.push(0);
    }

    let res = SecretKey::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = SecretKey::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_secret_key_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SECRETKEY_SIZE-1 {
        buf.push(0);
    }

    let res = SecretKey::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = SecretKey::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_public_key_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..PUBLICKEY_SIZE-1 {
        buf.push(0);
    }

    let res = PublicKey::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = PublicKey::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_public_key_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..PUBLICKEY_SIZE-1 {
        buf.push(0);
    }

    let res = PublicKey::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = PublicKey::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_signature_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SIGNATURE_SIZE-1 {
        buf.push(0);
    }

    let res = Signature::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Signature::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_signature_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SIGNATURE_SIZE-1 {
        buf.push(0);
    }

    let res = Signature::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Signature::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_sign_ed25519() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut signer = Ed25519{};

    let res = signer.generate_keys(None);
    assert!(res.is_ok());

    let (pk, sk) = res.unwrap();

    let res = signer.sign(&msg, &sk);
    assert!(res.is_ok());

    let sig = res.unwrap();

    let res = signer.verify(&msg, &pk, &sig);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = signer.check(&msg, &pk, &sig);
    assert!(res.is_ok());

    msg.push(0);

    let res = signer.verify(&msg, &pk, &sig);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = signer.check(&msg, &pk, &sig);
    assert!(res.is_err());
}