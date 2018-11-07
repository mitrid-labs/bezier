use mitrid_core::crypto::Prove;
use bezier_lib::crypto::prove::*;

#[test]
fn test_prove_hashcash() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let bits = 3;
    let mut prover = HashCash::new(bits);

    let res = prover.prove(&msg);
    assert!(res.is_ok());

    let proof = res.unwrap();

    let res = prover.verify(&msg, &proof);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = prover.check(&msg, &proof);
    assert!(res.is_ok());

    msg.push(0);

    let res = prover.verify(&msg, &proof);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = prover.check(&msg, &proof);
    assert!(res.is_err());
}