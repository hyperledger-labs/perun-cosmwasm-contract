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

use super::random::{random_account, random_nonce};
use crate::{
    contract::{execute, instantiate, query},
    crypto::Sig,
    error::ContractError,
    msg::*,
    types::*,
};
use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Coin, DepsMut, Env, OwnedDeps, Response,
};
use std::ops::Add;

type Deps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

use k256::ecdsa::SigningKey;

pub const DENOMS: [&str; 2] = ["COSM", "ATOM"];
pub const ALICE: &str = "alice";
pub const BOB: &str = "bob";

pub struct Setup {
    pub keys: Vec<SigningKey>,
    pub params: Params,
    pub cid: ChannelId,
    pub fids: Vec<FundingId>,
    pub final_state: State,
    pub nfinal_state: State,
    pub alloc: Vec<WrappedNativeBalance>,
    pub outcome: WrappedNativeBalance,
}

pub fn new_setup() -> Setup {
    let mut rng = rand::thread_rng();
    let (alice_off, bob_off) = (random_account(&mut rng), random_account(&mut rng));
    let params = Params {
        nonce: random_nonce(&mut rng),
        participants: vec![alice_off.1.clone(), bob_off.1.clone()],
        dispute_duration:  60u64.into(),
    };
    let cid = params.channel_id().unwrap();
    let alloc = vec![
        WrappedNativeBalance::from(vec![coin(2, DENOMS[0]), coin(20, DENOMS[1])]),
        WrappedNativeBalance::from(vec![coin(0, DENOMS[0]), coin(10, DENOMS[1])]),
    ];
    let outcome = alloc[0].clone().add(&alloc[1]);
    Setup {
        keys: vec![alice_off.0, bob_off.0],
        params,
        cid: cid.clone(),
        fids: vec![
            calc_funding_id(&cid, &alice_off.1).unwrap(),
            calc_funding_id(&cid, &bob_off.1).unwrap(),
        ],
        final_state: State {
            channel_id: cid.clone(),
            version: 123u64.into(),
            balances: alloc.iter().map(|bals| bals.0.clone()).collect(),
            finalized: true,
        },
        nfinal_state: State {
            channel_id: cid.clone(),
            version: 123u64.into(),
            balances: alloc.iter().map(|bals| bals.0.clone()).collect(),
            finalized: false,
        },
        alloc,
        outcome,
    }
}

pub fn do_init() -> (Setup, Deps) {
    let mut deps = mock_dependencies(&[]);

    // Instantiate
    let msg = InitMsg {};
    let info = mock_info("creator_key", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).expect("Init failed");
    (new_setup(), deps)
}

pub fn do_deposit(
    deps: DepsMut,
    fid: &FundingId,
    bals: &WrappedNativeBalance,
    who: String,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Deposit(fid.clone());
    let info = mock_info(who.as_ref(), Vec::<Coin>::from(bals.clone()).as_slice());
    execute(deps, mock_env(), info, msg)
}

pub fn do_conclude(
    deps: DepsMut,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Conclude(params.clone(), state.clone(), sigs.into());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_dispute(
    deps: DepsMut,
    params: &Params,
    state: &State,
    sigs: &Vec<Sig>,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Dispute(params.clone(), state.clone(), sigs.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_conclude_dispute(deps: DepsMut, params: &Params) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::ConcludeDispute(params.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn do_withdraw(
    deps: DepsMut,
    withdrawal: &Withdrawal,
    sig: &Sig,
) -> Result<Response, ContractError> {
    let msg = ExecuteMsg::Withdraw(withdrawal.clone(), sig.clone());
    let info = mock_info(ALICE, &[]);
    execute(deps, mock_env(), info, msg)
}

pub fn query_deposit(deps: DepsMut, fid: FundingId) -> WrappedNativeBalance {
    match query(deps.as_ref(), mock_env(), QueryMsg::Deposit(fid)) {
        Err(_) => WrappedNativeBalance::default(),
        Ok(deposit) => {
            let bals: cw0::NativeBalance = cosmwasm_std::from_binary(&deposit).unwrap();
            WrappedNativeBalance::from(bals.0)
        },
    }
}

pub fn advance_time(mut env: Env, by: Seconds) -> Env {
    env.block.time = env.block.time.plus_seconds(by.u64());
    env
}
