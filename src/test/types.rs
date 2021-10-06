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

use crate::{
    test::common::{
        crypto::{fully_sign, sign},
        random::{random_account, random_params_state, random_state, random_withdrawal},
    },
    types::*,
};
use cosmwasm_std::{coin, coins, Coin, Uint128};

#[test]
fn native_balance_cmp() {
    let a = NativeBalance::from(coins(1u128, "PRN"));
    let b = NativeBalance::from(coins(2u128, "PRN"));

    assert!(a.greater_or_equal(&a));
    assert!(!a.greater_or_equal(&b));
    assert!(b.greater_or_equal(&a));
}

#[test]
fn native_balance_cmp_multiple() {
    let a = NativeBalance::from(vec![coin(2u128, "PRN"), coin(2u128, "BTC")]);
    let b = NativeBalance::from(vec![coin(1u128, "PRN"), coin(3u128, "BTC")]);
    let c = NativeBalance::from(vec![coin(3u128, "PRN"), coin(3u128, "BTC")]);
    let d = NativeBalance::from(coins(3u128, "PRN"));

    assert!(a.greater_or_equal(&a));
    assert!(!a.greater_or_equal(&b));
    assert!(!b.greater_or_equal(&a));
    assert!(!a.greater_or_equal(&c));
    assert!(c.greater_or_equal(&a));
    assert!(!a.greater_or_equal(&d));
    assert!(!d.greater_or_equal(&a));
}

#[test]
fn native_balance_normalize() {
    let a = NativeBalance::from(vec![coin(2u128, "PRN"), coin(2u128, "PRN")]);
    let v: Vec<Coin> = a.into();

    assert_eq!(v.len(), 1);
    assert_eq!(v[0].amount, Uint128::new(4));
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn native_balance_overflow() {
    let a = NativeBalance::from(coins(u128::MAX - 1, "PRN"));
    let _ = a.clone() + &a;
}

#[test]
fn state_sig_verify() {
    let mut rng = rand::thread_rng();
    let (sk, pk) = random_account(&mut rng);
    let state = random_state(&mut rng);

    let sig = sign(&state, &sk);
    assert!(state.verify(&sig, &pk).is_ok());
}

#[test]
fn state_sig_verify_full() {
    let mut rng = rand::thread_rng();
    let ((params, sks), state) = random_params_state(&mut rng);

    let sigs = fully_sign(&state, &sks);
    assert!(state.verify_fully_signed(&params, &sigs).is_ok());
}

#[test]
fn withdrawal_sig_verify() {
    let mut rng = rand::thread_rng();
    let (withdrawal, sks, index) = random_withdrawal(&mut rng);

    let sig = sign(&withdrawal, &sks[index]);
    assert!(withdrawal.verify(&sig).is_ok());
}
