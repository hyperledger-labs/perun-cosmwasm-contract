//  Copyright 2021 PolyCrypt GmbH
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use crate::{crypto::OffIdentity, types::*};
use cosmwasm_std::{coin, Coin};
use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
};
use rand::{CryptoRng, Rng};
use std::convert::TryInto;

pub type KeyPair = (SigningKey, OffIdentity);

pub fn random_accounts<T: CryptoRng + Rng>(rng: &mut T, num: usize) -> Vec<KeyPair> {
    (1..num).map(|_| random_account(rng)).collect()
}

pub fn random_account<T: CryptoRng + Rng>(rng: &mut T) -> KeyPair {
    let sk = SigningKey::random(rng);
    let pk = VerifyingKey::from(&sk).to_encoded_point(true);
    (sk, OffIdentity(pk.as_bytes().try_into().unwrap()))
}

pub fn random_params<T: CryptoRng + Rng>(rng: &mut T) -> (Params, Vec<SigningKey>) {
    let num_parts: usize = rng.gen_range(1..10);
    let key_pairs = random_parts(rng, num_parts);

    (
        Params {
            nonce: random_nonce(rng),
            participants: key_pairs.iter().map(|p| p.1.clone()).collect(),
            dispute_duration: random_dispute_duration(rng),
        },
        key_pairs.iter().map(|p| p.0.clone()).collect(),
    )
}

pub fn random_state<T: CryptoRng + Rng>(rng: &mut T) -> State {
    let (_, state) = random_params_state(rng);
    state
}

pub fn random_params_state<T: CryptoRng + Rng>(rng: &mut T) -> ((Params, Vec<SigningKey>), State) {
    let (params, sks) = random_params(rng);
    (
        (params.clone(), sks),
        State {
            channel_id: params.channel_id().unwrap(),
            version: random_version(rng),
            balances: random_balances(rng, params.participants.len()),
            finalized: random_finalized(rng),
        },
    )
}

pub fn random_withdrawal<T: CryptoRng + Rng>(rng: &mut T) -> (Withdrawal, Vec<SigningKey>, usize) {
    let (params, sks) = random_params(rng);
    let index = rng.gen_range(0..params.participants.len());
    (
        Withdrawal {
            channel_id: params.channel_id().unwrap(),
            part: params.participants[index].clone(),
            receiver: cosmwasm_std::Addr::unchecked("ALICE"),
        },
        sks,
        index,
    )
}

pub fn random_part<T: CryptoRng + Rng>(rng: &mut T) -> KeyPair {
    random_account(rng)
}

pub fn random_parts<T: CryptoRng + Rng>(rng: &mut T, n: usize) -> Vec<KeyPair> {
    (0..n).map(|_| random_part(rng)).collect()
}

pub fn random_dispute_duration<T: CryptoRng + Rng>(rng: &mut T) -> Seconds {
    rng.gen_range(1..600)
}

pub fn random_balance<T: CryptoRng + Rng>(rng: &mut T) -> WrappedNativeBalance {
    let num_coins = rng.gen_range(0..9);
    (0..num_coins)
        .map(|i| coin(rng.next_u64().into(), format!("asset-#{}", i)))
        .collect::<Vec<Coin>>()
        .into()
}

pub fn random_balances<T: CryptoRng + Rng>(rng: &mut T, num_parts: usize) -> Vec<WrappedNativeBalance> {
    (1..num_parts).map(|_| random_balance(rng)).collect()
}

pub fn random_finalized<T: CryptoRng + Rng>(rng: &mut T) -> bool {
    rng.gen_range(0..1) == 1
}

pub fn random_version<T: CryptoRng + Rng>(rng: &mut T) -> Version {
    rng.next_u32().into()
}

pub fn random_nonce<T: CryptoRng + Rng>(rng: &mut T) -> Nonce {
    let mut ret = [0; 32];
    rng.fill_bytes(&mut ret);
    ret.into()
}
