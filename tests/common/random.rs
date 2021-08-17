use cosmwasm_std::{coin, Coin};
use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
};
use perun_cosmwasm::{crypto::OffIdentity, types::*};
use rand::{CryptoRng, Rng};
use std::convert::TryInto;

pub fn random_state<T: CryptoRng + Rng>(rng: &mut T) -> (Params, State) {
    let num_parts: usize = rng.gen_range(1..10);
    let params = random_params(rng, num_parts);

    (
        params.clone(),
        State {
            channel_id: params.channel_id().unwrap(),
            version: random_version(rng),
            balances: random_balances(rng, num_parts),
            finalized: random_finalized(rng),
        },
    )
}

pub fn random_params<T: CryptoRng + Rng>(rng: &mut T, num_parts: usize) -> Params {
    Params {
        nonce: random_nonce(rng),
        participants: random_parts(rng, num_parts),
        dispute_duration: random_dispute_duration(rng),
    }
}

pub fn random_account<T: CryptoRng + Rng>(rng: &mut T) -> (SigningKey, OffIdentity) {
    let sk = SigningKey::random(rng);
    let pk = VerifyingKey::from(&sk).to_encoded_point(true);
    (sk, OffIdentity(pk.as_bytes().try_into().unwrap()))
}

pub fn random_part<T: CryptoRng + Rng>(rng: &mut T) -> OffIdentity {
    random_account(rng).1
}

pub fn random_parts<T: CryptoRng + Rng>(rng: &mut T, n: usize) -> Vec<OffIdentity> {
    (1..n).map(|_| random_part(rng)).collect()
}

pub fn random_dispute_duration<T: CryptoRng + Rng>(rng: &mut T) -> Seconds {
    rng.gen_range(1..600)
}

pub fn random_balance<T: CryptoRng + Rng>(rng: &mut T) -> NativeBalance {
    let num_coins = rng.gen_range(0..9);
    (0..num_coins)
        .map(|i| coin(rng.next_u64().into(), format!("asset-#{}", i)).into())
        .collect::<Vec<Coin>>()
        .into()
}

pub fn random_balances<T: CryptoRng + Rng>(rng: &mut T, num_parts: usize) -> Vec<NativeBalance> {
    (1..num_parts).map(|_| random_balance(rng)).collect()
}

pub fn random_finalized<T: CryptoRng + Rng>(rng: &mut T) -> bool {
    rng.gen_range(0..1) == 1
}

pub fn random_version<T: CryptoRng + Rng>(rng: &mut T) -> Version {
    rng.next_u32().into()
}

pub fn random_nonce<T: CryptoRng + Rng>(rng: &mut T) -> Nonce {
    let mut ret: Nonce = [0; NONCE_LEN];
    rng.fill_bytes(&mut ret);
    ret
}
