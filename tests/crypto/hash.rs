use mitrid_core::crypto::Hash;
use bezier_lib::crypto::hash::*;

#[test]
fn test_digest_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..DIGEST_SIZE-1 {
        buf.push(0);
    }

    let res = Digest::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Digest::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_digest_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..DIGEST_SIZE-1 {
        buf.push(0);
    }

    let res = Digest::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Digest::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_hash_sha512() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut hash = SHA512{};

    let res = hash.digest(&msg);
    assert!(res.is_ok());

    let digest = res.unwrap();

    let res = hash.verify(&msg, &digest);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = hash.check(&msg, &digest);
    assert!(res.is_ok());

    msg.push(0);

    let res = hash.verify(&msg, &digest);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = hash.check(&msg, &digest);
    assert!(res.is_err());
}