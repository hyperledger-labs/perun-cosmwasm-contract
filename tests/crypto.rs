mod common;
use common::crypto::{random_account, sign};
use perun_cosmwasm::crypto::verify;
use rand_core::{OsRng, RngCore};

#[test]
fn test_sig_verify() {
    let mut rng = rand::thread_rng();
    let (sk, pk) = random_account(&mut rng);
    let mut msg: [u8; 32] = Default::default();
    OsRng.fill_bytes(&mut msg);

    let sig = sign(&msg, &sk);
    assert!(verify(&msg, &pk, &sig).is_ok());
}
