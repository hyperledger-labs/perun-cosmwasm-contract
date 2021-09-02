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
    error::ContractError,
    mock::{
        crypto::{fully_sign, sign},
        random::*,
        setup::*,
    },
    types::*,
};

#[test]
fn withdraw_wrong_sig() {
    let (s, mut deps) = do_init();
    let withdrawal = Withdrawal {
        channel_id: s.cid,
        part: s.params.participants[0].clone(), // Alice wants to withdraw
        receiver: cosmwasm_std::Addr::unchecked(ALICE),
    };
    let sig = sign(&withdrawal, &s.keys[1]); // But Bob signed

    assert_eq!(
        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
        ContractError::WrongSignature {}
    );
}

#[test]
fn withdraw_unknown_channel() {
    let (s, mut deps) = do_init();
    let withdrawal = Withdrawal {
        channel_id: s.cid,
        part: s.params.participants[0].clone(),
        receiver: cosmwasm_std::Addr::unchecked(ALICE),
    };
    let sig = sign(&withdrawal, &s.keys[0]);

    assert_eq!(
        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
        ContractError::UnknownChannel {}
    );
}

#[test]
fn withdraw_not_concluded() {
    let (s, mut deps) = do_init();

    let sigs = fully_sign(&s.nfinal_state, &s.keys);
    do_dispute(deps.as_mut(), &s.params, &s.nfinal_state, &sigs).unwrap();

    let withdrawal = Withdrawal {
        channel_id: s.cid,
        part: s.params.participants[0].clone(),
        receiver: cosmwasm_std::Addr::unchecked(ALICE),
    };
    let sig = sign(&withdrawal, &s.keys[0]);

    assert_eq!(
        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
        ContractError::NotConcluded {}
    );
}

#[test]
fn withdraw_unknown_deposit() {
    let (s, mut deps) = do_init();
    let mut state = s.final_state.clone();
    state.balances = vec![Default::default(), Default::default()];

    let sigs = fully_sign(&state, &s.keys);
    do_conclude(deps.as_mut(), &s.params, &state, &sigs).unwrap();

    // Generate a new account 'Carl' that is not part of the channel.
    let mut rng = rand::thread_rng();
    let carl = random_account(&mut rng);
    let withdrawal = Withdrawal {
        channel_id: s.cid,
        part: carl.1, // Carl wants to withdraw
        receiver: cosmwasm_std::Addr::unchecked(ALICE),
    };
    let sig = sign(&withdrawal, &carl.0); // Carl signed

    assert_eq!(
        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
        ContractError::UnknownDeposit {}
    );
}

#[test]
fn withdraw_ok() {
    let (s, mut deps) = do_init();
    // Alice deposits
    do_deposit(deps.as_mut(), &s.fids[0], &s.alloc[0], ALICE.into()).unwrap();
    // Bob deposits
    do_deposit(deps.as_mut(), &s.fids[1], &s.alloc[1], BOB.into()).unwrap();

    // Assert both balances to alloc.
    let deposited = query_deposit(deps.as_mut(), s.fids[0].clone());
    assert_eq!(deposited, s.alloc[0]);
    let deposited = query_deposit(deps.as_mut(), s.fids[1].clone());
    assert_eq!(deposited, s.alloc[1]);

    // Update the balances by swapping them.
    let mut state = s.final_state.clone();
    state.balances = vec![state.balances[1].clone(), state.balances[0].clone()];
    // Conclude.
    let sigs = fully_sign(&s.final_state, &s.keys);
    do_conclude(deps.as_mut(), &s.params, &s.final_state, &sigs).unwrap();

    // Alice withdraws
    {
        let withdrawal = Withdrawal {
            channel_id: s.cid.clone(),
            part: s.params.participants[0].clone(),
            receiver: cosmwasm_std::Addr::unchecked(ALICE),
        };
        let sig = sign(&withdrawal, &s.keys[0]);

        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap();
        // Withdrawing twice errors.
        assert_eq!(
            do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
            ContractError::UnknownDeposit {}
        );
    }
    // Bob  withdraws
    {
        let withdrawal = Withdrawal {
            channel_id: s.cid,
            part: s.params.participants[1].clone(),
            receiver: cosmwasm_std::Addr::unchecked(BOB),
        };
        let sig = sign(&withdrawal, &s.keys[1]);

        do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap();
        // Withdrawing twice errors.
        assert_eq!(
            do_withdraw(deps.as_mut(), &withdrawal, &sig).unwrap_err(),
            ContractError::UnknownDeposit {}
        );
    }

    // Assert both balances to 0.
    let deposited = query_deposit(deps.as_mut(), s.fids[0].clone());
    assert_eq!(deposited, Default::default());
    let deposited = query_deposit(deps.as_mut(), s.fids[1].clone());
    assert_eq!(deposited, Default::default());
}
