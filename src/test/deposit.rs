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

use crate::{test::common::setup::*, types::NativeBalance};
use cosmwasm_std::coin;

#[test]
fn init() {
    do_init();
}

#[test]
fn deposit_some() {
    let (s, mut deps) = do_init();
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();

    let deposited = query_deposit(deps.as_mut(), s.fids[0].clone());
    assert_eq!(deposited, s.alloc[0]);
}

#[test]
fn deposit_twice() {
    let (s, mut deps) = do_init();
    let fid = &s.fids[0];

    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[1], BOB.into()).unwrap();

    let deposited = query_deposit(deps.as_mut(), fid.clone());
    assert_eq!(deposited, s.outcome);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn deposit_overflow() {
    let (s, mut deps) = do_init();

    let bals: NativeBalance = vec![coin(std::u128::MAX - 10, DENOMS[0])].into();
    // Normal
    do_deposit(deps.as_mut(), &s.fids[0], &bals, ALICE.into()).unwrap();
    // Overflow
    do_deposit(deps.as_mut(), &s.fids[0], &bals, ALICE.into()).expect_err("Should overflow");
}
