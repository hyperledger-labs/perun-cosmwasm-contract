use k256::{
    ecdsa::{
        signature::{DigestSigner, Signature as _},
        SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
};
use perun_cosmwasm::{
    crypto::{hash, OffIdentity, Sig},
    types::State,
};
use rand::rngs::ThreadRng;
use serde::Serialize;
use std::convert::TryInto;

pub fn sign<T: Serialize>(obj: &T, sk: &SigningKey) -> Sig {
    let h = hash(obj).unwrap();
    let s: k256::ecdsa::Signature = sk.try_sign_digest(h).unwrap();
    Sig(s.as_bytes().into())
}

pub fn random_account(rng: &mut ThreadRng) -> (SigningKey, OffIdentity) {
    let sk = SigningKey::random(rng);
    let pk = VerifyingKey::from(&sk).to_encoded_point(true);
    (sk, OffIdentity(pk.as_bytes().try_into().unwrap()))
}

pub fn fully_sign(state: &State, keys: &[SigningKey]) -> Vec<Sig> {
    keys.iter().map(|key| sign(state, key)).collect()
}
