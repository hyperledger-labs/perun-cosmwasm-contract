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
    contract::execute,
    crypto::*,
    error::ContractError,
    msg::*,
    test::common::{
        crypto::{fully_sign, sign},
        setup::*,
    },
};
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Uint64,
};

#[test]
fn conclude_f_final() {
    let (s, mut deps) = do_init();

    // First deposit.
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    do_deposit(deps.as_mut(), &s.fids[1], &s.alloc[1], BOB.into()).unwrap();
    // Then conclude with final state.
    let sigs = fully_sign(&s.final_state, &s.keys);
    do_conclude(deps.as_mut(), &s.params, &s.final_state, &sigs).unwrap();
}

#[test]
fn conclude_f_non_final() {
    let (s, mut deps) = do_init();

    // First deposit.
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    // Then conclude with non-final state.
    let sigs = fully_sign(&s.nfinal_state, &s.keys);
    assert_eq!(
        do_conclude(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::StateNotFinal {}
    );
}

#[test]
fn conclude_f_wrong_sig_nums() {
    let (s, mut deps) = do_init();
    let state = s.final_state;

    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();

    let sigs = [
        fully_sign(&state, &s.keys[0..1]),                      // 1 sig
        fully_sign(&state, &[s.keys.clone(), s.keys].concat()), // 4 sigs
    ];

    for bad_sigs in sigs {
        assert_eq!(
            do_conclude(deps.as_mut(), &s.params, &state, &bad_sigs).unwrap_err(),
            ContractError::WrongSignatureNum {}
        );
    }
}

#[test]
fn conclude_f_invalid_sig_nums() {
    let (s, mut deps) = do_init();
    assert_eq!(
        do_conclude(deps.as_mut(), &s.params, &s.final_state, &[]).unwrap_err(),
        ContractError::InvalidSignatureNum {}
    );
}

#[test]
fn conclude_f_wrong_sigs() {
    let (s, mut deps) = do_init();
    let good_state = s.final_state;
    let bad_state = s.nfinal_state;

    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();

    let sigs = [
        vec![sign(&good_state, &s.keys[0]), sign(&bad_state, &s.keys[1])],
        vec![sign(&bad_state, &s.keys[0]), sign(&good_state, &s.keys[1])],
        vec![sign(&bad_state, &s.keys[0]), sign(&bad_state, &s.keys[1])],
    ];
    for bad_sigs in sigs {
        assert_eq!(
            do_conclude(deps.as_mut(), &s.params, &good_state, &bad_sigs).unwrap_err(),
            ContractError::WrongSignature {}
        );
    }
}

#[test]
fn conclude_f_invalid_sigs() {
    let (s, mut deps) = do_init();
    let good_state = s.final_state;

    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();

    let good_sigs = fully_sign(&good_state, &s.keys);
    // Create two bad signatures.
    let bad_sigs = [
        Sig("Invalid".as_bytes().into()),
        Sig([good_sigs[0].0.as_slice(), b"Invalid"]
            .concat()
            .as_slice()
            .into()),
    ];

    let sigs = [
        vec![good_sigs[0].clone(), bad_sigs[1].clone()],
        vec![bad_sigs[0].clone(), good_sigs[1].clone()],
        bad_sigs.into(),
    ];

    for bad_sigs in sigs {
        assert_eq!(
            do_conclude(deps.as_mut(), &s.params, &good_state, &bad_sigs).unwrap_err(),
            ContractError::InvalidSignature("Invalid signature format".to_string())
        );
    }
}

#[test]
fn conclude_f_wrong_params() {
    let (s, mut deps) = do_init();
    let mut wrong_params = s.params.clone();
    // Change the participants by reversing them.
    wrong_params.participants.reverse();

    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    let msg = ExecuteMsg::Conclude(
        wrong_params,
        s.final_state,
        vec![], // Use empty sigs since they are not checked.
    );
    let info = mock_info(ALICE, &[]);
    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg).unwrap_err(),
        ContractError::WrongChannelId {}
    );
}

#[test]
fn conclude_d_too_early() {
    let (s, mut deps) = do_init();

    // Omit the `deposit` since disputing an unfunded channel is possible.
    let sigs = fully_sign(&s.nfinal_state, &s.keys);
    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();

    assert_eq!(
        do_conclude(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::ConcludedTooEarly {}
    );
}

#[test]
fn conclude_d_after_timeout() {
    let (s, mut deps) = do_init();
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    do_deposit(deps.as_mut(), &s.fids[1], &s.alloc[1], BOB.into()).unwrap();

    // Omit the `deposit` since disputing an unfunded channel is possible.
    let sigs = fully_sign(&s.nfinal_state, &s.keys);
    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();

    // Advance time after the timeout and try call to `ConcludeDispute`.
    let env = advance_time(mock_env(), s.params.dispute_duration + Uint64::from(1u64));
    let msg = ExecuteMsg::Conclude(s.params, s.nfinal_state, sigs);
    let info = mock_info(ALICE, &[]);
    execute(deps.as_mut(), env, info, msg).unwrap();
}
