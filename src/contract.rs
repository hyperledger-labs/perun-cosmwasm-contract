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

//! Core functionality for controlling the on-chain part of state channels.
use crate::{
    crypto::{OffIdentity, Sig},
    ensure,
    error::ContractError,
    msg::{ExecuteMsg, InitMsg, QueryMsg, DepositResponse, DisputeResponse},
    storage::{DEPOSITS, DISPUTES},
    types::*,
};
use cosmwasm_std::{
    entry_point, to_binary, BankMsg::Send, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    Storage, Timestamp,
};
use std::{ops::Add, result::Result};

/// Handles all [InitMsg] messages.
///
/// Can be used to initialize the contract, which is only done once.
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    Ok(Default::default())
}

/// Handles all [ExecuteMsg] messages.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit(funding_id) => deposit(deps.storage, info, funding_id),
        ExecuteMsg::Dispute(params, state, sigs) => {
            dispute(deps, env.block.time, &params, &state, &sigs)
        }
        ExecuteMsg::Conclude(params, state, sigs) => conclude(deps, &params, &state, &sigs),
        ExecuteMsg::ConcludeDispute(params) => {
            conclude_dispute(deps.storage, env.block.time, &params)
        }
        ExecuteMsg::Withdraw(withdrawal, sig) => withdraw(deps, &withdrawal, &sig),
    }
}

/// Handles all [QueryMsg] messages.
///
/// Can be used to query the contract state.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Deposit(fid) => query_deposit(deps, fid),
        QueryMsg::Dispute(cid) => query_dispute(deps, cid),
    }
}

/// See [crate::msg::QueryMsg::Deposit].
fn query_deposit(deps: Deps, fid: FundingId) -> Result<Binary, ContractError> {
    match DEPOSITS.may_load(deps.storage, fid)? {
        Some(deposit) => {
            let out = to_binary(&DepositResponse(deposit.0.0))?;
            Ok(out)
        }
        None => Err(ContractError::UnknownChannel {}),
    }
}

/// See [crate::msg::QueryMsg::Dispute].
fn query_dispute(deps: Deps, cid: ChannelId) -> Result<Binary, ContractError> {
    match DISPUTES.may_load(deps.storage, cid)? {
        Some(dispute) => {
            let out = to_binary(&DisputeResponse(dispute))?;
            Ok(out)
        }
        None => Err(ContractError::UnknownDispute {}),
    }
}

/// See [crate::msg::ExecuteMsg::Deposit].
fn deposit(
    storage: &mut dyn Storage,
    info: MessageInfo,
    funding_id: FundingId,
) -> Result<Response, ContractError> {
    DEPOSITS.update(storage, funding_id, |holding| -> Result<_, ContractError> {
        Ok(holding.unwrap_or_default().add(&info.funds.into()))
    })?;
    Ok(Default::default())
}

/// See [crate::msg::ExecuteMsg::Dispute].
fn dispute(
    deps: DepsMut,
    now: Timestamp,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    ensure!(!state.finalized, ContractError::StateFinal {});
    state.verify_fully_signed(params, sigs, deps.api)?;
    let channel_id = state.channel_id.clone();

    match DISPUTES.may_load(deps.storage, channel_id.clone())? {
        None => {
            let timeout = now.plus_seconds(params.dispute_duration.u64());
            let dispute = Dispute {
                state: state.clone(),
                timeout: timeout,
                concluded: false,
            };
            DISPUTES.save(deps.storage, channel_id, &dispute)?;
            Ok(Default::default())
        }
        Some(Dispute {
            state: old_state,
            timeout,
            concluded,
        }) => {
            ensure!(!concluded, ContractError::AlreadyConcluded {});
            ensure!(
                state.version > old_state.version,
                ContractError::DisputeVersionTooLow {}
            );
            ensure!(now < timeout, ContractError::DisputeTimedOut {});

            let dispute = Dispute {
                state: state.clone(),
                timeout,
                concluded: false,
            };
            DISPUTES.save(deps.storage, channel_id, &dispute)?;
            Ok(Default::default())
        }
    }
}

/// See [crate::msg::ExecuteMsg::Conclude].
fn conclude(
    deps: DepsMut,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    ensure!(state.finalized, ContractError::StateNotFinal {});
    state.verify_fully_signed(params, sigs, deps.api)?;
    let channel_id = &state.channel_id;

    match DISPUTES.may_load(deps.storage, channel_id.clone())? {
        Some(dispute) => {
            if dispute.concluded {
                Err(ContractError::AlreadyConcluded {})
            } else {
                Err(ContractError::DisputeActive {})
            }
        },
        None => {
            push_outcome(deps.storage, channel_id, &params.participants, &state.balances)?;
            let reg = Dispute {
                state: state.clone(),
                timeout: Timestamp::from_seconds(0),
                concluded: true,
            };
            DISPUTES.save(deps.storage, channel_id.clone(), &reg)?; // TODO maybe use error into?
            Ok(Default::default())
        }
    }
}

/// See [crate::msg::ExecuteMsg::ConcludeDispute].
fn conclude_dispute(
    storage: &mut dyn Storage,
    now: Timestamp,
    params: &Params,
) -> Result<Response, ContractError> {
    let channel_id = params.channel_id()?;
    match DISPUTES.may_load(storage, channel_id.clone())? {
        None => Err(ContractError::UnknownDispute {}),
        Some(Dispute { state, timeout, concluded }) => {
            if concluded {
                Err(ContractError::AlreadyConcluded {})
            } else {
            // Check that the timeout has elapsed for non-final states.
            ensure!(
                state.finalized || now >= timeout,
                ContractError::ConcludedTooEarly {}
            );
            // Write the outcome of the channel.
            push_outcome(storage, &channel_id, &params.participants, &state.balances)?;
            // End the dispute.
            DISPUTES.save(storage, channel_id, &Dispute { state, timeout, concluded: true })?;
            Ok(Default::default())
            }
        },
    }
}

/// See [crate::msg::ExecuteMsg::Withdraw].
fn withdraw(
    deps: DepsMut,
    withdrawal: &Withdrawal,
    withdrawal_sig: &Sig,
) -> Result<Response, ContractError> {
    withdrawal.verify(withdrawal_sig, deps.api)?;
    // Load the dispute.
    match DISPUTES.may_load(deps.storage, withdrawal.channel_id.clone())? {
        None => Err(ContractError::UnknownChannel {}),
        Some(Dispute { state: _state, timeout: _timeout, concluded }) => {
            if !concluded {
                Err(ContractError::NotConcluded {})
            } else {
                let funding_id = withdrawal.funding_id()?;
                // Load the deposit.
                let deposit = DEPOSITS.may_load(deps.storage, funding_id.clone())?;
                ensure!(deposit.is_some(), ContractError::UnknownDeposit {});
                let deposit = deposit.unwrap();
                // Remove the deposit.
                DEPOSITS.remove(deps.storage, funding_id);
                // Transfer the outcome to the user.
                let transfer = Send {
                    to_address: withdrawal.receiver.clone().into_string(),
                    amount: deposit.into(),
                };
                Ok(Response::new().add_message(transfer))
            }
        },
    }
}

/// Pushes the outcome of a channel back into the `DEPOSITS` map.
///
/// Checks that the sum of outcome is smaller or equal to the sum
/// of deposits in the channel.
/// This ensures that the participants cannot withdraw more than they
/// initially deposited.
fn push_outcome(
    storage: &mut dyn Storage,
    channel_id: &ChannelId,
    parts: &[OffIdentity],
    outcome: &[cw0::NativeBalance],
) -> Result<Response, ContractError> {
    ensure!(
        parts.len() == outcome.len(),
        ContractError::InvalidOutcome {}
    );
    ensure!(!parts.is_empty(), ContractError::InvalidOutcome {});

    // Save all Funding IDs for later.
    let mut fids = Vec::<FundingId>::default();
    // Calculate the sums of the outcome and deposit.
    let mut sum_outcome = WrappedBalance::default();
    let mut sum_deposit = WrappedBalance::default();

    for (i, part) in parts.iter().enumerate() {
        let fid = calc_funding_id(channel_id, part)?;
        fids.push(fid.clone());
        let deposit = DEPOSITS.load(storage, fid).unwrap_or_default();

        let outcome_ = WrappedBalance::from(outcome[i].0.clone());
        sum_outcome = sum_outcome.add(&outcome_);
        sum_deposit = sum_deposit.add(&deposit);
    }
    // Ensure that the participants of a channel can never withdraw more
    // than their initially deposited.
    ensure!(
        sum_deposit.greater_or_equal(&sum_outcome),
        ContractError::InsufficientDeposits {}
    );
    // Over-funding a channel will result in lost funds.
    // Now we split up all funds according to the outcome.
    for (i, fid) in fids.iter().enumerate() {
        let outcome_ = WrappedBalance::from(outcome[i].0.clone());
        DEPOSITS.save(storage, fid.to_vec(), &outcome_)?;
    }
    Ok(Default::default())
}
