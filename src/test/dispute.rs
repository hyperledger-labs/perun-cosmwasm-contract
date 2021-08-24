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
    crypto::Sig,
    error::ContractError,
    msg::ExecuteMsg,
    test::common::{
        crypto::{fully_sign, sign},
        setup::*,
    },
    types::*,
};
use cosmwasm_std::testing::{mock_env, mock_info};

/// Dispute works with a non-final state.
#[test]
fn dispute_ok() {
    let (s, mut deps) = do_init();

    let sigs = fully_sign(&s.nfinal_state, &s.keys);
    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();
}

/// Dispute fails with a final state.
#[test]
fn dispute_final() {
    let (s, mut deps) = do_init();

    let sigs = fully_sign(&s.final_state, &s.keys);
    assert_eq!(
        do_dispute(deps.as_mut(), &s.params, &s.final_state, &sigs).unwrap_err(),
        ContractError::StateFinal {}
    );
}

/// Dispute panics when the [Params::dispute_duration] overflows.
#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn dispute_dispute_duration_overflow() {
    let (mut s, mut deps) = do_init();
    s.params.dispute_duration = Seconds::MAX - 1;
    s.nfinal_state.channel_id = s.params.channel_id().unwrap();
    let sigs = fully_sign(&s.nfinal_state, &s.keys);

    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();
}

/// Dispute fails with an incorrect number of signatures.
#[test]
fn dispute_wrong_sig_nums() {
    let (s, mut deps) = do_init();
    let state = s.nfinal_state;

    let sigs = [
        fully_sign(&state, &s.keys[0..1]),                      // 1 sig
        fully_sign(&state, &[s.keys.clone(), s.keys].concat()), // 4 sigs
    ];

    for bad_sigs in sigs {
        assert_eq!(
            do_dispute(deps.as_mut(), &s.params, &state, &bad_sigs).unwrap_err(),
            ContractError::WrongSignatureNum {}
        );
    }
}

// Dispute fails with an invalid number of signatures.
#[test]
fn dispute_invalid_sig_nums() {
    let (s, mut deps) = do_init();

    assert_eq!(
        do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &vec![]).unwrap_err(),
        ContractError::InvalidSignatureNum {}
    );
}

/// Dispute fails when any of the signatures is incorrect.
#[test]
fn dispute_wrong_sigs() {
    let (s, mut deps) = do_init();
    let good_state = s.nfinal_state;
    let mut bad_state = good_state.clone();
    bad_state.version += 1;

    let sigs = [
        vec![sign(&good_state, &s.keys[0]), sign(&bad_state, &s.keys[1])],
        vec![sign(&bad_state, &s.keys[0]), sign(&good_state, &s.keys[1])],
        vec![sign(&bad_state, &s.keys[0]), sign(&bad_state, &s.keys[1])],
    ];
    for bad_sigs in sigs {
        assert_eq!(
            do_dispute(deps.as_mut(), &s.params, &good_state, &bad_sigs).unwrap_err(),
            ContractError::WrongSignature {}
        );
    }
}

/// Dispute fails then any of the signatures is malformed.
#[test]
fn dispute_invalid_sigs() {
    let (s, mut deps) = do_init();
    let good_state = s.nfinal_state;

    let good_sigs = fully_sign(&good_state, &s.keys);
    // Create two bad signatures.
    let bad_sigs = [
        Sig("Invalid".into()),
        Sig([good_sigs[0].0.as_slice(), b"Invalid"].concat()),
    ];

    let sigs = [
        vec![good_sigs[0].clone(), bad_sigs[1].clone()],
        vec![bad_sigs[0].clone(), good_sigs[1].clone()],
        bad_sigs.into(),
    ];

    for bad_sigs in sigs {
        assert_eq!(
            do_dispute(deps.as_mut(), &s.params, &good_state, &bad_sigs).unwrap_err(),
            ContractError::InvalidSignature {}
        );
    }
}

/// Dispute fails when the channel_id of the params does not match the one
/// from the state.
#[test]
fn dispute_wrong_channel_id() {
    let (s, mut deps) = do_init();
    let sigs = fully_sign(&s.nfinal_state, &s.keys);

    // Modify the nonce
    let mut params = s.params.clone();
    params.nonce = Default::default();
    assert_eq!(
        do_dispute(deps.as_mut(), &params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::WrongChannelId {}
    );

    // Modify the parts
    let mut params = s.params.clone();
    params.participants = vec![];
    assert_eq!(
        do_dispute(deps.as_mut(), &params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::WrongChannelId {}
    );

    // Modify the dispute_duration
    let mut params = s.params.clone();
    params.dispute_duration = Default::default();
    assert_eq!(
        do_dispute(deps.as_mut(), &params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::WrongChannelId {}
    );

    // Modify the channel_id from the state
    let mut state = s.nfinal_state.clone();
    state.channel_id = Default::default();
    assert_eq!(
        do_dispute(deps.as_mut(), &s.params, &state, &sigs).unwrap_err(),
        ContractError::WrongChannelId {}
    );
}

#[test]
fn dispute_same_version() {
    let (s, mut deps) = do_init();
    let sigs = fully_sign(&s.nfinal_state, &s.keys);

    // First time it works.
    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();
    // Second time with the same version it fails.
    assert_eq!(
        do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap_err(),
        ContractError::DisputeVersionTooLow {}
    );
}

/// Overwriting a dispute with a newer state version is possible.
#[test]
fn dispute_higher_version() {
    let (s, mut deps) = do_init();
    let mut state = s.nfinal_state.clone();
    let sigs = fully_sign(&state, &s.keys);

    do_dispute(deps.as_mut(), &s.params, &state, &sigs).unwrap();

    for _ in 1..10 {
        state.version += 1;
        let sigs = fully_sign(&state, &s.keys);
        do_dispute(deps.as_mut(), &s.params, &state, &sigs).unwrap();
    }
}

/// Overwriting a dispute with a newer state version after the dispute timed
/// out fails.
#[test]
fn dispute_timeout() {
    let (s, mut deps) = do_init();
    let mut state = s.nfinal_state.clone();
    let sigs = fully_sign(&state, &s.keys);

    // Set the version to `version`.
    do_dispute(deps.as_mut(), &s.params, &state, &sigs).unwrap();

    state.version += 1;
    let sigs = fully_sign(&state, &s.keys);

    // Try to dispute again with `version + 1` after the timeout.
    let msg = ExecuteMsg::Dispute(s.params.clone(), state, sigs);
    let info = mock_info(ALICE, &[]);
    let env = advance_time(mock_env(), s.params.dispute_duration + 1);
    assert_eq!(
        execute(deps.as_mut(), env, info, msg).unwrap_err(),
        ContractError::DisputeTimedOut {}
    );
}
